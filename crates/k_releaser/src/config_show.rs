use cargo_metadata::Package;
use serde::Serialize;
use std::collections::HashMap;

use crate::args::{config::ShowConfig, manifest_command::ManifestCommand};
use crate::config::{Config, PackageConfig};

#[derive(Serialize, Debug)]
pub struct ConfigDisplay {
    config_source: String,
    workspace_defaults: WorkspaceDefaultsDisplay,
    workspace_overrides: WorkspaceOverridesDisplay,
    packages: Vec<PackageConfigDisplay>,
}

#[derive(Serialize, Debug)]
pub struct WorkspaceDefaultsDisplay {
    changelog_path: Option<String>,
    changelog_update: Option<bool>,
    features_always_increment_minor: Option<bool>,
    git_release_enable: Option<bool>,
    git_release_body: Option<String>,
    git_release_type: Option<String>,
    git_release_draft: Option<bool>,
    git_release_latest: Option<bool>,
    git_release_name: Option<String>,
    git_tag_enable: Option<bool>,
    git_tag_name: Option<String>,
    publish_allow_dirty: Option<bool>,
    publish_no_verify: Option<bool>,
    publish_features: Option<Vec<String>>,
    publish_all_features: Option<bool>,
    semver_check: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct WorkspaceOverridesDisplay {
    allow_dirty: Option<bool>,
    changelog_config: Option<String>,
    dependencies_update: Option<bool>,
    pr_name: Option<String>,
    pr_body: Option<String>,
    pr_draft: bool,
    pr_labels: Vec<String>,
    pr_branch_prefix: Option<String>,
    publish_timeout: Option<String>,
    repo_url: Option<String>,
    release_commits: Option<String>,
    release_always: Option<bool>,
    max_analyze_commits: Option<u32>,
}

#[derive(Serialize, Debug)]
pub struct PackageConfigDisplay {
    name: String,
    path: String,
    explicit_overrides: HashMap<String, String>,
}

impl ConfigDisplay {
    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("Configuration source: {}\n\n", self.config_source));

        output.push_str("=== Workspace Defaults ===\n");
        output.push_str("(These apply to all packages unless overridden)\n\n");
        output.push_str(&format_option_fields(&self.workspace_defaults));
        output.push_str("\n");

        output.push_str("=== Workspace-Specific Settings ===\n");
        output.push_str("(These don't apply to individual packages)\n\n");
        output.push_str(&format_workspace_overrides(&self.workspace_overrides));
        output.push_str("\n");

        output.push_str("=== Package Configurations ===\n");
        output.push_str("(Only showing packages with explicit overrides)\n\n");

        let packages_with_overrides: Vec<_> = self.packages.iter()
            .filter(|pkg| !pkg.explicit_overrides.is_empty())
            .collect();

        if packages_with_overrides.is_empty() {
            output.push_str("  No packages have explicit overrides\n");
            output.push_str("  All packages use workspace defaults\n\n");
        } else {
            for pkg in packages_with_overrides {
                output.push_str(&format!("Package: {} ({})\n", pkg.name, pkg.path));
                output.push_str("  Explicit overrides:\n");
                for (key, value) in &pkg.explicit_overrides {
                    output.push_str(&format!("    {}: {}\n", key, value));
                }
                output.push_str("\n");
            }
        }

        output
    }
}

fn format_option_fields(defaults: &WorkspaceDefaultsDisplay) -> String {
    let mut output = String::new();

    if let Some(ref val) = defaults.changelog_path {
        output.push_str(&format!("  changelog_path: {}\n", val));
    }
    if let Some(val) = defaults.changelog_update {
        output.push_str(&format!("  changelog_update: {}\n", val));
    }
    if let Some(val) = defaults.features_always_increment_minor {
        output.push_str(&format!("  features_always_increment_minor: {}\n", val));
    }
    if let Some(val) = defaults.git_release_enable {
        output.push_str(&format!("  git_release_enable: {}\n", val));
    }
    if let Some(ref val) = defaults.git_release_body {
        output.push_str(&format!("  git_release_body: {}\n", val));
    }
    if let Some(ref val) = defaults.git_release_type {
        output.push_str(&format!("  git_release_type: {}\n", val));
    }
    if let Some(val) = defaults.git_release_draft {
        output.push_str(&format!("  git_release_draft: {}\n", val));
    }
    if let Some(val) = defaults.git_release_latest {
        output.push_str(&format!("  git_release_latest: {}\n", val));
    }
    if let Some(ref val) = defaults.git_release_name {
        output.push_str(&format!("  git_release_name: {}\n", val));
    }
    if let Some(val) = defaults.git_tag_enable {
        output.push_str(&format!("  git_tag_enable: {}\n", val));
    }
    if let Some(ref val) = defaults.git_tag_name {
        output.push_str(&format!("  git_tag_name: {}\n", val));
    }
    if let Some(val) = defaults.publish_allow_dirty {
        output.push_str(&format!("  publish_allow_dirty: {}\n", val));
    }
    if let Some(val) = defaults.publish_no_verify {
        output.push_str(&format!("  publish_no_verify: {}\n", val));
    }
    if let Some(ref val) = defaults.publish_features {
        output.push_str(&format!("  publish_features: {:?}\n", val));
    }
    if let Some(val) = defaults.publish_all_features {
        output.push_str(&format!("  publish_all_features: {}\n", val));
    }
    if let Some(val) = defaults.semver_check {
        output.push_str(&format!("  semver_check: {}\n", val));
    }

    if output.is_empty() {
        output.push_str("  (No explicit workspace defaults set)\n");
    }

    output
}

fn format_workspace_overrides(overrides: &WorkspaceOverridesDisplay) -> String {
    let mut output = String::new();

    if let Some(val) = overrides.allow_dirty {
        output.push_str(&format!("  allow_dirty: {}\n", val));
    }
    if let Some(ref val) = overrides.changelog_config {
        output.push_str(&format!("  changelog_config: {}\n", val));
    }
    if let Some(val) = overrides.dependencies_update {
        output.push_str(&format!("  dependencies_update: {}\n", val));
    }
    if let Some(ref val) = overrides.pr_name {
        output.push_str(&format!("  pr_name: {}\n", val));
    }
    if let Some(ref val) = overrides.pr_body {
        output.push_str(&format!("  pr_body: {}\n", val));
    }
    // Only show pr_draft if explicitly set (not default false)
    // Since we can't distinguish explicit false from default false,
    // we'll skip showing boolean defaults that are false
    if !overrides.pr_labels.is_empty() {
        output.push_str(&format!("  pr_labels: {:?}\n", overrides.pr_labels));
    }
    if let Some(ref val) = overrides.pr_branch_prefix {
        output.push_str(&format!("  pr_branch_prefix: {}\n", val));
    }
    if let Some(ref val) = overrides.publish_timeout {
        output.push_str(&format!("  publish_timeout: {}\n", val));
    }
    if let Some(ref val) = overrides.repo_url {
        output.push_str(&format!("  repo_url: {}\n", val));
    }
    if let Some(ref val) = overrides.release_commits {
        output.push_str(&format!("  release_commits: {}\n", val));
    }
    if let Some(val) = overrides.release_always {
        output.push_str(&format!("  release_always: {}\n", val));
    }
    // Don't show max_analyze_commits if it's the default value
    if let Some(val) = overrides.max_analyze_commits {
        if val != 1000 {
            output.push_str(&format!("  max_analyze_commits: {}\n", val));
        }
    }

    if output.is_empty() {
        output.push_str("  (No workspace-specific settings set)\n");
    }

    output
}

pub fn show_config(args: ShowConfig) -> anyhow::Result<()> {
    // Load config - if manifest_path is specified, use it for config too
    let config = if let Some(manifest_path) = args.optional_manifest() {
        args.config.load_from(manifest_path)?
    } else {
        args.config.load()?
    };
    let config_source = determine_config_source(&args);

    // Load workspace metadata to get package info
    let metadata = args.cargo_metadata()?;
    let workspace_packages = cargo_utils::workspace_members(&metadata)?
        .collect::<Vec<_>>();

    // Filter to specific package if requested
    let packages = if let Some(pkg_name) = &args.package {
        workspace_packages
            .into_iter()
            .filter(|p| p.name == *pkg_name)
            .collect()
    } else {
        workspace_packages
    };

    // Build display structure
    let display = build_config_display(&config, &packages, config_source)?;

    // Output
    if let Some(output_type) = args.output {
        match output_type {
            crate::args::OutputType::Json => {
                println!("{}", serde_json::to_string_pretty(&display)?);
            }
        }
    } else {
        println!("{}", display.display());
    }

    Ok(())
}

fn determine_config_source(_args: &ShowConfig) -> String {
    // Configuration is loaded from Cargo.toml metadata
    "Cargo.toml metadata ([workspace.metadata.k-releaser])".to_string()
}

fn build_config_display(
    config: &Config,
    packages: &[Package],
    config_source: String,
) -> anyhow::Result<ConfigDisplay> {
    // Extract workspace defaults
    let workspace_defaults = extract_workspace_defaults(&config.workspace.packages_defaults);

    // Extract workspace-specific settings
    let workspace_overrides = extract_workspace_overrides(&config.workspace);

    // Build package configs
    let package_configs = packages
        .iter()
        .map(|pkg| build_package_config_display(config, pkg))
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(ConfigDisplay {
        config_source,
        workspace_defaults,
        workspace_overrides,
        packages: package_configs,
    })
}

pub(crate) fn extract_workspace_defaults(defaults: &PackageConfig) -> WorkspaceDefaultsDisplay {
    WorkspaceDefaultsDisplay {
        changelog_path: defaults.changelog_path.as_ref().map(|p| p.display().to_string()),
        changelog_update: defaults.changelog_update,
        features_always_increment_minor: defaults.features_always_increment_minor,
        git_release_enable: defaults.git_release_enable,
        git_release_body: defaults.git_release_body.clone(),
        git_release_type: defaults.git_release_type.as_ref().map(|t| format!("{:?}", t)),
        git_release_draft: defaults.git_release_draft,
        git_release_latest: defaults.git_release_latest,
        git_release_name: defaults.git_release_name.clone(),
        git_tag_enable: defaults.git_tag_enable,
        git_tag_name: defaults.git_tag_name.clone(),
        publish_allow_dirty: defaults.publish_allow_dirty,
        publish_no_verify: defaults.publish_no_verify,
        publish_features: defaults.publish_features.clone(),
        publish_all_features: defaults.publish_all_features,
        semver_check: defaults.semver_check,
    }
}

pub(crate) fn extract_workspace_overrides(workspace: &crate::config::Workspace) -> WorkspaceOverridesDisplay {
    WorkspaceOverridesDisplay {
        allow_dirty: workspace.allow_dirty,
        changelog_config: workspace.changelog_config.as_ref().map(|p| p.display().to_string()),
        dependencies_update: workspace.dependencies_update,
        pr_name: workspace.pr_name.clone(),
        pr_body: workspace.pr_body.clone(),
        pr_draft: workspace.pr_draft,
        pr_labels: workspace.pr_labels.clone(),
        pr_branch_prefix: workspace.pr_branch_prefix.clone(),
        publish_timeout: workspace.publish_timeout.clone(),
        repo_url: workspace.repo_url.as_ref().map(|u| u.to_string()),
        release_commits: workspace.release_commits.clone(),
        release_always: workspace.release_always,
        max_analyze_commits: workspace.max_analyze_commits,
    }
}

fn build_package_config_display(
    config: &Config,
    package: &Package,
) -> anyhow::Result<PackageConfigDisplay> {
    let package_configs = config.packages();
    let package_config = package_configs.get(package.name.as_str());

    // Determine which fields are explicitly overridden
    let explicit_overrides = if let Some(pkg_cfg) = package_config {
        extract_explicit_overrides(pkg_cfg.common())
    } else {
        HashMap::new()
    };

    Ok(PackageConfigDisplay {
        name: package.name.to_string(),
        path: package.manifest_path.parent()
            .map(|p| p.to_string())
            .unwrap_or_else(|| ".".to_string()),
        explicit_overrides,
    })
}

pub(crate) fn extract_explicit_overrides(config: &PackageConfig) -> HashMap<String, String> {
    let mut overrides = HashMap::new();

    if let Some(ref val) = config.changelog_path {
        overrides.insert("changelog_path".to_string(), val.display().to_string());
    }
    if let Some(val) = config.changelog_update {
        overrides.insert("changelog_update".to_string(), val.to_string());
    }
    if let Some(val) = config.features_always_increment_minor {
        overrides.insert("features_always_increment_minor".to_string(), val.to_string());
    }
    if let Some(val) = config.git_release_enable {
        overrides.insert("git_release_enable".to_string(), val.to_string());
    }
    if let Some(ref val) = config.git_release_body {
        overrides.insert("git_release_body".to_string(), val.clone());
    }
    if let Some(ref val) = config.git_release_type {
        overrides.insert("git_release_type".to_string(), format!("{:?}", val));
    }
    if let Some(val) = config.git_release_draft {
        overrides.insert("git_release_draft".to_string(), val.to_string());
    }
    if let Some(val) = config.git_release_latest {
        overrides.insert("git_release_latest".to_string(), val.to_string());
    }
    if let Some(ref val) = config.git_release_name {
        overrides.insert("git_release_name".to_string(), val.clone());
    }
    if let Some(val) = config.git_tag_enable {
        overrides.insert("git_tag_enable".to_string(), val.to_string());
    }
    if let Some(ref val) = config.git_tag_name {
        overrides.insert("git_tag_name".to_string(), val.clone());
    }
    if let Some(val) = config.publish_allow_dirty {
        overrides.insert("publish_allow_dirty".to_string(), val.to_string());
    }
    if let Some(val) = config.publish_no_verify {
        overrides.insert("publish_no_verify".to_string(), val.to_string());
    }
    if let Some(ref val) = config.publish_features {
        overrides.insert("publish_features".to_string(), format!("{:?}", val));
    }
    if let Some(val) = config.publish_all_features {
        overrides.insert("publish_all_features".to_string(), val.to_string());
    }
    if let Some(val) = config.semver_check {
        overrides.insert("semver_check".to_string(), val.to_string());
    }

    overrides
}

#[cfg(test)]
#[path = "config_show_test.rs"]
mod tests;
