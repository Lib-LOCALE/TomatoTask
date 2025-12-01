# Script to convert MP3 ambient sounds to optimized OGG format
# This reduces file size from ~105MB to ~10-15MB without quality loss for ambient sounds
#
# Prerequisites: Install ffmpeg
# Windows: choco install ffmpeg OR download from https://ffmpeg.org/download.html
#
# Usage: .\convert-sounds.ps1

Write-Host "🎵 Converting ambient sounds to optimized OGG format..." -ForegroundColor Cyan

# Check if ffmpeg is installed
try {
    $null = ffmpeg -version
} catch {
    Write-Host "❌ Error: ffmpeg is not installed or not in PATH" -ForegroundColor Red
    Write-Host "Install ffmpeg first:" -ForegroundColor Yellow
    Write-Host "  - Using Chocolatey: choco install ffmpeg" -ForegroundColor Yellow
    Write-Host "  - Or download from: https://ffmpeg.org/download.html" -ForegroundColor Yellow
    exit 1
}

$soundsDir = "public/sounds"
$tempDir = "$soundsDir/temp"

# Create temp directory
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

# Convert each MP3 to OGG with 64kbps (perfect for ambient sounds)
$files = @("birds.mp3", "forest.mp3", "sea.mp3", "storm.mp3")

foreach ($file in $files) {
    $inputFile = "$soundsDir/$file"
    $outputFile = "$tempDir/$($file -replace '\.mp3$', '.ogg')"

    if (Test-Path $inputFile) {
        Write-Host "Converting $file..." -ForegroundColor Green
        ffmpeg -i $inputFile -c:a libvorbis -q:a 3 -ar 44100 $outputFile -y 2>$null

        if ($LASTEXITCODE -eq 0) {
            $originalSize = (Get-Item $inputFile).Length / 1MB
            $newSize = (Get-Item $outputFile).Length / 1MB
            $savings = $originalSize - $newSize
            Write-Host "  ✓ $file converted: $([math]::Round($originalSize, 2))MB → $([math]::Round($newSize, 2))MB (saved $([math]::Round($savings, 2))MB)" -ForegroundColor Green
        } else {
            Write-Host "  ✗ Failed to convert $file" -ForegroundColor Red
        }
    }
}

# Backup originals
Write-Host "`n📦 Creating backup of original files..." -ForegroundColor Cyan
$backupDir = "$soundsDir/originals"
New-Item -ItemType Directory -Force -Path $backupDir | Out-Null

foreach ($file in $files) {
    $inputFile = "$soundsDir/$file"
    if (Test-Path $inputFile) {
        Move-Item $inputFile "$backupDir/$file" -Force
    }
}

# Move converted files to sounds directory
Write-Host "📁 Moving converted files..." -ForegroundColor Cyan
Get-ChildItem "$tempDir/*.ogg" | ForEach-Object {
    Move-Item $_.FullName "$soundsDir/$($_.Name)" -Force
}

# Clean up temp directory
Remove-Item $tempDir -Recurse -Force

Write-Host "`n✨ Conversion complete!" -ForegroundColor Green
Write-Host "Original files backed up to: $backupDir" -ForegroundColor Yellow
Write-Host "`n⚠️  Don't forget to update the audio file references in your code from .mp3 to .ogg" -ForegroundColor Yellow

# Calculate total savings
$totalOriginal = 0
$totalNew = 0
Get-ChildItem "$backupDir/*.mp3" | ForEach-Object { $totalOriginal += $_.Length }
Get-ChildItem "$soundsDir/*.ogg" | ForEach-Object { $totalNew += $_.Length }

Write-Host "`n📊 Total size reduction: $([math]::Round($totalOriginal / 1MB, 2))MB → $([math]::Round($totalNew / 1MB, 2))MB" -ForegroundColor Cyan
Write-Host "   Saved: $([math]::Round(($totalOriginal - $totalNew) / 1MB, 2))MB ($([math]::Round((($totalOriginal - $totalNew) / $totalOriginal) * 100, 1))% reduction)" -ForegroundColor Green
