use semver::Version;
use thiserror::Error;

use crate::module_map::{ModuleMap, SCHEMA_VERSION};

#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("Version parse error: {0}")]
    VersionParse(#[from] semver::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Missing schema_version field")]
    MissingVersion,

    #[error("Incompatible version: found {found}, minimum required {minimum}")]
    IncompatibleVersion { found: String, minimum: String },
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

    pub fn load_module_map(&self, data: &str) -> Result<ModuleMap, SchemaError> {
        let value: serde_json::Value = serde_json::from_str(data)?;
        self.validate_version(&value)?;
        Ok(serde_json::from_value(value)?)
    }

    fn validate_version(&self, value: &serde_json::Value) -> Result<(), SchemaError> {
        let version_str = value
            .get("schema_version")
            .and_then(|v| v.as_str())
            .ok_or(SchemaError::MissingVersion)?;

        let version = Version::parse(version_str)?;

        if version.major != self.current_version.major {
            return Err(SchemaError::IncompatibleVersion {
                found: version_str.to_string(),
                minimum: SCHEMA_VERSION.to_string(),
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

    #[test]
    fn test_registry_creation() {
        let registry = SchemaRegistry::new();
        assert_eq!(registry.version().major, 1);
    }

    #[test]
    fn test_version_validation_success() {
        let registry = SchemaRegistry::new();
        let data = serde_json::json!({ "schema_version": "1.0.0" });
        assert!(registry.validate_version(&data).is_ok());
    }

    #[test]
    fn test_version_validation_minor_difference() {
        let registry = SchemaRegistry::new();
        let data = serde_json::json!({ "schema_version": "1.1.0" });
        assert!(registry.validate_version(&data).is_ok());
    }

    #[test]
    fn test_version_validation_major_mismatch() {
        let registry = SchemaRegistry::new();
        let data = serde_json::json!({ "schema_version": "2.0.0" });
        assert!(matches!(
            registry.validate_version(&data),
            Err(SchemaError::IncompatibleVersion { .. })
        ));
    }

    #[test]
    fn test_missing_version() {
        let registry = SchemaRegistry::new();
        let data = serde_json::json!({});
        assert!(matches!(
            registry.validate_version(&data),
            Err(SchemaError::MissingVersion)
        ));
    }

    #[test]
    fn test_schema_version_constant() {
        Version::parse(SCHEMA_VERSION).expect("SCHEMA_VERSION must be valid semver");
    }
}
