Get-ChildItem -Path 'E:\plazavm\staging' -Directory | ForEach-Object {
    $name = $_.Name
    $srcDir = Join-Path $_.FullName 'src'
    if (Test-Path $srcDir) {
        $files = Get-ChildItem $srcDir -Filter '*.rs' -Recurse
        $totalLines = 0
        foreach ($f in $files) {
            $totalLines += (Get-Content $f.FullName | Measure-Object -Line).Lines
        }
        Write-Host "$name : $($files.Count) files, $totalLines lines"
    } else {
        Write-Host "$name : NO src/"
    }
}
