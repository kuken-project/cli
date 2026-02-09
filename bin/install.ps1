$ErrorActionPreference = 'Stop'

$repo = "kuken-project/cli"
$installDir = "$env:LOCALAPPDATA\Kuken"
$binPath = "$installDir\kuken-cli.exe"

Write-Host "Installing Küken CLI..." -ForegroundColor Cyan

$arch = if ([System.Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }
$binaryName = "kuken-windows-$arch.exe"

try {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest"
    $version = $release.tag_name
    Write-Host "Latest version: $version" -ForegroundColor Green
} catch {
    Write-Error "Failed to fetch latest release information"
    exit 1
}

$asset = $release.assets | Where-Object { $_.name -eq $binaryName }
if (-not $asset) {
    Write-Error "Binary not found for your system: $binaryName"
    exit 1
}

if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

Write-Host "Downloading $binaryName..." -ForegroundColor Cyan
try {
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $binPath
} catch {
    Write-Error "Failed to download binary"
    exit 1
}

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-Host "Adding to PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    $env:Path = "$env:Path;$installDir"
    Write-Host "Added $installDir to PATH. You may need to restart your terminal." -ForegroundColor Yellow
}

Write-Host "`nKüken CLI installed successfully!" -ForegroundColor Green
Write-Host "Version: $version" -ForegroundColor Gray
Write-Host "Location: $binPath" -ForegroundColor Gray
Write-Host "`nRun 'kuken --help' to get started" -ForegroundColor Cyan