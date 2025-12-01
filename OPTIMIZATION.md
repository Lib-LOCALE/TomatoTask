# 🚀 Application Size Optimization Guide

This document explains the optimizations applied to reduce TomatoTask's bundle size.

## 📊 Optimizations Applied

### 1. ✅ Rust Binary Optimization (Cargo.toml)
**Impact: ~2-3 MB reduction**

Added aggressive release profile settings:
```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Enable Link Time Optimization
codegen-units = 1      # Better optimization
strip = true           # Strip symbols from binary
panic = "abort"        # Smaller panic runtime
```

### 2. ✅ Screenshots Moved Out of Bundle
**Impact: ~120 KB reduction**

Screenshots moved from `public/captureImg/` to `.github/images/` to exclude them from the application bundle. They're only needed for the GitHub README.

### 3. 🔄 Audio Optimization (Requires Manual Conversion)
**Impact: ~90 MB reduction (105 MB → ~10-15 MB)**

Ambient sound files need to be converted from MP3 to OGG Vorbis format with optimized bitrate.

#### How to Convert Audio Files:

**Step 1: Install ffmpeg**

On Windows with Chocolatey:
```powershell
choco install ffmpeg
```

Or download from: https://ffmpeg.org/download.html

**Step 2: Run the Conversion Script**

```powershell
.\convert-sounds.ps1
```

This script will:
- Convert all ambient sounds (birds, forest, sea, storm) to OGG Vorbis @ 64kbps
- Backup original MP3 files to `public/sounds/originals/`
- Show size reduction statistics

**Step 3: Test the Application**

The code has already been updated to use `.ogg` files instead of `.mp3`. After conversion:

```bash
npm run dev
```

Test each ambient sound in the app to ensure they work correctly.

## 📈 Expected Results

| Component | Before | After | Savings |
|-----------|--------|-------|---------|
| Rust Binary | ~8 MB | ~5-6 MB | 2-3 MB |
| Screenshots | 120 KB | 0 KB | 120 KB |
| Ambient Sounds | 105 MB | 10-15 MB | ~90 MB |
| **Total** | **~113 MB** | **~15-21 MB** | **~92-98 MB (82-87%)** |

## 🔍 Additional Optimization Opportunities

### Frontend Bundle Analysis

To analyze the frontend bundle size:

```bash
npm run build
```

Check the `dist/` folder size and Vite's build output for detailed asset sizes.

### Dependencies Audit

Review package.json for unused dependencies:

```bash
npm list --depth=0
```

Consider removing or replacing large dependencies if not essential.

## ⚡ Build Performance

The optimizations may increase build time slightly:
- LTO (Link Time Optimization): +10-20 seconds
- Size optimization (opt-level = "z"): +5-10 seconds

This is a worthwhile tradeoff for significantly smaller binaries.

## 📝 Notes

- **OGG vs MP3**: OGG Vorbis provides better compression than MP3 at lower bitrates, ideal for ambient sounds
- **64 kbps**: Perfect for ambient/background sounds where absolute fidelity isn't critical
- **Browser Support**: OGG is supported by all modern browsers and Electron/Tauri
- **Original Files**: Always kept in `public/sounds/originals/` as backup

## 🎯 Future Optimizations

Consider for future releases:
- [ ] Lazy loading for rarely-used features
- [ ] Code splitting for different sections of the app
- [ ] Further Rust binary optimization with `upx` compression (optional)
- [ ] Image optimization (SVGs are already optimal)

---

**Made with 🍅 and optimizations!**
