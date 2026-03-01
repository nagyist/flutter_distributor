# fastforge Windows installer
# Usage:
#   iwr https://fastforge.dev/install.ps1 | iex
#   $env:FASTFORGE_VERSION="0.7.0"; iwr https://fastforge.dev/install.ps1 | iex

$ErrorActionPreference = 'Stop'

$Repo        = "fastforgedev/fastforge"
$BinaryName  = "fastforge"
$InstallDir  = if ($env:FASTFORGE_INSTALL_DIR) { $env:FASTFORGE_INSTALL_DIR } else { "$env:LOCALAPPDATA\fastforge\bin" }

# ── Helpers ───────────────────────────────────────────────────────────────────
function Write-Info    { param($msg) Write-Host "info  $msg" -ForegroundColor Cyan }
function Write-Success { param($msg) Write-Host "✓     $msg" -ForegroundColor Green }
function Write-Warn    { param($msg) Write-Host "warn  $msg" -ForegroundColor Yellow }
function Write-Fail    { param($msg) Write-Host "error $msg" -ForegroundColor Red; exit 1 }

# ── Detect architecture ───────────────────────────────────────────────────────
function Get-Arch {
  $arch = $env:PROCESSOR_ARCHITECTURE
  switch ($arch) {
    "AMD64" { return "x86_64" }
    "ARM64" { return "aarch64" }
    default { Write-Fail "Unsupported architecture: $arch" }
  }
}

# ── Map to Rust target triple ─────────────────────────────────────────────────
function Get-Target {
  param($arch)
  switch ($arch) {
    "x86_64"   { return "x86_64-pc-windows-msvc" }
    "aarch64"  { return "aarch64-pc-windows-msvc" }
    default    { Write-Fail "No prebuilt binary available for architecture: $arch" }
  }
}

# ── Resolve version ───────────────────────────────────────────────────────────
function Resolve-Version {
  if ($env:FASTFORGE_VERSION) {
    Write-Info "Using specified version: $($env:FASTFORGE_VERSION)"
    return $env:FASTFORGE_VERSION
  }

  Write-Info "Fetching latest release version..."

  try {
    $headers = @{ "User-Agent" = "fastforge-installer" }
    $response = Invoke-RestMethod `
      -Uri "https://api.github.com/repos/$Repo/releases/latest" `
      -Headers $headers
    $tag = $response.tag_name -replace '^v', ''
    if (-not $tag) { Write-Fail "Failed to parse version from GitHub API response." }
    Write-Info "Latest version: $tag"
    return $tag
  } catch {
    Write-Fail "Failed to fetch latest version from GitHub: $_`nSet `$env:FASTFORGE_VERSION to specify a version manually."
  }
}

# ── Check if release already on PATH ─────────────────────────────────────────
function Get-InstalledVersion {
  try {
    $v = & fastforge --version 2>$null
    return $v
  } catch {
    return $null
  }
}

# ── Download ──────────────────────────────────────────────────────────────────
function Download-Archive {
  param($version, $target)

  $archiveName = "$BinaryName-$version-$target.zip"
  $downloadUrl = "https://github.com/$Repo/releases/download/v$version/$archiveName"
  $tmpDir      = Join-Path $env:TEMP "fastforge-install-$([System.IO.Path]::GetRandomFileName())"
  $archivePath = Join-Path $tmpDir $archiveName

  New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

  Write-Info "Downloading $archiveName..."
  Write-Info "  from $downloadUrl"

  try {
    $progressPreference = $global:ProgressPreference
    $global:ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
    $global:ProgressPreference = $progressPreference
  } catch {
    Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    Write-Fail "Download failed. Check your network or verify that version v$version exists.`n$_"
  }

  Write-Success "Downloaded $archiveName"
  return @{ TmpDir = $tmpDir; ArchivePath = $archivePath; ArchiveName = $archiveName }
}

# ── Install ───────────────────────────────────────────────────────────────────
function Install-Binary {
  param($tmpDir, $archivePath, $version, $target)

  Write-Info "Extracting archive..."
  Expand-Archive -Path $archivePath -DestinationPath $tmpDir -Force

  $binaryPath = Join-Path $tmpDir "$BinaryName-$version-$target\$BinaryName.exe"
  if (-not (Test-Path $binaryPath)) {
    Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    Write-Fail "Binary not found in archive at expected path: $binaryPath"
  }

  if (-not (Test-Path $InstallDir)) {
    Write-Info "Creating install directory: $InstallDir"
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  }

  $dest = Join-Path $InstallDir "$BinaryName.exe"
  Move-Item -Path $binaryPath -Destination $dest -Force
  Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue

  Write-Success "Installed $BinaryName to $dest"
}

# ── Update PATH ───────────────────────────────────────────────────────────────
function Update-UserPath {
  $currentPath = [System.Environment]::GetEnvironmentVariable("PATH", "User")
  if ($currentPath -notlike "*$InstallDir*") {
    Write-Info "Adding $InstallDir to your user PATH..."
    $newPath = "$InstallDir;$currentPath"
    [System.Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
    $env:PATH = "$InstallDir;$env:PATH"
    Write-Success "PATH updated. Restart your terminal to apply changes."
  } else {
    Write-Info "$InstallDir is already in your PATH."
  }
}

# ── Verify ────────────────────────────────────────────────────────────────────
function Verify-Install {
  $exePath = Join-Path $InstallDir "$BinaryName.exe"
  if (Test-Path $exePath) {
    try {
      $v = & $exePath --version 2>$null
      Write-Success "Installation verified: $v"
    } catch {
      Write-Success "Binary installed at $exePath"
    }
  } else {
    Write-Warn "Could not verify installation. Binary not found at $exePath"
  }
}

# ── Main ──────────────────────────────────────────────────────────────────────
function Main {
  Write-Host ""
  Write-Host "fastforge installer" -ForegroundColor White -BackgroundColor DarkGreen
  Write-Host ""

  $arch    = Get-Arch
  $target  = Get-Target -arch $arch
  $version = Resolve-Version

  Write-Info "Platform : windows-$arch"
  Write-Info "Target   : $target"
  Write-Info "Version  : $version"
  Write-Host ""

  $dl = Download-Archive -version $version -target $target
  Install-Binary -tmpDir $dl.TmpDir -archivePath $dl.ArchivePath -version $version -target $target
  Update-UserPath
  Verify-Install

  Write-Host ""
  Write-Host "fastforge $version installed successfully!" -ForegroundColor Green
  Write-Host ""
  Write-Host "Run " -NoNewline
  Write-Host "fastforge --help" -ForegroundColor Cyan -NoNewline
  Write-Host " to get started."
  Write-Host ""
}

Main
