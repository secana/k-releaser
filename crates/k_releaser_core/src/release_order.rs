use cargo_metadata::{Dependency, DependencyKind, Package, semver::VersionReq};
use tracing::{debug, warn};

/// Return packages in an order they can be released.
/// In the result, the packages are placed after all their dependencies.
/// Return an error if a circular dependency is detected.
///
/// Dev-dependencies with a version requirement are placed before the package
/// too, because `cargo publish` keeps them in the published manifest and
/// resolves them from the registry. If that leads to a cycle, they are
/// ignored: an already published version may still satisfy them.
pub fn release_order<'a>(packages: &'a [&Package]) -> anyhow::Result<Vec<&'a Package>> {
    let order = match release_order_with(packages, DevDeps::Versioned) {
        Ok(order) => order,
        Err(e) => {
            warn!("ignoring dev-dependencies for the release order: {e}");
            release_order_with(packages, DevDeps::Ignored)?
        }
    };
    debug!(
        "Release order: {:?}",
        order.iter().map(|p| &p.name).collect::<Vec<_>>()
    );
    Ok(order)
}

/// Which dev-dependencies must be released before the package.
#[derive(Clone, Copy)]
enum DevDeps {
    /// Dev-dependencies with a version requirement.
    Versioned,
    /// None, unless they are enabled in features.
    Ignored,
}

fn release_order_with<'a>(
    packages: &'a [&Package],
    dev_deps: DevDeps,
) -> anyhow::Result<Vec<&'a Package>> {
    let mut order = vec![];
    let mut passed = vec![];
    for p in packages {
        release_order_inner(packages, p, dev_deps, &mut order, &mut passed)?;
    }
    Ok(order)
}

/// The `passed` argument is used to track packages that you already visited to
/// detect circular dependencies.
fn release_order_inner<'a>(
    packages: &[&'a Package],
    pkg: &'a Package,
    dev_deps: DevDeps,
    order: &mut Vec<&'a Package>,
    passed: &mut Vec<&'a Package>,
) -> anyhow::Result<()> {
    if is_package_in(pkg, order) {
        return Ok(());
    }
    passed.push(pkg);

    for d in &pkg.dependencies {
        // Check if the dependency is part of the packages we are releasing.
        if let Some(dep) = packages.iter().find(|p| {
            d.name == *p.name
              // Exclude the current package.
              && p.name != pkg.name
              && should_dep_be_released_before(d, pkg, dev_deps)
        }) {
            anyhow::ensure!(
                !is_package_in(dep, passed),
                "Circular dependency detected: {} -> {}",
                dep.name,
                pkg.name,
            );
            release_order_inner(packages, dep, dev_deps, order, passed)?;
        }
    }

    order.push(pkg);
    passed.clear();
    Ok(())
}

/// Return true if the package is part of a packages array.
/// This function exists because `package.contains(pkg)` is expensive,
/// because it compares the whole package struct.
fn is_package_in(pkg: &Package, packages: &[&Package]) -> bool {
    packages.iter().any(|p| p.name == pkg.name)
}

/// Check if the dependency is enabled in features.
fn is_dep_in_features(pkg: &Package, dep: &str) -> bool {
    pkg.features
        // Discard features name.
        .values()
        // Any feature contains the dependency in the format `dep/feature`.
        .any(|enabled_features| {
            enabled_features
                .iter()
                .filter_map(|feature| feature.split_once('/').map(|split| split.0))
                .any(|enabled_dependency| enabled_dependency == dep)
        })
}

/// Check if the dependency should be released before the current package.
fn should_dep_be_released_before(dep: &Dependency, pkg: &Package, dev_deps: DevDeps) -> bool {
    match dep.kind {
        DependencyKind::Normal | DependencyKind::Build => true,
        // `cargo publish` drops dev-dependencies without a version (path only)
        // from the published manifest, but has to resolve the others from the registry.
        DependencyKind::Development
            if matches!(dev_deps, DevDeps::Versioned) && dep.req != VersionReq::STAR =>
        {
            true
        }
        // `cargo publish` compiles crates that are in features and dev-dependencies,
        // even if they are not present in normal dependencies.
        _ => is_dep_in_features(pkg, &dep.name),
    }
}

#[cfg(test)]
mod tests {
    use fake_package::{FakeDependency, FakePackage};

    use super::*;
    use crate::publishable_packages_from_manifest;

    // Test the package release order in the release-plz workspace itself.
    #[test]
    fn workspace_release_order_is_correct() {
        let public_packages = publishable_packages_from_manifest("../../Cargo.toml").unwrap();
        let pkgs = &public_packages.iter().collect::<Vec<_>>();
        assert_eq!(
            order(pkgs),
            [
                "cargo_utils",
                "git_cmd",
                "test_logs",
                "next_version",
                "k_releaser_core",
                "k-releaser"
            ]
        );
    }

    /// Package
    fn pkg(name: &str, deps: &[FakeDependency]) -> Package {
        FakePackage::new(name)
            .with_dependencies(deps.to_vec())
            .into()
    }

    /// Dependency
    fn dep(name: &str) -> FakeDependency {
        FakeDependency::new(name)
    }

    /// Development dependency with a version requirement
    fn dev_dep(name: &str) -> FakeDependency {
        FakeDependency::new(name).dev()
    }

    /// Development dependency without a version (path only)
    fn path_dev_dep(name: &str) -> FakeDependency {
        FakeDependency::new(name).dev().path_only()
    }

    fn order<'a>(pkgs: &'a [&'a Package]) -> Vec<&'a str> {
        release_order(pkgs)
            .unwrap()
            .iter()
            .map(|p| p.name.as_str())
            .collect()
    }

    // Diagrams created with https://asciiflow.com/

    /// A─►B
    #[test]
    fn single_package_is_returned() {
        let pkgs = [&pkg("a", &[dep("b")])];
        assert_eq!(order(&pkgs), ["a"]);
    }

    /// ┌──┐
    /// │  ▼
    /// A  B
    /// ▲  │
    /// └──┘
    #[test]
    fn two_packages_cycle_is_detected() {
        let pkgs = [&pkg("a", &[dep("b")]), &pkg("b", &[dep("a")])];
        expect_test::expect!["Circular dependency detected: a -> b"]
            .assert_eq(&release_order(&pkgs).unwrap_err().to_string());
    }

    /// ┌──┐
    /// │  ▼
    /// A  B (dev dependency)
    /// ▲  │
    /// └──┘
    #[test]
    fn two_packages_dev_cycle_is_ok() {
        let pkgs = [&pkg("a", &[dev_dep("b")]), &pkg("b", &[dep("a")])];
        assert_eq!(order(&pkgs), ["a", "b"]);

        // check if the order of the vector matters.
        let pkgs = [&pkg("b", &[dep("a")]), &pkg("a", &[dev_dep("b")])];
        assert_eq!(order(&pkgs), ["a", "b"]);
    }

    /// ┌─────┐
    /// ▼     │
    /// A────►B
    /// │     ▲
    /// └─►C──┘
    #[test]
    fn three_packages_cycle_is_detected() {
        let pkgs = [
            &pkg("a", &[dep("b")]),
            &pkg("a", &[dep("c")]),
            &pkg("b", &[dep("a")]),
            &pkg("c", &[dep("b")]),
        ];
        expect_test::expect!["Circular dependency detected: a -> b"]
            .assert_eq(&release_order(&pkgs).unwrap_err().to_string());
    }

    /// A────►C
    /// │     ▲
    /// └─►B──┘
    #[test]
    fn three_packages_are_ordered() {
        let pkgs = [
            &pkg("a", &[dep("b")]),
            &pkg("b", &[dep("c")]),
            &pkg("c", &[]),
        ];
        assert_eq!(order(&pkgs), ["c", "b", "a"]);
    }

    /// ┌──┐
    /// │  ▼
    /// A  B (dev dependency)
    /// ▲  │
    /// └──┘
    #[test]
    fn two_packages_dev_cycle_with_package_in_features_is_detected() {
        let mut a = pkg("a", &[dev_dep("b")]);
        a.features = [("my_feat".to_string(), vec!["b/feat".to_string()])].into();
        let pkgs = [&a, &pkg("b", &[dep("a")])];
        expect_test::expect!["Circular dependency detected: a -> b"]
            .assert_eq(&release_order(&pkgs).unwrap_err().to_string());
    }

    /// ┌──┐
    /// │  ▼
    /// A  B (dev dependency)
    /// ▲  │
    /// └──┘
    #[test]
    fn two_packages_dev_cycle_with_random_feature_is_ok() {
        let mut a = pkg("a", &[dev_dep("b")]);
        a.features = [(
            "my_feat".to_string(),
            vec!["b".to_string(), "rand/b".to_string()],
        )]
        .into();
        let pkgs = [&a, &pkg("b", &[dep("a")])];
        assert_eq!(order(&pkgs), ["a", "b"]);
    }

    /// A──►B (versioned dev dependency)
    #[test]
    fn versioned_dev_dependency_is_released_first() {
        let pkgs = [&pkg("a", &[dev_dep("b")]), &pkg("b", &[])];
        assert_eq!(order(&pkgs), ["b", "a"]);
    }

    /// A──►B (path only dev dependency)
    #[test]
    fn path_only_dev_dependency_is_ignored() {
        let pkgs = [&pkg("a", &[path_dev_dep("b")]), &pkg("b", &[])];
        assert_eq!(order(&pkgs), ["a", "b"]);
    }

    /// A──►B (versioned dev dependency)
    /// │
    /// └─►C──►B (path only dev dependency)
    #[test]
    fn mixed_dev_dependencies_are_ordered() {
        let pkgs = [
            &pkg("a", &[dep("c"), dev_dep("b")]),
            &pkg("c", &[path_dev_dep("b")]),
            &pkg("b", &[dep("c")]),
        ];
        assert_eq!(order(&pkgs), ["c", "b", "a"]);
    }
}
