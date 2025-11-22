function Get-Config {
    [OutputType([hashtable])]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $false)]
        [hashtable]$InitialContent
    )

    if (-not ($Path -match ".json$")) { throw "File must be of type JSON." }

    if (-not (Test-Path $Path)) {
        if ($null -ne $InitialContent) {
            $content = $InitialContent | ConvertTo-Json `
                -Depth 7 `
                -Compress

            New-Item `
                -Path $Path `
                -Value $content `
                -Force
            | Out-Null
        }
        else {
            throw 'No file found, no initial content provided.'
        }
    }

    $obj = (Get-Content -Path $Path) | ConvertFrom-Json -Depth 7 -AsHashTable

    $obj | Add-Member `
        -Membertype NoteProperty `
        -Name _SavePath `
        -Value $Path
    
    $saveMethod = {
        $json = ($this `
            | Select-Object `
                -ExcludeProperty _SavePath `
            | ConvertTo-Json `
                -Depth 7
        )

        Set-Content `
            -Path $this._SavePath `
            -Value $json
    }

    $obj | Add-Member `
        -MemberType ScriptMethod `
        -Name _Save `
        -Value $saveMethod

    return $obj
}