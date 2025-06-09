Import-Module "$PSScriptRoot/scripts/helpers/Read-Menu.psm1"

function SC() {
    $options = ("New session", "Settings")

    $action = Read-Menu -Options $options -Header "Supercharge071" -ExitOption 'Exit'

    switch ($action) {
        'New session' { . "$PSScriptRoot/scripts/chat.ps1" }

        'Settings' { Write-Host "I have no settings yet." }
 
        'Exit' { return }
    }
}

SC