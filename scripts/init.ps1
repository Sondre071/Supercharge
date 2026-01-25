$dataPath = Join-Path $env:UserProfile '.supercharge' 'data'
$scriptsPath = Join-Path $env:UserProfile '.supercharge' 'scripts'
$openrouterDataPath = Join-Path $dataPath 'openrouter.json'
$blobstorageDataPath = Join-Path $dataPath 'blobstorage.json'

if (-not (Test-Path $dataPath))
{
    New-Item -ItemType Directory -Path $dataPath
} else
{
    Write-Host "data-path already exists, skipping this step.." -ForegroundColor DarkGreen
}

if (-not (Test-Path $scriptsPath))
{
    New-Item -ItemType Directory -Path $scriptsPath
} else
{
    Write-Host "scripts-path already exists, skipping this step.." -ForegroundColor DarkGreen
}

if (Test-Path $openRouterDataPath)
{
    Write-Host "openrouter.json already exists, skipping this step.." -ForegroundColor DarkGreen
} else
{
    $initialContent = @{
        api_key    = ""
        model      = "anthropic/claude-sonnet-4.5"
        models     = @(
            "anthropic/claude-haiku-4.5",
            "anthropic/claude-sonnet-4.5",
            "openai/gpt-5.1"
        )
        parameters = @{
            temperature        = 0.6
            top_p              = 0.2
            top_k              = 0
            frequency_penalty  = 0.0
            presence_penalty   = 0.0
            repetition_penalty = - 1.0
            min_p              = 0.0
            top_a              = 0.0
        }
    }

    $json = $initialContent | ConvertTo-Json `
        -Depth 7 `
        -Compress

    New-Item `
        -Path $openrouterDataPath `
        -Value $json
}

if (Test-Path $blobstorageDataPath)
{
    Write-Host "blobstorage.json already exists, skipping this step.." -ForegroundColor DarkGreen
} else
{
    $initialContent = @{
        storage_accounts = @(
            @{
                name              = ""
                local_files_path  = ""
                connection_string = ""
            }
        )
    }

    $json = $initialContent | ConvertTo-Json `
        -Depth 7 `
        -Compress

    New-Item `
        -Path $blobstorageDataPath `
        -Value $json
}
