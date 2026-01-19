use crate::config::{PackageConfig, Workspace};
use crate::config_show::{
    extract_explicit_overrides, extract_workspace_defaults, extract_workspace_overrides,
};

#[test]
fn extract_workspace_defaults_with_all_fields() {
    let mut defaults = PackageConfig::default();
    defaults.changelog_path = Some("CHANGELOG.md".into());
    defaults.changelog_update = Some(true);
    defaults.features_always_increment_minor = Some(true);
    defaults.git_release_enable = Some(true);
    defaults.git_release_body = Some("{{ changelog }}".to_string());
    defaults.git_release_draft = Some(true);
    defaults.git_release_latest = Some(false);
    defaults.git_release_name = Some("Release {{ version }}".to_string());
    defaults.git_tag_enable = Some(true);
    defaults.git_tag_name = Some("v{{ version }}".to_string());
    defaults.publish_allow_dirty = Some(true);
    defaults.publish_no_verify = Some(true);
    defaults.publish_features = Some(vec!["feature1".to_string()]);
    defaults.publish_all_features = Some(true);
    defaults.semver_check = Some(false);

    let display = extract_workspace_defaults(&defaults);

    assert_eq!(display.changelog_path, Some("CHANGELOG.md".to_string()));
    assert_eq!(display.changelog_update, Some(true));
    assert_eq!(display.features_always_increment_minor, Some(true));
    assert_eq!(display.git_release_enable, Some(true));
    assert_eq!(display.git_release_body, Some("{{ changelog }}".to_string()));
    assert_eq!(display.git_release_draft, Some(true));
    assert_eq!(display.git_release_latest, Some(false));
    assert_eq!(
        display.git_release_name,
        Some("Release {{ version }}".to_string())
    );
    assert_eq!(display.git_tag_enable, Some(true));
    assert_eq!(display.git_tag_name, Some("v{{ version }}".to_string()));
    assert_eq!(display.publish_allow_dirty, Some(true));
    assert_eq!(display.publish_no_verify, Some(true));
    assert_eq!(
        display.publish_features,
        Some(vec!["feature1".to_string()])
    );
    assert_eq!(display.publish_all_features, Some(true));
    assert_eq!(display.semver_check, Some(false));
}

#[test]
fn extract_workspace_defaults_with_none_fields() {
    let defaults = PackageConfig::default();

    let display = extract_workspace_defaults(&defaults);

    assert_eq!(display.changelog_path, None);
    assert_eq!(display.changelog_update, None);
    assert_eq!(display.features_always_increment_minor, None);
    assert_eq!(display.git_release_enable, None);
    assert_eq!(display.git_release_body, None);
    assert_eq!(display.git_release_draft, None);
    assert_eq!(display.git_release_latest, None);
    assert_eq!(display.git_release_name, None);
    assert_eq!(display.git_tag_enable, None);
    assert_eq!(display.git_tag_name, None);
    assert_eq!(display.publish_allow_dirty, None);
    assert_eq!(display.publish_no_verify, None);
    assert_eq!(display.publish_features, None);
    assert_eq!(display.publish_all_features, None);
    assert_eq!(display.semver_check, None);
}

#[test]
fn extract_workspace_overrides_with_all_fields() {
    let workspace = Workspace {
        allow_dirty: Some(true),
        changelog_config: Some("cliff.toml".into()),
        dependencies_update: Some(true),
        pr_name: Some("Release PR".to_string()),
        pr_body: Some("Release body".to_string()),
        pr_draft: true,
        pr_labels: vec!["release".to_string()],
        pr_branch_prefix: Some("release-".to_string()),
        publish_timeout: Some("30m".to_string()),
        repo_url: Some("https://github.com/user/repo".parse().unwrap()),
        release_commits: Some("^feat:".to_string()),
        release_always: Some(true),
        max_analyze_commits: Some(2000),
        packages_defaults: PackageConfig::default(),
    };

    let display = extract_workspace_overrides(&workspace);

    assert_eq!(display.allow_dirty, Some(true));
    assert_eq!(display.changelog_config, Some("cliff.toml".to_string()));
    assert_eq!(display.dependencies_update, Some(true));
    assert_eq!(display.pr_name, Some("Release PR".to_string()));
    assert_eq!(display.pr_body, Some("Release body".to_string()));
    assert_eq!(display.pr_draft, true);
    assert_eq!(display.pr_labels, vec!["release".to_string()]);
    assert_eq!(display.pr_branch_prefix, Some("release-".to_string()));
    assert_eq!(display.publish_timeout, Some("30m".to_string()));
    assert_eq!(
        display.repo_url,
        Some("https://github.com/user/repo".to_string())
    );
    assert_eq!(display.release_commits, Some("^feat:".to_string()));
    assert_eq!(display.release_always, Some(true));
    assert_eq!(display.max_analyze_commits, Some(2000));
}

#[test]
fn extract_explicit_overrides_with_all_fields() {
    let mut config = PackageConfig::default();
    config.changelog_path = Some("CHANGELOG.md".into());
    config.changelog_update = Some(true);
    config.features_always_increment_minor = Some(true);
    config.git_release_enable = Some(true);
    config.git_release_body = Some("{{ changelog }}".to_string());
    config.git_release_draft = Some(true);
    config.git_release_latest = Some(false);
    config.git_release_name = Some("Release {{ version }}".to_string());
    config.git_tag_enable = Some(true);
    config.git_tag_name = Some("v{{ version }}".to_string());
    config.publish_allow_dirty = Some(true);
    config.publish_no_verify = Some(true);
    config.publish_features = Some(vec!["feature1".to_string()]);
    config.publish_all_features = Some(true);
    config.semver_check = Some(false);

    let overrides = extract_explicit_overrides(&config);

    assert_eq!(overrides.get("changelog_path"), Some(&"CHANGELOG.md".to_string()));
    assert_eq!(overrides.get("changelog_update"), Some(&"true".to_string()));
    assert_eq!(
        overrides.get("features_always_increment_minor"),
        Some(&"true".to_string())
    );
    assert_eq!(overrides.get("git_release_enable"), Some(&"true".to_string()));
    assert_eq!(
        overrides.get("git_release_body"),
        Some(&"{{ changelog }}".to_string())
    );
    assert_eq!(overrides.get("git_release_draft"), Some(&"true".to_string()));
    assert_eq!(overrides.get("git_release_latest"), Some(&"false".to_string()));
    assert_eq!(
        overrides.get("git_release_name"),
        Some(&"Release {{ version }}".to_string())
    );
    assert_eq!(overrides.get("git_tag_enable"), Some(&"true".to_string()));
    assert_eq!(
        overrides.get("git_tag_name"),
        Some(&"v{{ version }}".to_string())
    );
    assert_eq!(
        overrides.get("publish_allow_dirty"),
        Some(&"true".to_string())
    );
    assert_eq!(overrides.get("publish_no_verify"), Some(&"true".to_string()));
    assert_eq!(
        overrides.get("publish_features"),
        Some(&"[\"feature1\"]".to_string())
    );
    assert_eq!(
        overrides.get("publish_all_features"),
        Some(&"true".to_string())
    );
    assert_eq!(overrides.get("semver_check"), Some(&"false".to_string()));
}

#[test]
fn extract_explicit_overrides_with_no_fields() {
    let config = PackageConfig::default();

    let overrides = extract_explicit_overrides(&config);

    assert!(overrides.is_empty());
}
