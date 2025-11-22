function Select-Prompt {
    [OutputType([array])]
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

    if ($prompts.Length -eq 0) {
        return $null, $false
    }

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
            return $null, $false
        }

        'Back' {
            return $null, $true
        }

        default {
            $prompt = Get-Content `
                -Path $choice.Path `
                -Raw
                
            return $prompt, $false
        }
    }
}