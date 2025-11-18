param (
    [Parameter(Mandatory)]
    [string]$Text,

    [Parameter(Mandatory)]
    [string]$Role
)

return @{
    type    = 'message'
    role    = $Role
    content = @(
        @{
            type = 'input_text'
            text = $Text
        }
    )
}