Get-ChildItem -Directory crates\* | ForEach-Object {
    $name = $_.Name
    $hasR = Test-Path (Join-Path $_.FullName 'README.md')
    $hasC = Test-Path (Join-Path $_.FullName 'Cargo.toml')
    $hasL = Test-Path (Join-Path $_.FullName 'src\lib.rs')
    $r = if ($hasR) { 'Y' } else { 'N' }
    $c = if ($hasC) { 'Y' } else { 'N' }
    $l = if ($hasL) { 'Y' } else { 'N' }
    Write-Output ("{0} | Cargo={1} | README={2} | lib.rs={3}" -f $name, $c, $r, $l)
}
