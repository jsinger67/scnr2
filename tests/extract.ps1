<#
.Synopsis
    Extracts x2, x3, n and e macros from a C file and converts them to test macros.
    The C file is taken from https://github.com/kkos/oniguruma/blob/master/test/test_utf8.c
    from the Oniguruma project.
    Copyright (c) 2002-2019 K.Kosako kkosako0@gmail.com All rights reserved.
#>
#[CmdLetBinding()]
param(
    [Parameter(Position = 0, Mandatory = $true, ValueFromPipeline = $true)]
    [ValidateScript({ Test-Path $_ -PathType Leaf })]
    [string] $Path
)

# These are the macros we are looking for:

# Pattern and input string
# x2("<pattern>", "<input>", <match_start>, <match_end>);
#define x2(p,s,f,t)    xx2(p,s,f,t, __LINE__)

# Pattern and input string with match group
# Since we are not extracting the match group we evaluate this macro as it were a x2 macro.
# x3("<pattern>", "<input>", <match_start>, <match_end>, <match_group>);
#define x3(p,s,f,t,m)  xx3(p,s,f,t,m, __LINE__)

# Empty string match
# n("<pattern>", "<input>");
#define n(p,s)          xn(p,s,   __LINE__)

# Match with expected error
# e("<pattern>", "<input>", <error_number>);
#define e(p,s,en)       xe(p,s,en, __LINE__)

function Write-Macro {
    param(
        [string] $Pattern,
        [string] $InputString,
        [string] $ExpectedMatch,
        [string] $Line,
        [int] $Count
    )
    $Line = $Line.Trim()
    Write-Output "// -------------------------------------------------------------------------"
    Write-Output "// $Line"
    Write-Output "// td!(r`#`"$Pattern`"`#, `"$InputString`", $ExpectedMatch, $Count),"
    Write-Output "// scanner! { S$Count { mode M { token r#`"$Pattern`"# => 0; } } }"
    Write-Output "// #[test] fn test_match_$Count() {"
    Write-Output "//   use s$Count::S$Count as S;"
    Write-Output "//   let scanner = S::new();"
    Write-Output "//   let matches = scanner.find_matches(`"$InputString`", 0).collect::<Vec<_>>();"
    Write-Output "//   const EXPECTED_MATCHES: &[(&str, usize, usize)] =  $ExpectedMatch;"
    Write-Output "//   assert_eq!(matches.len(), EXPECTED_MATCHES.len(), `"${Count}: Unexpected match count`");"
    if ($ExpectedMatch -ne "&[]") {
        Write-Output "//   for (i, ma) in EXPECTED_MATCHES.iter().enumerate() {"
        Write-Output "//       assert_eq!(matches[i].span.start, ma.1, `"${Count}: Match start does not match`");"
        Write-Output "//       assert_eq!(matches[i].span.end, ma.2, `"${Count}: Match end does not match`");"
        Write-Output "//       assert_eq!(&`"$InputString`"[ma.1..ma.2]`, ma.0, `"${Count}: Matched substring does not match expected`");"
        Write-Output "//   }"
    }
    Write-Output "//}"
    Write-Output ""
}

function Write-Error {
    param(
        [string] $Pattern,
        [string] $InputString,
        [string] $ErrorNumber,
        [string] $Line,
        [int] $Count
    )
    $Line = $Line.Trim()
    Write-Output "// -------------------------------------------------------------------------"
    Write-Output "// $Line"
    Write-Output "// tr!(r`#`"$Pattern`"`#, `"$InputString`", `"$ErrorNumber`", $Count),"
    Write-Output "// scanner! { S$Count { mode M { token r#`"$Pattern`"# => 0; } } }"
    Write-Output "// #[test] fn test_error_$Count() {"
    Write-Output "// }"
    Write-Output ""
}

Write-Output "/// This file contains a hopefully increasing number of match tests to verify the correctness of the"
Write-Output "/// scanner."
Write-Output "///"
Write-Output "/// Some tests are based on the https://github.com/kkos/oniguruma/blob/master/test/test_utf8.c file"
Write-Output "/// from the Oniguruma project."
Write-Output "/// Copyright (c) 2002-2019 K.Kosako kkosako0@gmail.com All rights reserved."
Write-Output "use scnr2::scanner;"
Write-Output ""

Get-Content $Path |
Where-Object { $_ -match "^\s*(x[23]|[ne])\("
} |
ForEach-Object -Begin {
    [Diagnostics.CodeAnalysis.SuppressMessageAttribute('UseDeclaredVarsMoreThanAssignments', '',
        Justification = 'Is actually used in the Process block')]
    $Count = 0
} -Process {
    $line = $_
    # x2("<pattern>", "<input>", <match_start>, <match_end>);
    # x3("<pattern>", "<input>", <match_start>, <match_end>, <match_group>);
    $matched = $_ -match 'x[23]\("(?<pattern>.*)",\s*"(?<input_string>.*)",\s*(?<span_start>\d+),\s*(?<span_end>\d+)\s*(,\s*(?<match_group>\d+)\s*)?\);'
    if ($matched) {
        # Write-Host "Matched: $_"
        $pattern = $matches['pattern']
        if ($pattern -eq $null) {
            $pattern = ""
        }
        $pattern = $pattern -replace '\\\\', '\'
        $input_string = $matches['input_string']
        if ($input_string -eq $null) {
            $input_string = ""
        }
        $span_start = [int]$matches['span_start']
        $span_end = [int]$matches['span_end']
        try {
            $matched_substring = $input_string.Substring($span_start, $span_end - $span_start)
            $matched_substring = $matched_substring -replace '\\', '\\'
            $expected_match = "(`"$matched_substring`", $span_start, $span_end)"
            if ($expected_match -eq '("", 0, 0)') {
                # ("", 0, 0) is the value for no match
                $expected_match = ""
            }
            Write-Macro -Pattern $pattern -InputString $input_string -ExpectedMatch "&[$expected_match]" -Count $Count -Line $line
        }
        catch {
            # Error handling: Output the original line commented out
            $line = $line.Trim()
            Write-Output "// Exception: $_ $line // $Count"
        }
    } else {
        $matched = $_ -match '(?<macro>[en])\("(?<pattern>.*)",\s*"(?<input_string>.*)"\s*(,\s*(?<error_number>.+?)\s*)?\);'
        if ($matched) {
            $macro = $matches['macro']
            $pattern = $matches['pattern']
            if ($pattern -eq $null) {
                $pattern = ""
            }
            $pattern = $pattern -replace '\\\\', '\'
            $input_string = $matches['input_string']
            if ($input_string -eq $null) {
                $input_string = ""
            }
            if ($macro -eq 'n') {
                # n("<pattern>", "<input>");
                Write-Macro -Pattern $pattern -InputString $input_string -ExpectedMatch "&[]" -Count $Count -Line $line
            } elseif ($macro -eq 'e') {
                # e("<pattern>", "<input>", <error_number>);
                $error_number = $matches['error_number']
                if ($error_number -eq $null) {
                    $error_number = ""
                }
                Write-Error -Pattern $pattern -InputString $input_string -ErrorNumber $error_number -Count $Count -Line $line
            }
        } else {
            # If the line does not match any of the expected patterns, output it as is
            $line = $_.Trim()
            Write-Output "// $line // $Count"
        }
    }
    $Count += 1
} -End {
    Write-Host "Converted $Count macros."
}