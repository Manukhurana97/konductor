# Release Workflow

This project has two GitHub Actions workflows for building and releasing:

## 1. Automatic Releases (build.yml)
**Triggers:** Push to `main` branch
**What it does:**
- Automatically builds the application for all platforms
- Generates a new version number (increments patch version)
- Creates a new release tag and GitHub release
- Uploads build artifacts

**Example:** When you merge a PR to main, it will:
- Build v1.0.1 (if the last tag was v1.0.0)
- Create a GitHub release with the new version
- Upload the built applications

## 2. Manual Releases (main.yml)
**Triggers:** 
- Push with a tag (e.g., `git tag v2.0.0 && git push origin v2.0.0`)
- Manual workflow dispatch from GitHub Actions UI

**What it does:**
- Builds the application for all platforms
- Creates a release with the specified tag version
- Generates changelog from commits since the last tag
- Uploads build artifacts

## How to Use

### For Regular Development:
1. Push your code to a feature branch
2. Create a Pull Request to `main`
3. When merged, the automatic workflow will:
   - Build the app
   - Create a new release (e.g., v1.0.1)
   - Upload the binaries

### For Major/Minor Releases:
1. Create and push a tag:
   ```bash
   git tag v2.0.0
   git push origin v2.0.0
   ```
2. The manual workflow will create a release with that version

### For Manual Releases:
1. Go to GitHub Actions → Manual Release Builder
2. Click "Run workflow"
3. The workflow will build and create a release

## Versioning
- **Automatic releases:** Increment patch version (1.0.0 → 1.0.1)
- **Manual releases:** Use whatever version you specify in the tag

## Requirements
Make sure these secrets are set in your GitHub repository:
- `TAURI_PRIVATE_KEY`: Your Tauri signing key
- `TAURI_KEY_PASSWORD`: Password for the signing key
- `GITHUB_TOKEN`: Automatically provided by GitHub

## Troubleshooting
- If releases aren't being created, check that the secrets are properly configured
- If builds fail, check the GitHub Actions logs for platform-specific issues
- Make sure your main branch is protected and requires PR reviews if needed
