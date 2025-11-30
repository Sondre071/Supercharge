function Read-Menu {
    param (
        [string]$Header,
        [string[]]$Subheaders,
        [object[]]$Options,
        [object]$ExitOption
    )

    $binaryPath = Join-Path $ProjectRoot 'bin' 'release' 'read-menu.exe'

    $argsList = @()
    $argsList += "--width={0}" -f (Get-Host).UI.RawUI.WindowSize.Width  

    $argsList += "--options={0}" -f (($Options | ForEach-Object {
                Get-OptionName($_)
            }) -join ',')

    if ($Header -ne '') {
        $argsList += ("--header={0}" -f $Header)
    }
    if ($null -ne $Subheaders -and $Subheaders.Length -gt 0) {
        $argsList += "--subheaders={0}" -f ($Subheaders -join ',')
    }
    if ($null -ne $ExitOption) {
        $argsList += "--exit-option={0}" -f (Get-OptionName($ExitOption))
    }

    [System.Console]::CursorVisible = $false
    
    & $binaryPath @argsList | Out-Host

    $code = $LASTEXITCODE

    [System.Console]::CursorVisible = $false

    return $($code -eq -1 ? $ExitOption : $Options[$code])
}

function Get-OptionName([object]$Option) {
    $type = $Option.GetType().Name

    switch ($type) {
        'Hashtable' {
            return $Option.ContainsKey("Name") ? $Option.Name : $Option
        }

        'String' {
            return $Option
        }
    }
}