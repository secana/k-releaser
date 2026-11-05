use cargo_metadata::{Dependency, DependencyKind};

#[derive(Clone, Debug)]
pub struct FakeDependency {
    name: String,
    kind: DependencyKind,
    req: String,
}

impl FakeDependency {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: DependencyKind::Normal,
            req: "0.1.0".to_string(),
        }
    }

    pub fn dev(self) -> Self {
        Self {
            kind: DependencyKind::Development,
            ..self
        }
    }

    /// A dependency without a version requirement, like a path only dependency.
    pub fn path_only(self) -> Self {
        Self {
            req: "*".to_string(),
            ..self
        }
    }
}

impl From<FakeDependency> for Dependency {
    fn from(dep: FakeDependency) -> Self {
        serde_json::from_value(serde_json::json!({
            "name": dep.name,
            "req": dep.req,
            "kind": dep.kind,
            "optional": false,
            "uses_default_features": true,
            "features": [],
        }))
        .unwrap()
    }
}
