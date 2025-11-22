function Open-Settings {
    [OutputType([void])]
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $choice = Read-Menu `
        -Header 'Settings' `
        -Options 'Select model', 'Open settings file' `
        -ExitOption 'Back'

    switch ($choice) {
        'Open settings file' {
            & $Config._SavePath

            exit 0
        }

        'Select model' {
            $models = Get-Models `
                -Config $Config

            $choice = Read-Menu -Header 'Select model' -Options $models -ExitOption 'Back'

            switch ($choice) {
                'Back' { return }

                default {
                    $Config.Model = $choice
                    $Config._Save()

                    Write-Host "Model set to: `'$choice`'.`n" -ForegroundColor Green

                    return
                }
            }
        }

        default { return }
    }
}