# Submitting TomatoTask to Flathub

This guide explains how to submit and maintain TomatoTask on Flathub.

## Prerequisites

1. GitHub account
2. Join the [Flathub organization](https://github.com/flathub)
3. Read the [Flathub submission guidelines](https://docs.flathub.org/docs/for-app-authors/submission/)

## Initial Submission

### Step 1: Create Flathub Repository

1. Go to https://github.com/flathub/flathub
2. Create a new repository request via issue or PR
3. Repository name must be: `com.tomatotask.app`

### Step 2: Prepare Repository Content

Create a new repository `flathub/com.tomatotask.app` with the following files:

```
com.tomatotask.app/
├── com.tomatotask.app.yml          # Main manifest (from this directory)
├── com.tomatotask.app.metainfo.xml # AppStream metadata (from this directory)
├── flathub.json                    # Flathub configuration (from this directory)
└── README.md                       # Optional: build instructions
```

### Step 3: Configure Auto-updates

The `flathub.json` file in this directory configures automatic version checking:

```json
{
  "only-arches": ["x86_64", "aarch64"],
  "automerge-flathubbot-prs": true,
  "end-of-life": null,
  "end-of-life-rebase": null
}
```

### Step 4: Update Manifest for Flathub

Before submitting, update the manifest (`com.tomatotask.app.yml`):

1. Change the git source to use the latest tag:
   ```yaml
   sources:
     - type: git
       url: https://github.com/AnthonyMahe/TomatoTask.git
       tag: v1.0.8
       commit: <full-commit-hash>
   ```

2. Generate the source files:
   ```bash
   # Generate Node.js sources
   flatpak-node-generator npm ../package-lock.json -o generated-sources.json

   # Generate Cargo sources
   python3 -m flatpak_cargo_generator ../src-tauri/Cargo.lock -o cargo-sources.json
   ```

3. Add the generated files to the repository

### Step 5: Test the Build

Before submitting, test the build locally:

```bash
# Install required runtimes
flatpak install flathub org.freedesktop.Platform//23.08
flatpak install flathub org.freedesktop.Sdk//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.node20//23.08

# Build
flatpak-builder --force-clean build-dir com.tomatotask.app.yml

# Test run
flatpak-builder --run build-dir com.tomatotask.app.yml tomatotask

# Install locally
flatpak-builder --user --install --force-clean build-dir com.tomatotask.app.yml
```

### Step 6: Submit to Flathub

1. Push the files to `flathub/com.tomatotask.app` repository
2. Create a pull request to the Flathub repository
3. Wait for Flathub bot to run quality checks
4. Address any review comments from Flathub maintainers
5. Once approved, the app will be published to Flathub

## Updating on Flathub

When releasing a new version:

1. Update the version in the manifest
2. Update the git tag and commit hash
3. Regenerate source files if dependencies changed
4. Update the release notes in metainfo.xml
5. Create a PR to `flathub/com.tomatotask.app`

### Automated Updates

With `automerge-flathubbot-prs: true` in `flathub.json`, the Flathub bot will:
- Detect new GitHub releases automatically
- Create PRs with updated manifest
- Auto-merge if all checks pass

## Quality Checks

Flathub will verify:
- ✅ Valid AppStream metadata
- ✅ Proper sandboxing permissions
- ✅ No bundled dependencies (all from Flatpak)
- ✅ Desktop file validation
- ✅ Icon presence and format
- ✅ License compatibility
- ✅ Build reproducibility

## Common Issues

### Issue: Build timeout
**Solution:** Optimize build process, reduce dependencies

### Issue: Missing desktop file
**Solution:** Ensure Tauri generates .desktop file or create manually

### Issue: Icon not found
**Solution:** Verify icon paths in manifest match actual files

### Issue: AppStream validation fails
**Solution:** Use `appstreamcli validate` to check metainfo.xml

## Resources

- [Flathub Documentation](https://docs.flathub.org/)
- [Flatpak Builder Documentation](https://docs.flatpak.org/en/latest/flatpak-builder.html)
- [AppStream Guidelines](https://www.freedesktop.org/software/appstream/docs/)
- [Flathub Quality Guidelines](https://docs.flathub.org/docs/for-app-authors/appdata-guidelines/)
