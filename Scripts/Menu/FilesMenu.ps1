param (
    [string]$ProjectRoot
)

$files = Get-ChildItem `
    -Path "$ProjectRoot/data" `
| Select-Object PSIsContainer,@{
    Name = 'Name'
    Expression = { if ($_.PSIsContainer -eq $True) { "📂 $($_.Name)" } else { "📝 $($_.Name)"} }
} | Sort-Object Name

$choice = Read-Menu -Options ($files | Select-Object -ExpandProperty Name)  -ExitOption '❌ Exit'

Write-Host $choice
