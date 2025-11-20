function Get-Models {
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $models = Invoke-RestMethod `
        -Headers @{
        Authorization = "Bearer $($Config.ApiKey)"
    } `
        -Uri 'https://openrouter.ai/api/v1/models'

    return ($models.data | Select-Object -ExpandProperty id)
}