param (
    [string]$ProjectRoot
)

$choice = Read-Menu -Options ('Files')  -ExitOption 'Exit'

switch ($choice) {
    'Files' {
        . $ProjectRoot\Scripts\Menu\FilesMenu.ps1 -ProjectRoot $ProjectRoot
    }
}