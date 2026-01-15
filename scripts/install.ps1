# Godot Addon Manager (gdam) installer for Windows
# Usage: irm https://raw.githubusercontent.com/moonrailgun/godot-addon-manager/master/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "moonrailgun/godot-addon-manager"
$BinaryName = "gdam"
$InstallDir = if ($env:GDAM_INSTALL_DIR) { $env:GDAM_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] " -ForegroundColor Green -NoNewline
    Write-Host $Message
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] " -ForegroundColor Yellow -NoNewline
    Write-Host $Message
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] " -ForegroundColor Red -NoNewline
    Write-Host $Message
    exit 1
}

function Get-Platform {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($arch) {
        "X64" { return "windows-x86_64" }
        "Arm64" {
            Write-Warn "ARM64 Windows is not officially supported, trying x86_64 version..."
            return "windows-x86_64"
        }
        default { Write-Error "Unsupported architecture: $arch" }
    }
}

function Get-LatestVersion {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers @{ "User-Agent" = "gdam-installer" }
        return $response.tag_name
    }
    catch {
        Write-Error "Failed to get latest version: $_"
    }
}

function Install-Gdam {
    Write-Host ""
    Write-Host "  ╔════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "  ║   Godot Addon Manager (gdam) Installer ║" -ForegroundColor Cyan
    Write-Host "  ╚════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""

    $platform = Get-Platform
    Write-Info "Detected platform: $platform"

    $version = Get-LatestVersion
    Write-Info "Latest version: $version"

    $downloadUrl = "https://github.com/$Repo/releases/download/$version/$BinaryName-$platform.zip"
    $tempDir = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "gdam-install-$(Get-Random)")
    $archivePath = Join-Path $tempDir "$BinaryName.zip"

    Write-Info "Downloading from: $downloadUrl"
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
    }
    catch {
        Write-Error "Download failed: $_"
    }

    Write-Info "Extracting..."
    Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $binaryPath = Join-Path $tempDir "$BinaryName.exe"
    $destPath = Join-Path $InstallDir "$BinaryName.exe"
    Move-Item -Path $binaryPath -Destination $destPath -Force

    Remove-Item -Path $tempDir -Recurse -Force
    Write-Info "Installed to: $destPath"

    # Check if InstallDir is in PATH
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        Write-Warn "Adding $InstallDir to your PATH..."
        $newPath = "$userPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Info "PATH updated. You may need to restart your terminal."
    }

    Write-Host ""
    Write-Info "Installation complete! Run 'gdam --help' to get started."
    Write-Host ""
}

Install-Gdam
