param (
    [string]$ProjectRoot
)

$currentFolder = 'data'

while ($true) {
    $path = Join-Path $ProjectRoot $currentFolder

    [psobject]$files = @()
    
    $files += Get-ChildItem `
        -Path $path `
    | Select-Object Name, PSIsContainer, @{
        Name       = 'Icon'
        Expression = { $true -eq $_.PSIsContainer ? "📂" : "📝"  }
    } | Sort-Object Icon

    $files += [psobject]@{
        Name = 'New'
        Icon = '➕'
    }

    $header = "$currentFolder"

    $choice = Read-Menu -Header $header -HeaderSymbol '-' -HeaderWidth 25 -Options $files -ExitOption @{Name = 'Back'; Icon = '❌' }

    if ($choice.Name -eq 'Back') {

        if ($currentFolder -eq 'data') { return }

        $currentFolder = Split-Path -Parent $currentFolder
        continue
    }

    if ($choice.PSIsContainer -eq $true) {
        $currentFolder = Join-Path $currentFolder $choice.Name
        continue
    }

    if ($choice.Name -eq 'New') {
        $typeChoice = Read-Menu -Options ('File', 'Folder') -ExitOption 'Cancel'

        switch ($typeChoice) {
            'File' {
                $name = Read-Input -Header 'New file' -Instruction 'Name'
                
                New-Item -Name $name -Path "$path" | Out-Null
            }

            'Folder' {
                $name = Read-Input -Header 'New folder' -Instruction 'Name'
                
                New-Item -Name $name -Path "$path" -ItemType Directory | Out-Null
            }
        }

        continue
    }

    $filePath = Join-Path $ProjectRoot $currentFolder $choice.Name

    nvim $filePath
}
