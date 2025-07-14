# Path to your test file
$testFile = ".\match_test.rs"
$lines = Get-Content $testFile
$changed = $false

function Get-TestName($line) {
    if ($line -match 'fn\s+(\w+)\s*\(') { return $matches[1] }
    return $null
}

for ($i = 0; $i -lt $lines.Count; $i++) {
    # Find uncommented scanner! macro
    if ($lines[$i] -match '^\s*scanner!\s*{') {
        # Look ahead for a commented test function
        $j = $i + 1
        if ($lines[$j] -notmatch '^//') {
            # Skip already commissioned tests and search for the next scanner! macro
            continue
        }
        while ($lines[$j] -match '^//') {
            # Uncomment all lines until the end of the test function, i.e., an empty line
            $lines[$j] = $lines[$j] -replace '^\s*// ', ''
            $j++
            if ($j -ge $lines.Count) { break }
        }
        # Save and get test name
        Set-Content $testFile $lines
        $testName = Get-TestName $lines[$i + 1]
        if ($null -eq $testName) { break }
        # Run the test
        cargo test --test match_test --color never --message-format human | Write-Host
        if ($LASTEXITCODE -ne 0) {
            # Re-comment all lines of the test function, $i + 1 until $j + 1
            for ($k = $i + 1; $k -lt $j; $k++) {
                $lines[$k] = "// " + $lines[$k]
            }
            Set-Content $testFile $lines
            Write-Host "Test $testName failed, commented out again." -ForegroundColor Red
            $changed = $true
        }
        else {
            Write-Host "Test $testName passed and remains uncommented." -ForegroundColor Green
        }
        $i = $j + 1  # Move to the next scanner! macro
    }
}

if ($changed) {
    Write-Host "Done. All tests after scanner! macros have been checked." -ForegroundColor Cyan
}
else {
    Write-Host "No commented tests after scanner! macros found to check." -ForegroundColor Yellow
}

