function Get-Config {
    param(
        [Parameter(Mandatory)]
        [string]$Path,

        [Parameter(Mandatory)]
        [string]$InitialJSONContent
    )

    if (-not ($Path -match ".json$")) { throw "File must be of type JSON." }

    if (-not (Test-Path $Path)) {
        if ($InitialJSONContent -ne '') {
            $content = $InitialJSONContent | ConvertFrom-Json -Depth 7 `
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
            throw 'No JSON provided to populate initial file.'
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