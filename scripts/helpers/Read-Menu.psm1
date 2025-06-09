function Write-MenuHeader($Header, $HeaderWidth = 40) {

    $headerMaxLength = $HeaderWidth - 12

    if ($Header.Length -gt $headerMaxLength) {
        $truncatedHeader = $Header.Substring(0, $headerMaxLength) + '..'
        $headerWithSpaces = " $truncatedHeader "
    }
    else {
        $headerWithSpaces = " $Header "
    }

    $paddingLength = [Math]::Max(0, ($HeaderWidth - $headerWithSpaces.Length) / 2)
    $padding = '=' * [Math]::Floor($paddingLength)
    $line = "$padding$headerWithSpaces$padding"

    if ($line.Length -lt $HeaderWidth) {
        $line += '='
    }

    Write-Host $line -ForegroundColor Yellow
}

function Clear-Menu($TotalMenuHeight) {

    # Jump $TotalMenuHeight lines up and clear everything below.
    Write-Host "$([char]27)[$($TotalMenuHeight)A" -NoNewLine
    Write-Host "$([char]27)[0J" -NoNewLine
}

function Read-Menu {
    param (
        [string[]]$Options,

        [string]$ExitOption,

        [string]$Header,

        [int]$HeaderWidth = 40,

        [string[]]$Subheaders,

        [string]$MenuTextColor = 'Yellow'
    )

    $combinedOptions = @()

    if ($Options) { $CombinedOptions += $Options }
    if ($ExitOption) { $CombinedOptions += $ExitOption }

    $hasHeader = -not [string]::IsNullOrWhiteSpace($Header)
    $hasSubheaders = $Subheaders.Count -gt 0

    $headerRowCount = 0

    if ($hasHeader) { $headerRowCount++ }
    if ($hasSubheaders) { $headerRowCount += $Subheaders.Count }

    $combinedOptionsHeight = $combinedOptions.Count
    $totalMenuHeight = $combinedOptionsHeight + $headerRowCount

    if ($hasHeader) {
        Write-MenuHeader -Header $Header -HeaderWidth $HeaderWidth
    }

    if ($hasSubheaders) {
        $Subheaders | ForEach-Object { Write-Host $_ -ForegroundColor $MenuTextColor }
    }

    $currentIndex = 0
    $startingRow = [System.Console]::CursorTop

    [System.Console]::CursorVisible = $False

    while ($true) {
        for ($i = 0; $i -lt $combinedOptionsHeight; $i++) {
            $color = if ($i -eq $currentIndex) { $MenuTextColor } else { 'Gray' }
            Write-Host ">  $($combinedOptions[$i])" -ForegroundColor $color
        }

        $keyInfo = $null

        # ReadKey is nested in a loop to enable script termination through SIGINT, AKA CTRL+C.
        while ($true) {
            if ([Console]::KeyAvailable) {
                $keyInfo = [Console]::ReadKey($true)
                break
            }
        }

        switch ($keyInfo.Key) {
            { $_ -in "UpArrow", "K" } {
                $currentIndex = [Math]::Max(0, $currentIndex - 1)
            }
            { $_ -in "DownArrow", "J" } {
                $currentIndex = [Math]::Min($combinedOptionsHeight - 1, $currentIndex + 1)
            }
            { $_ -in "Enter", "L" } {
                Clear-Menu -TotalMenuHeight $totalMenuHeight

                [System.Console]::CursorVisible = $true
                return $combinedOptions[$currentIndex]
            }
            { ($_ -in ("Escape", "Q", "H")) -and $ExitOption } {
                Clear-Menu -TotalMenuHeight $totalMenuHeight

                [System.Console]::CursorVisible = $true
                return $ExitOption
            }
        }

        # This is to correct for when the terminal scrolls after rendering the menu.
        $startingRow = [System.Console]::CursorTop - $combinedOptionsHeight
        [System.Console]::SetCursorPosition(0, $startingRow)
    }
}

function Read-Input() {
    param (
        [string]$Header,

        [int]$HeaderWidth = 40,

        [string[]]$Subheaders,

        [string]$Instruction = 'You',

        [string]$MenuTextColor = 'Yellow'
    )

    $startingRow = [System.Console]::CursorTop

    if ($Header) { Write-MenuHeader -Header $Header -HeaderWidth $HeaderWidth }
    if ($Subheaders -gt 0) { $Subheaders | ForEach-Object { Write-Host $_ -ForegroundColor $MenuTextColor } }

    $userInput = Read-Host $Instruction

    $currentRow = [System.Console]::CursorTop
    $totalMenuHeight = $currentRow - $startingRow

    Clear-Menu -TotalMenuHeight $TotalMenuHeight

    return $userInput
}

Export-ModuleMember -Function Read-Menu
Export-ModuleMember -Function Read-Input
Export-ModuleMember -Function Write-MenuHeader