function Get-Models {
    [OutputType([string[]])]
    param (
        [Parameter(Mandatory)]
        [hashtable]$Config
    )

    $response = Invoke-RestMethod `
        -Headers @{
        Authorization = "Bearer $($Config.ApiKey)"
    } `
        -Uri 'https://openrouter.ai/api/v1/models'

    return $response.data | Select-Object -ExpandProperty id
}