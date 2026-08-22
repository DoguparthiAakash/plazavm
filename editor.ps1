$file = $args[0]
(Get-Content $file) -replace 'pick 12ac72c', 'edit 12ac72c' | Set-Content $file
