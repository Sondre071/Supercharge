function Get-Config {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $false)]
        [hashtable]$InitialContent
    )

    if (-not ($Path -match ".json$")) { throw "File must be of type JSON." }

    if (-not (Test-Path $Path)) {
        if ($null -ne $InitialContent) {
            $content = $InitialContent `
            | ConvertTo-Json `
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

    if ($obj.PSObject.Properties.Name.Contains('Save')) {
        throw "JSON cannot contain a property named `'Save`' at root level."
    }

    $obj | Add-Member `
        -Membertype NoteProperty `
        -Name _savePath `
        -Value $Path
    
    $saveMethod = {
        $json = ($this `
            | Select-Object `
                -ExcludeProperty _savePath `
            | ConvertTo-Json `
                -Depth 7
        )

        Set-Content `
            -Path $this._savePath `
            -Value $json
    }

    $obj | Add-Member `
        -MemberType ScriptMethod `
        -Name Save `
        -Value $saveMethod

    return $obj
}