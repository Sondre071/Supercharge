function Read-StreamLine {
    [OutputType([string])]
    param (
        [Parameter(Mandatory)]
        [AllowEmptyString()]
        [string]$LineStr
    )

    $valuesToSkip = (': OPENROUTER PROCESSING', 'data: [DONE]', '')

    if ($lineStr -in $valuesToSkip) { return '' }

    try {
        $line = $lineStr -replace 'data: ', '' | ConvertFrom-Json

        if ($line.type -eq 'response.output_text.delta') {
            return $line.delta
        }
    }
    catch {
        throw "Stream parsing error: `'$_`'."
    }
}