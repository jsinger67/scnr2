# Path to your test file
$testFile = ".\match_test.rs"
# Read all lines
$lines = Get-Content $testFile
$changed = $false

for ($i = 1; $i -lt $lines.Count; $i++) {
    $line = $lines[$i]
    $prevLine = $lines[$i - 1]

    # Only process commented scanner! macros with td! above
    if ($line -match '^\s*//\s*scanner!' -and $prevLine -match 'td!') {
        # Uncomment the scanner! macro
        $lines[$i] = $line -replace '^\s*//\s*', ''
        Set-Content $testFile $lines

        # Run cargo check
        $output = cargo test test_match --color never --message-format human *>&1
        if ($LASTEXITCODE -ne 0) {
            # Re-comment if compilation fails
            $error_text = $output | ForEach-Object { $_.ToString() } | Where-Object { $_.Contains("= help") }
            $matched_regex_error = $error_text -match 'kind: (?<kind>\w*)'
            if ($matched_regex_error) {
                $kind = $matches['kind']
                $lines[$i - 1] = $lines[$i - 1] -replace 'td!', 'tr!'
                $lines[$i - 1] += " $kind"
                Write-Host "Re-commented scanner! macro due to compilation error: ${i}: $kind"
            } else {
                $matched_scnr2_error = $error_text -match 'UnsupportedFeatureError\("(?<error>.*)"\)'
                if ($matched_scnr2_error) {
                    $error_text = $matches['error']
                    $lines[$i - 1] = $lines[$i - 1] -replace 'td!', 'tu!'
                    $lines[$i - 1] += " UnsupportedFeatureError(`"$error_text`")"
                    Write-Host "Re-commented scanner! macro due to unsupported feature error: ${i}: $error_text"
                } else {
                    Write-Host "Unexpected error during compilation: $output"
                }
            }
            $lines[$i] = "// " + $lines[$i]
            Set-Content $testFile $lines
        }
        $changed = $true
    }
}

if ($changed) {
    Write-Host "Done. Checked all scanner! macros after td! macros."
} else {
    Write-Host "No scanner! macros after td! macros found to check."
}