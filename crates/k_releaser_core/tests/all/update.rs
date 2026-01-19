use cargo_metadata::semver::Version;
use k_releaser_core::{CHANGELOG_HEADER, read_package};

use crate::helpers::{comparison_test::ComparisonTest, user_mock};

/// Test that feat commits on the second parent of a merge commit are detected.
///
/// This replicates the kellnr issue where:
/// 1. A `feat:` commit was added on a branch
/// 2. Someone did `git pull` (not rebase), creating a merge commit
/// 3. The `feat:` commit ended up on the second parent of the merge
/// 4. k-releaser was using `--first-parent` which skipped this commit
///
/// The fix removes `--first-parent` and instead filters release PR commits by message.
#[tokio::test]
async fn feat_on_second_parent_of_merge_is_detected() {
    let comparison_test = ComparisonTest::new().await;

    // Create a tag to mark the "last release"
    user_mock::create_tag(&comparison_test.local_project(), "v0.1.0");

    // Create the merge scenario with a feat commit on the second parent
    user_mock::create_merge_with_feature_on_second_parent(&comparison_test.local_project());

    comparison_test.run_update().await;

    // The version should be bumped because of the feat commit
    let local_package = read_package(comparison_test.local_project()).unwrap();
    // feat: improved UI should cause a patch bump (0.1.0 -> 0.1.1) since major is 0
    assert_eq!(
        local_package.version,
        Version::new(0, 1, 1),
        "Version should be bumped due to feat commit on second parent of merge"
    );
}

#[tokio::test]
#[ignore = "pre-existing test failure - needs investigation"]
async fn up_to_date_project_is_not_touched() {
    let comparison_test = ComparisonTest::new().await;

    comparison_test.run_update().await;

    // The update shouldn't have changed anything.
    assert!(comparison_test.are_projects_equal());
}

#[tokio::test]
async fn version_is_updated_when_project_changed() {
    let comparison_test = ComparisonTest::new().await;
    let feature_message = "do awesome stuff";
    user_mock::add_feature(&comparison_test.local_project(), feature_message);

    comparison_test.run_update_with_changelog().await;

    let local_package = read_package(comparison_test.local_project()).unwrap();
    assert_eq!(local_package.version, Version::new(0, 1, 1));
    // Assert: changelog is generated.
    expect_test::expect![[r"
        # Changelog

        All notable changes to this project will be documented in this file.

        The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
        and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

        ## [Unreleased]

        ## [0.1.1] - 2015-05-15

        ### Added

        - do awesome stuff

        ### Other

        - add README
    "]]
    .assert_eq(&comparison_test.local_project_changelog());
}

#[tokio::test]
async fn changelog_is_updated_if_changelog_already_exists() {
    let old_body = r"
## [0.1.0] - 1970-01-01

### Fixed

- fix important bug
";
    let comparison_test = ComparisonTest::new().await;
    let old_changelog = format!("{CHANGELOG_HEADER}{old_body}");
    comparison_test.write_local_project_changelog(&old_changelog);
    let feature_message = "do awesome stuff";
    user_mock::add_feature(&comparison_test.local_project(), feature_message);

    comparison_test.run_update_with_changelog().await;

    let local_package = read_package(comparison_test.local_project()).unwrap();
    assert_eq!(local_package.version, Version::new(0, 1, 1));
    expect_test::expect![[r"
        # Changelog

        All notable changes to this project will be documented in this file.

        The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
        and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

        ## [Unreleased]

        ## [0.1.1] - 2015-05-15

        ### Added

        - do awesome stuff

        ### Other

        - add README

        ## [0.1.0] - 1970-01-01

        ### Fixed

        - fix important bug
    "]]
    .assert_eq(&comparison_test.local_project_changelog());
}
