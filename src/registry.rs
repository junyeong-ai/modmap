use semver::Version;
use thiserror::Error;

use crate::manifest::ProjectManifest;
use crate::module_map::SCHEMA_VERSION;

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Version parse error: {0}")]
    VersionParse(#[from] semver::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Incompatible schema version: found {found}, required major version {required_major}")]
    IncompatibleVersion { found: String, required_major: u64 },
}

pub struct SchemaRegistry {
    current_version: Version,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            current_version: Version::parse(SCHEMA_VERSION)
                .expect("SCHEMA_VERSION must be valid semver"),
        }
    }

    pub fn load(&self, data: &str) -> Result<ProjectManifest, SchemaError> {
        let manifest: ProjectManifest = serde_json::from_str(data)?;
        self.validate_project_version(&manifest)?;
        Ok(manifest)
    }

    fn validate_project_version(&self, manifest: &ProjectManifest) -> Result<(), SchemaError> {
        let version = Version::parse(&manifest.project.schema_version)?;
        if version.major != self.current_version.major {
            return Err(SchemaError::IncompatibleVersion {
                found: manifest.project.schema_version.clone(),
                required_major: self.current_version.major,
            });
        }
        Ok(())
    }

    pub fn version(&self) -> &Version {
        &self.current_version
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest_json(schema_version: &str) -> String {
        format!(
            r#"{{
                "version": "2.0.0",
                "created_at": "2026-01-29T00:00:00Z",
                "generator": "claudegen",
                "project": {{
                    "schema_version": "{}",
                    "generator": {{"name": "test", "version": "1.0.0"}},
                    "project": {{
                        "name": "test",
                        "workspace": {{}},
                        "tech_stack": {{"primary_language": "rust"}},
                        "languages": [],
                        "total_files": 0
                    }},
                    "modules": [],
                    "generated_at": "2026-01-29T00:00:00Z"
                }}
            }}"#,
            schema_version
        )
    }

    #[test]
    fn test_registry_creation() {
        let registry = SchemaRegistry::new();
        assert_eq!(registry.version().major, 1);
    }

    #[test]
    fn test_load_valid_manifest() {
        let registry = SchemaRegistry::new();
        let json = sample_manifest_json("1.0.0");
        let result = registry.load(&json);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().project.project.name, "test");
    }

    #[test]
    fn test_load_compatible_minor_version() {
        let registry = SchemaRegistry::new();
        let json = sample_manifest_json("1.5.0");
        assert!(registry.load(&json).is_ok());
    }

    #[test]
    fn test_load_incompatible_major_version() {
        let registry = SchemaRegistry::new();
        let json = sample_manifest_json("2.0.0");
        assert!(matches!(
            registry.load(&json),
            Err(SchemaError::IncompatibleVersion { .. })
        ));
    }

    #[test]
    fn test_schema_version_constant() {
        Version::parse(SCHEMA_VERSION).expect("SCHEMA_VERSION must be valid semver");
    }
}
