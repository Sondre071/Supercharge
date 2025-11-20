function Open-Settings {
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $choice = Read-Menu -Header 'Settings' -Options ('Open settings file')

    switch ($choice) {
        'Open settings file' {
            & $Config._savePath

            exit 0
        }
    }
}