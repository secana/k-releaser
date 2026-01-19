use cargo_metadata::camino::Utf8Path;
use git_cmd::git_in_dir;

pub fn add_feature(project: &Utf8Path, message: &str) {
    fs_err::write(project.join("src").join("lib.rs"), "fn hello(){}").unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    let commit_message = format!("feat: {message}");
    git_in_dir(project, &["commit", "-m", &commit_message]).unwrap();
}

pub fn add_chore(project: &Utf8Path, message: &str) {
    // Create a small change to allow commit
    let lib_path = project.join("src").join("lib.rs");
    let content = fs_err::read_to_string(&lib_path).unwrap_or_default();
    fs_err::write(&lib_path, format!("{content}\n// {message}")).unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    let commit_message = format!("chore: {message}");
    git_in_dir(project, &["commit", "-m", &commit_message]).unwrap();
}

pub fn add_ci(project: &Utf8Path, message: &str) {
    // Create a small change to allow commit
    let lib_path = project.join("src").join("lib.rs");
    let content = fs_err::read_to_string(&lib_path).unwrap_or_default();
    fs_err::write(&lib_path, format!("{content}\n// ci: {message}")).unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    let commit_message = format!("ci: {message}");
    git_in_dir(project, &["commit", "-m", &commit_message]).unwrap();
}

pub fn create_tag(project: &Utf8Path, tag: &str) {
    git_in_dir(project, &["tag", "-m", tag, tag]).unwrap();
}

/// Creates a merge commit scenario similar to what happens when someone does `git pull`
/// instead of `git pull --rebase`. This creates a branch, adds commits to both branches,
/// and then merges them.
///
/// The scenario replicates the kellnr issue where a `feat:` commit on the second parent
/// of a merge commit was being skipped by `--first-parent`.
///
/// Structure:
/// ```text
/// *   Merge branch 'feature-branch'
/// |\
/// | * feat: feature on branch (THIS SHOULD BE DETECTED)
/// * | chore: update version (this is filtered as release commit)
/// |/
/// * ci: some ci change (base commit after tag)
/// * v0.1.0 (tag)
/// ```
pub fn create_merge_with_feature_on_second_parent(project: &Utf8Path) {
    // Get the current branch name (might be "main", "master", or something else)
    let current_branch = git_in_dir(project, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap();
    let current_branch = current_branch.trim();

    // We're on the main branch after the tag, add a ci commit using a separate file
    fs_err::write(project.join("ci_change.txt"), "ci change").unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    git_in_dir(project, &["commit", "-m", "ci: some ci change"]).unwrap();

    // Create a feature branch from the current commit
    git_in_dir(project, &["checkout", "-b", "feature-branch"]).unwrap();

    // Add a feature commit on the branch (use a separate file to avoid conflicts)
    fs_err::write(project.join("feature.txt"), "new feature").unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    git_in_dir(project, &["commit", "-m", "feat: improved UI"]).unwrap();

    // Go back to the original branch
    git_in_dir(project, &["checkout", current_branch]).unwrap();

    // Add a chore commit on main (simulating a release PR commit that should be filtered)
    // Use a separate file to avoid conflicts
    fs_err::write(project.join("version.txt"), "version update").unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    git_in_dir(project, &["commit", "-m", "chore: update version in Cargo.toml"]).unwrap();

    // Merge the feature branch (creates a merge commit like `git pull` would)
    git_in_dir(
        project,
        &["merge", "feature-branch", "-m", "Merge branch 'feature-branch'"],
    )
    .unwrap();

    // Add another chore commit after the merge (simulating workspace version update)
    fs_err::write(project.join("workspace.txt"), "workspace versions").unwrap();
    git_in_dir(project, &["add", "."]).unwrap();
    git_in_dir(project, &["commit", "-m", "chore: update workspace versions"]).unwrap();
}
