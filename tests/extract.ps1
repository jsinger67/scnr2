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
# Since this wil not compile we will not extract it.
# e("<pattern>", "<input>", <match_start>, <match_end>);
#define e(p,s,en)       xe(p,s,en, __LINE__)


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
    $matched = $_ -match 'x[23]\("(?<pattern>.*)",\s*"(?<input_string>.*)",\s*(?<span_start>\d+),\s*(?<span_end>\d+)\s*(,\s*(?<span_end>\d+)\s*)?\);'
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
            $expected_match = "(`"$matched_substring`", $span_start, $span_end)"
            if ($expected_match -eq '("", 0, 0)') {
                # ("", 0, 0) is the value for no match
                $expected_match = ""
            }
            # Output the converted td! macro commented out, it has to be manually revised and
            # uncommented to be used
            Write-Output "// td!(r`#`"$pattern`"`#, `"$input_string`", &[$expected_match], $Count);"
        }
        catch {
            # Error handling: Output the original line commented out
            $line = $line.Trim()
            Write-Output "// $line // $Count"
        }
        $Count += 1
    } else {
        $matched = $_ -match 'n\("(?<pattern>.*)",\s*"(?<input_string>.*)"\);'
        Write-Output "// td!(r`#`"$pattern`"`#, `"$input_string`", $Count);"
    }
} -End {
    Write-Output "Converted $Count macros."
}