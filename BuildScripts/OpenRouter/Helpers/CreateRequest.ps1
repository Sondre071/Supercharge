param (
    [Parameter(Mandatory)]
    [hashtable[]]$Messages,

    [Parameter(Mandatory)]
    [string]$Model,

    [Parameter(Mandatory)]
    [string]$ApiKey,
    
    [Parameter(Mandatory)]
    [string]$Url
)

return @{
    Uri     = $Url
    Method  = 'POST'
    Headers = @{
        'Authorization' = "Bearer $ApiKey"
        'Content-Type'  = 'application/json'
    }
    Body    = (
        ConvertTo-Json `
            -InputObject @{model = $Model; input = $Messages } `
            -Depth 7
    )
}