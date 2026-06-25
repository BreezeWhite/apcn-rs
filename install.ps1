$ErrorActionPreference = 'Stop'

# APCN PowerShell Installer Script for Windows
# Can be run via: irm <script-url> | iex
# Or with rug backend: $env:BACKEND="rug"; irm <script-url> | iex

$Repo = "BreezeWhite/apcn-rs"

$Backend = if ($env:BACKEND) { $env:BACKEND } else { "rug" }

# Retrieve releases from GitHub API
Write-Host "Fetching latest release version..." -ForegroundColor Cyan
try {
    $Releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases" -UseBasicParsing
    $Tag = $Releases[0].tag_name
} catch {
    Write-Error "Failed to fetch latest release tags from GitHub API."
    exit 1
}

if (-not $Tag) {
    Write-Error "Could not retrieve latest release tag."
    exit 1
}

$Platform = "windows"
$Arch = "x86_64"
$ArchiveName = "apcn-$Backend-$Platform-$Arch.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Tag/$ArchiveName"

# Target installation directory in User Profile
$InstallDir = Join-Path $env:USERPROFILE ".local\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Create a temporary directory
$TempDir = Join-Path $env:TEMP ([Guid]::NewGuid().Guid)
New-Item -ItemType Directory -Path $TempDir | Out-Null

try {
    Write-Host "Downloading apcn $Tag ($Backend backend for Windows-x86_64)..." -ForegroundColor Cyan
    $ZipPath = Join-Path $TempDir "archive.zip"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
    
    Write-Host "Extracting binary..." -ForegroundColor Cyan
    Expand-Archive -Path $ZipPath -DestinationPath $TempDir -Force
    
    $ExeSource = Join-Path $TempDir "apcn.exe"
    if (-not (Test-Path $ExeSource)) {
        Write-Error "Binary apcn.exe was not found in the downloaded archive."
        exit 1
    }
    
    $ExeDest = Join-Path $InstallDir "apcn.exe"
    Write-Host "Installing binary to $ExeDest..." -ForegroundColor Cyan
    Copy-Item -Path $ExeSource -Destination $ExeDest -Force
    
    Write-Host "Successfully installed apcn to $InstallDir\apcn.exe!" -ForegroundColor Green
    
    # Check and add to User PATH environment registry permanently
    $UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
    $PathList = $UserPath -split ';'
    
    # Normalize paths for comparison
    $NormalizedInstallDir = [System.IO.Path]::GetFullPath($InstallDir).TrimEnd('\')
    $IsFound = $false
    foreach ($p in $PathList) {
        if ($p -eq "") { continue }
        try {
            $NormalizedP = [System.IO.Path]::GetFullPath($p).TrimEnd('\')
            if ($NormalizedP -eq $NormalizedInstallDir) {
                $IsFound = $true
                break
            }
        } catch {}
    }
    
    if (-not $IsFound) {
        Write-Host "Adding $InstallDir to your User PATH environment variable permanently..." -ForegroundColor Yellow
        $NewUserPath = if ($UserPath -and -not $UserPath.EndsWith(';')) { "$UserPath;$InstallDir" } else { "$UserPath$InstallDir" }
        [Environment]::SetEnvironmentVariable("Path", $NewUserPath, [EnvironmentVariableTarget]::User)
        
        # Also update current session PATH
        $env:Path += ";$InstallDir"
        Write-Host "PATH successfully updated! Restart your terminal or shell to apply system-wide." -ForegroundColor Green
    } else {
        Write-Host "You can now run: apcn --help" -ForegroundColor Green
    }
}
finally {
    # Ensure temporary directory cleanup
    Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
}
