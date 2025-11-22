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

            return
        }

        'Select model' {
            Select-Model `
                -Config $Config
        }

        default { return }
    }
}