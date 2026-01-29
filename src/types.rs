use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceType {
    #[default]
    SinglePackage,
    Monorepo,
    Microservices,
    MultiPackage,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    #[default]
    Application,
    Library,
    Service,
    Cli,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    #[default]
    Runtime,
    Build,
    Test,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
pub struct ModuleDependency {
    pub module_id: String,
    #[serde(default)]
    pub dependency_type: DependencyType,
}

impl ModuleDependency {
    pub fn new(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            dependency_type: DependencyType::default(),
        }
    }

    pub fn runtime(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            dependency_type: DependencyType::Runtime,
        }
    }

    pub fn build(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            dependency_type: DependencyType::Build,
        }
    }

    pub fn test(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            dependency_type: DependencyType::Test,
        }
    }

    pub fn optional(module_id: impl Into<String>) -> Self {
        Self {
            module_id: module_id.into(),
            dependency_type: DependencyType::Optional,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(default)]
pub struct EvidenceLocation {
    pub file: String,
    pub start_line: u32,
    pub end_line: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
}

impl EvidenceLocation {
    pub fn new(file: impl Into<String>, line: u32) -> Self {
        Self {
            file: file.into(),
            start_line: line,
            end_line: line,
            start_column: None,
            end_column: None,
        }
    }

    pub fn new_range(file: impl Into<String>, start_line: u32, end_line: u32) -> Self {
        Self {
            file: file.into(),
            start_line,
            end_line,
            start_column: None,
            end_column: None,
        }
    }

    pub fn to_reference(&self) -> String {
        if self.end_line != self.start_line && self.end_line > 0 {
            format!("{}:{}-{}", self.file, self.start_line, self.end_line)
        } else {
            format!("{}:{}", self.file, self.start_line)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeneratorInfo {
    pub name: String,
    pub version: String,
}

impl GeneratorInfo {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueCategory {
    Security,
    Performance,
    Correctness,
    Maintainability,
    Concurrency,
    Compatibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Convention {
    pub name: String,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<EvidenceLocation>,
}

impl Convention {
    pub fn new(name: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            pattern: pattern.into(),
            rationale: None,
            evidence: Vec::new(),
        }
    }

    pub fn with_rationale(mut self, rationale: impl Into<String>) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    pub fn with_evidence(mut self, evidence: Vec<EvidenceLocation>) -> Self {
        self.evidence = evidence;
        self
    }
}

impl fmt::Display for Convention {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.pattern)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct KnownIssue {
    pub id: String,
    pub description: String,
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prevention: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<EvidenceLocation>,
}

impl KnownIssue {
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        severity: IssueSeverity,
        category: IssueCategory,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            severity,
            category,
            prevention: None,
            evidence: Vec::new(),
        }
    }

    pub fn with_prevention(mut self, prevention: impl Into<String>) -> Self {
        self.prevention = Some(prevention.into());
        self
    }

    pub fn with_evidence(mut self, evidence: Vec<EvidenceLocation>) -> Self {
        self.evidence = evidence;
        self
    }
}

impl fmt::Display for KnownIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.severity, self.id, self.description)
    }
}

impl fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueSeverity::Critical => write!(f, "CRITICAL"),
            IssueSeverity::High => write!(f, "HIGH"),
            IssueSeverity::Medium => write!(f, "MEDIUM"),
            IssueSeverity::Low => write!(f, "LOW"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct TechStack {
    pub primary_language: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameworks: Vec<FrameworkInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub build_tools: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_frameworks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_libraries: Vec<LibraryInfo>,
}

impl TechStack {
    pub fn new(primary_language: impl Into<String>) -> Self {
        Self {
            primary_language: primary_language.into(),
            ..Default::default()
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.language_version = Some(version.into());
        self
    }

    pub fn with_framework(mut self, framework: FrameworkInfo) -> Self {
        self.frameworks.push(framework);
        self
    }

    pub fn with_build_tool(mut self, tool: impl Into<String>) -> Self {
        self.build_tools.push(tool.into());
        self
    }

    pub fn with_test_framework(mut self, framework: impl Into<String>) -> Self {
        self.test_frameworks.push(framework.into());
        self
    }

    pub fn with_library(mut self, library: LibraryInfo) -> Self {
        self.key_libraries.push(library);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FrameworkInfo {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub purpose: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}

impl FrameworkInfo {
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            purpose: purpose.into(),
            paths: Vec::new(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths = paths;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LibraryInfo {
    pub name: String,
    pub purpose: String,
}

impl LibraryInfo {
    pub fn new(name: impl Into<String>, purpose: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            purpose: purpose.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DetectedLanguage {
    pub name: String,
    #[serde(default)]
    pub percentage: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frameworks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub build_tools: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marker_files: Vec<String>,
}

impl DetectedLanguage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            percentage: 0.0,
            frameworks: Vec::new(),
            build_tools: Vec::new(),
            marker_files: Vec::new(),
        }
    }

    pub fn with_percentage(mut self, percentage: f64) -> Self {
        self.percentage = percentage;
        self
    }

    pub fn with_frameworks(mut self, frameworks: Vec<String>) -> Self {
        self.frameworks = frameworks;
        self
    }

    pub fn with_build_tools(mut self, build_tools: Vec<String>) -> Self {
        self.build_tools = build_tools;
        self
    }

    pub fn with_marker_files(mut self, marker_files: Vec<String>) -> Self {
        self.marker_files = marker_files;
        self
    }
}

pub fn is_path_in_scope<P: AsRef<Path>>(path: &Path, allowed_paths: &[P]) -> bool {
    allowed_paths
        .iter()
        .any(|allowed| path.starts_with(allowed.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_location_reference() {
        let loc = EvidenceLocation::new("src/main.rs", 42);
        assert_eq!(loc.to_reference(), "src/main.rs:42");

        let loc = EvidenceLocation::new_range("src/lib.rs", 10, 20);
        assert_eq!(loc.to_reference(), "src/lib.rs:10-20");
    }

    #[test]
    fn test_convention_builder() {
        let conv = Convention::new("error-handling", "Use ? operator for propagation")
            .with_rationale("Rust idiom for error handling")
            .with_evidence(vec![EvidenceLocation::new("src/lib.rs", 42)]);

        assert_eq!(conv.name, "error-handling");
        assert!(conv.rationale.is_some());
        assert_eq!(conv.evidence.len(), 1);
    }

    #[test]
    fn test_known_issue_builder() {
        let issue = KnownIssue::new(
            "race-condition",
            "Race condition in session refresh",
            IssueSeverity::High,
            IssueCategory::Concurrency,
        )
        .with_prevention("Use atomic CAS operations")
        .with_evidence(vec![EvidenceLocation::new("src/auth/session.rs", 128)]);

        assert_eq!(issue.id, "race-condition");
        assert_eq!(issue.severity, IssueSeverity::High);
        assert!(issue.prevention.is_some());
    }

    #[test]
    fn test_tech_stack_builder() {
        let stack = TechStack::new("rust")
            .with_version("1.92")
            .with_framework(FrameworkInfo::new("tokio", "async runtime"))
            .with_build_tool("cargo")
            .with_test_framework("built-in")
            .with_library(LibraryInfo::new("serde", "serialization"));

        assert_eq!(stack.primary_language, "rust");
        assert_eq!(stack.language_version, Some("1.92".into()));
        assert_eq!(stack.frameworks.len(), 1);
        assert_eq!(stack.build_tools, vec!["cargo"]);
    }

    #[test]
    fn test_issue_severity_ordering() {
        assert!(IssueSeverity::Critical < IssueSeverity::High);
        assert!(IssueSeverity::High < IssueSeverity::Medium);
        assert!(IssueSeverity::Medium < IssueSeverity::Low);
    }

    #[test]
    fn test_path_in_scope() {
        let allowed: &[&Path] = &[Path::new("src/auth")];
        assert!(is_path_in_scope(Path::new("src/auth/login.rs"), allowed));
        assert!(!is_path_in_scope(Path::new("src/api/routes.rs"), allowed));
    }

    #[test]
    fn test_module_dependency_factories() {
        let dep = ModuleDependency::runtime("database");
        assert_eq!(dep.dependency_type, DependencyType::Runtime);

        let dep = ModuleDependency::build("codegen");
        assert_eq!(dep.dependency_type, DependencyType::Build);

        let dep = ModuleDependency::test("fixtures");
        assert_eq!(dep.dependency_type, DependencyType::Test);

        let dep = ModuleDependency::optional("cache");
        assert_eq!(dep.dependency_type, DependencyType::Optional);
    }
}
