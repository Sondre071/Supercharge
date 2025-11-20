function Open-Settings {
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $choice = Read-Menu -Header 'Settings' -Options ('Open config file')

    switch ($choice) {
        'Open config file' {
            & $($config._savePath)
        }
    }
}