function Select-Model {
    [OutputType([void])]
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config,

        [switch]$UseApi
    )

    $models = & {
        if ($UseApi) {
            $response = Invoke-RestMethod `
                -Headers @{ Authorization = "Bearer $($Config.ApiKey)" } `
                -Uri 'https://openrouter.ai/api/v1/models'

            return $response.data | Select-Object -ExpandProperty id
        }
        else {
            return $Config.Models
        }
    }

    $choice = Read-Menu -Header 'Select model' -Options $models -ExitOption 'Back'

    switch ($choice) {
        'Back' { return }

        default {
            $Config.Model = $_
            $Config._Save()

            Write-Host "Model set to: `'$_`'.`n" -ForegroundColor Green

            return
        }
    }
}