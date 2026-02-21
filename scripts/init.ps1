$dataPath = Join-Path $env:UserProfile 'AppData' 'Local' 'Supercharge'

$openrouterDataPath = Join-Path $dataPath 'openrouter.json'
$blobstorageDataPath = Join-Path $dataPath 'blobstorage.json'

$folderPaths = @(
    $dataPath,
    (Join-Path $dataPath 'scripts'),
    (Join-Path $dataPath 'snippets')
)

Write-Host "======= Validating local files =======" -ForegroundColor Yellow

foreach ($path in $folderPaths)
{
    $name = Split-Path $path -Leaf

    if (-not (Test-Path $path))
    {
        New-Item -ItemType Directory -Path $path
    } else
    {
        Write-Host "✅ $name path already exists." -ForegroundColor Green
    }
}

if (Test-Path $openRouterDataPath)
{
    Write-Host "✅ openrouter.json already exists." -ForegroundColor Green
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
    Write-Host "✅ blobstorage.json already exists." -ForegroundColor Green
} else
{
    $initialContent = @{
        storage_accounts = @(
            @{
                name              = ""
                local_files_path  = ""
                connection_string = ""
                nested_containers = ""
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

$nvimCommand = Get-Command -ErrorAction SilentlyContinue nvim
if ($null -eq $nvimCommand)
{
    Write-Host "❌ Neovim missing from path." -ForegroundColor Red
} else
{
    Write-Host "✅ Neovim found in path." -ForegroundColor Green
}
