function Open-Settings {
    [OutputType([void])]

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
            Select-Model
        }

        default { return }
    }
}