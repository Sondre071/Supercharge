function Select-Model {
    [OutputType([void])]

    $models = & {
        $response = Invoke-RestMethod `
            -Headers @{ Authorization = "Bearer $($Config.ApiKey)" } `
            -Uri 'https://openrouter.ai/api/v1/models'

        $sortedModels = $response.data `
        | Sort-Object { $_.id -in $Config.Models }, created, id -Descending `
        | Select-Object -ExpandProperty id 

        return $sortedModels
    }

    $choice = Read-Menu `
        -Header 'Select model' `
        -Options $models `
        -ExitOption 'Back'

    switch ($choice) {
        'Back' { return }

        default {
            $Config.Model = $_
            $Config.Models = @($Config.Models; $_) | Sort-Object -Unique
            $Config._Save()

            Write-Host "Model set to: `'$_`'.`n" -ForegroundColor Green

            return
        }
    }
}