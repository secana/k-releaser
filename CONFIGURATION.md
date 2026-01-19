# Configuration

k-releaser is configured in your root `Cargo.toml` file under `[workspace.metadata.k-releaser]` for workspaces or `[package.metadata.k-releaser]` for single packages.

> **Note**: k-releaser is **command-driven, not config-driven**. Configuration controls *how* commands work (templates, labels, etc.), not *whether* they run. To create a release, run the `release` command. To skip a release, don't run it. Simple!

## Viewing Configuration

Use the `config show` command to inspect your current k-releaser configuration:

```bash
# Show configuration for current workspace
k-releaser config show

# Show configuration for a specific workspace
k-releaser config show --manifest-path path/to/Cargo.toml

# Filter to specific package
k-releaser config show --package my-package

# Output as JSON for scripting
k-releaser config show --output json
```

The output shows:
- **Workspace Defaults**: Settings that apply to all packages unless overridden
- **Workspace-Specific Settings**: Settings that don't apply to individual packages (PR config, repo URL, etc.)
- **Package Configurations**: Per-package overrides (only packages with explicit overrides are shown)

This is useful for:
- Understanding which values are explicitly set vs using defaults
- Debugging configuration issues in large workspaces
- Verifying per-package overrides are correctly applied

## Basic Configuration

```toml
[workspace.metadata.k-releaser]
# Only create releases when commits match this pattern (optional)
# Useful to skip releases for chore/docs commits
release_commits = "^(feat|fix):"

# Create and update CHANGELOG.md file (default: false)
# Set to true to maintain a changelog file in your repository
changelog_update = true

# Path to custom git-cliff changelog config (optional)
# Defaults to "keep a changelog" format
changelog_config = ".github/cliff.toml"
```

## Version Control

```toml
[workspace.metadata.k-releaser]
# Git tag name template (default: "v{{ version }}")
# Available variables: {{ version }}, {{ package }}
git_tag_name = "v{{ version }}"

# Maximum commits to analyze for first release (default: 1000)
max_analyze_commits = 2000

# Always increment minor version for feat: commits (default: false)
# By default, feat: commits on 0.x versions only bump the patch version
# (following semver: 0.x is considered unstable). Set to true to always
# bump the minor version for feat: commits, even on 0.x versions.
features_always_increment_minor = true
```

## Git Release Configuration

```toml
[workspace.metadata.k-releaser]
# Enable/disable GitHub/Gitea/GitLab releases (default: true)
git_release_enable = true

# Git release name template (optional)
# Available variables: {{ version }}, {{ package }}
git_release_name = "Release {{ version }}"

# Git release body template (optional)
# Uses changelog by default
git_release_body = "{{ changelog }}"

# Release type: "prod", "pre", or "auto" (default: "prod")
# "auto" marks as pre-release if version contains -rc, -beta, etc.
git_release_type = "auto"

# Create release as draft (default: false)
git_release_draft = false

# Mark release as latest (default: true)
git_release_latest = true
```

## Pull Request Configuration

```toml
[workspace.metadata.k-releaser]
# PR title template (optional)
pr_name = "chore: release {{ version }}"

# PR body template (optional)
# Available variables: {{ changelog }}, {{ version }}, {{ package }}
pr_body = """
## Release {{ version }}

{{ changelog }}
"""

# Create PR as draft (default: false)
pr_draft = false

# Labels to add to PR (optional)
pr_labels = ["release", "automated"]

# PR branch prefix (default: "release-plz-")
pr_branch_prefix = "release-"
```

## Changelog Customization

Advanced changelog customization using git-cliff templates:

```toml
[workspace.metadata.k-releaser.changelog]
# Changelog header
header = """
# Changelog

All notable changes to this project will be documented in this file.
"""

# Changelog entry template (Tera template)
body = """
## [{{ version }}]({{ release_link }}) - {{ timestamp | date(format="%Y-%m-%d") }}

{% for group, commits in commits | group_by(attribute="group") %}
### {{ group | upper_first }}
{% for commit in commits %}
  - {{ commit.message }}{% if commit.breaking %} **BREAKING**{% endif %}
{% endfor %}
{% endfor %}
"""

# Remove leading/trailing whitespace (default: true)
trim = true

# Sort commits: "oldest" or "newest" (default: "newest")
sort_commits = "newest"

# Protect breaking changes from being skipped (default: false)
protect_breaking_commits = true
```

## Repository Settings

```toml
[workspace.metadata.k-releaser]
# Repository URL (defaults to git remote)
# Used for generating changelog links
repo_url = "https://github.com/your-org/your-repo"

# Allow dirty working directory (default: false)
allow_dirty = false

# Update all dependencies in Cargo.lock (default: false)
# If false, only updates workspace packages
dependencies_update = false
```

## Per-Package Overrides

Override settings for specific packages. Each package override is defined with `[[workspace.metadata.k-releaser.package]]` (note the double brackets - this creates an array of package configurations):

```toml
# Package-specific override for special cases
[[workspace.metadata.k-releaser.package]]
name = "my-package"
# Custom changelog path for this package
changelog_path = "packages/my-package/CHANGELOG.md"
# Allow publishing even if git working directory is dirty
publish_allow_dirty = true

# Another package override
[[workspace.metadata.k-releaser.package]]
name = "my-codegen-package"
# Disable semver checking for codegen packages
semver_check = false
```

**Note**: With unified versioning (where all packages share the same version), per-package overrides are rarely needed. Most configuration should be done at the workspace level.

**Available per-package settings:**
- `changelog_path` - Custom path for package changelog
- `changelog_update` - Enable/disable changelog updates
- `publish_allow_dirty` - Allow publishing with dirty git state
- `publish_no_verify` - Skip build verification before publish
- `publish_features` - Features to enable during publish
- `publish_all_features` - Publish with all features enabled
- `semver_check` - Enable/disable semver compatibility checking
- `git_tag_name` - Custom tag name template
- `git_tag_enable` - Enable/disable git tag creation
- `git_release_enable` - Enable/disable git release creation
- `git_release_name` - Custom release name template
- `git_release_body` - Custom release body template
- `git_release_type` - Release type (prod/pre/auto)
- `git_release_draft` - Create as draft release
- `git_release_latest` - Mark as latest release
- `features_always_increment_minor` - Treat feature additions as minor bumps

## Complete Example

Here's a complete example showing workspace-level settings and per-package overrides:

```toml
[workspace.metadata.k-releaser]
# Core settings
changelog_update = true
release_commits = "^(feat|fix|perf):"

# Git configuration
git_tag_name = "v{{ version }}"
git_release_name = "{{ version }}"
git_release_type = "auto"

# PR configuration
pr_draft = false
pr_labels = ["release"]
pr_branch_prefix = "release-"

# Repository
repo_url = "https://github.com/your-org/your-repo"
dependencies_update = false

# Per-package override for embedded resources
[[workspace.metadata.k-releaser.package]]
name = "my-embedded-resources"
publish_allow_dirty = true

# Per-package override for CLI binary
[[workspace.metadata.k-releaser.package]]
name = "my-cli"
publish_all_features = true
```

Use `k-releaser config show` to verify your configuration is loaded correctly.
