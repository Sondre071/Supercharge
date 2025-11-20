function Open-Prompts {
    param (
        [Parameter(Mandatory)]
        [string]$Path
    )

    if (-not (Test-Path $Path)) {
        New-Item `
            -Path $Path `
            -ItemType Directory `
        | Out-Null
    }

    $prompts = Get-ChildItem `
        -Path $Path `
        -File 

    if ($null -eq $prompts) { return '' }

    $prompts = $prompts `
    | Select-Object `
    @{ Name = 'Name'; Expression = { $_.BaseName } }, `
    @{ Name = 'Path'; Expression = { $_.FullName } }

    $choice = Read-Menu `
        -Header 'Select prompt' `
        -Options $('None'; $prompts) `
        -ExitOption 'Back'

    switch ($choice) {
        'None' {
            return ''
        }

        'Back' {
            return $null
        }

        default {
            $prompt = Get-Content `
                -Path $choice.Path `
                -Raw

            # If file is empty.
            if ($null -eq $prompt) { return '' }

            return $prompt
        }
    }
}