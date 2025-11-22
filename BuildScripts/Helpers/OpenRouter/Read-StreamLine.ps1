function Read-StreamLine {
    [OutputType([string])]
    param (
        [Parameter(Mandatory)]
        [System.IO.StreamReader]$Reader,

        [Parameter(Mandatory)]
        [string]$Color
    )

    $line = $Reader.ReadLine()

    $valuesToSkip = (': OPENROUTER PROCESSING', 'data: [DONE]', '')

    if ($line -in $valuesToSkip) { continue }

    try {
        $line = ($line.Substring(6) | ConvertFrom-Json)

        if ($line.type -eq 'response.output_text.delta') {
            Write-Host $line.delta -NoNewLine -ForegroundColor $Color

            return $line.delta
        }
    }
    catch {
        throw "Stream parsing error: `'$_`'."
    }
}