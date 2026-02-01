use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{
    Convention, DetectedLanguage, EvidenceLocation, GeneratorInfo, KnownIssue, ModuleDependency,
    ProjectType, TechStack, WorkspaceType,
};

pub const SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleMap {
    pub schema_version: String,
    pub generator: GeneratorInfo,
    pub project: ProjectMetadata,
    pub modules: Vec<Module>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ModuleGroup>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<Domain>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency_graph: Option<DependencyGraph>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectMetadata {
    pub name: String,
    #[serde(default)]
    pub project_type: ProjectType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    pub workspace: WorkspaceInfo,
    pub tech_stack: TechStack,
    pub languages: Vec<DetectedLanguage>,
    pub total_files: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commands: Option<ProjectCommands>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct WorkspaceInfo {
    #[serde(default)]
    pub workspace_type: WorkspaceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectCommands {
    pub build: String,
    pub test: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ModuleMetrics {
    pub coverage_ratio: f64,
    pub value_score: f64,
    pub risk_score: f64,
}

impl ModuleMetrics {
    pub fn new(coverage_ratio: f64, value_score: f64, risk_score: f64) -> Self {
        Self {
            coverage_ratio,
            value_score,
            risk_score,
        }
    }

    pub fn priority_score(&self) -> f64 {
        self.value_score * 0.6 + self.risk_score * 0.4
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_files: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<ModuleDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependents: Vec<String>,
    pub responsibility: String,
    pub primary_language: String,
    #[serde(flatten)]
    pub metrics: ModuleMetrics,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conventions: Vec<Convention>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_issues: Vec<KnownIssue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<EvidenceLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModuleGroup {
    pub id: String,
    pub name: String,
    pub module_ids: Vec<String>,
    pub responsibility: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub boundary_rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_module: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(default)]
    pub depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub group_ids: Vec<String>,
    pub responsibility: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub boundary_rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<DomainInterface>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DomainInterface {
    pub name: String,
    #[serde(default)]
    pub interface_type: InterfaceType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumers: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceType {
    #[default]
    Api,
    Event,
    SharedLibrary,
    Database,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct DependencyGraph {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<DependencyEdge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub layers: Vec<ArchitectureLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub edge_type: crate::types::DependencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchitectureLayer {
    pub name: String,
    pub modules: Vec<String>,
}

impl ModuleMap {
    pub fn new(
        generator: GeneratorInfo,
        project: ProjectMetadata,
        modules: Vec<Module>,
        groups: Vec<ModuleGroup>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.into(),
            generator,
            project,
            modules,
            groups,
            domains: Vec::new(),
            dependency_graph: None,
            generated_at: chrono::Utc::now(),
        }
    }

    pub fn with_domains(mut self, domains: Vec<Domain>) -> Self {
        self.domains = domains;
        self
    }

    pub fn with_dependency_graph(mut self, graph: DependencyGraph) -> Self {
        self.dependency_graph = Some(graph);
        self
    }

    pub fn find_module(&self, module_id: &str) -> Option<&Module> {
        self.modules.iter().find(|m| m.id == module_id)
    }

    pub fn find_group(&self, group_id: &str) -> Option<&ModuleGroup> {
        self.groups.iter().find(|g| g.id == group_id)
    }

    pub fn find_domain(&self, domain_id: &str) -> Option<&Domain> {
        self.domains.iter().find(|d| d.id == domain_id)
    }

    pub fn find_group_containing(&self, module_id: &str) -> Option<&ModuleGroup> {
        self.groups
            .iter()
            .find(|g| g.module_ids.iter().any(|id| id == module_id))
    }

    pub fn find_domain_containing_group(&self, group_id: &str) -> Option<&Domain> {
        self.domains
            .iter()
            .find(|d| d.group_ids.iter().any(|id| id == group_id))
    }

    pub fn find_modules_in_group(&self, group_id: &str) -> Vec<&Module> {
        self.find_group(group_id)
            .map(|g| {
                g.module_ids
                    .iter()
                    .filter_map(|id| self.find_module(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn find_groups_in_domain(&self, domain_id: &str) -> Vec<&ModuleGroup> {
        self.find_domain(domain_id)
            .map(|d| {
                d.group_ids
                    .iter()
                    .filter_map(|id| self.find_group(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn find_child_groups(&self, parent_group_id: &str) -> Vec<&ModuleGroup> {
        self.groups
            .iter()
            .filter(|g| g.parent_group_id.as_deref() == Some(parent_group_id))
            .collect()
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Module {
    pub fn contains_file(&self, path: &str) -> bool {
        self.paths.iter().any(|p| path.starts_with(p))
    }
}

impl ModuleGroup {
    pub fn new(id: impl Into<String>, name: impl Into<String>, module_ids: Vec<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            module_ids,
            responsibility: String::new(),
            boundary_rules: Vec::new(),
            leader_module: None,
            parent_group_id: None,
            domain_id: None,
            depth: 0,
        }
    }

    pub fn with_responsibility(mut self, responsibility: impl Into<String>) -> Self {
        self.responsibility = responsibility.into();
        self
    }

    pub fn with_boundary_rules(mut self, rules: Vec<String>) -> Self {
        self.boundary_rules = rules;
        self
    }

    pub fn with_domain(mut self, domain_id: impl Into<String>) -> Self {
        self.domain_id = Some(domain_id.into());
        self
    }

    pub fn with_parent(mut self, parent_group_id: impl Into<String>, depth: u8) -> Self {
        self.parent_group_id = Some(parent_group_id.into());
        self.depth = depth;
        self
    }
}

impl Domain {
    pub fn new(id: impl Into<String>, name: impl Into<String>, group_ids: Vec<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            group_ids,
            responsibility: String::new(),
            boundary_rules: Vec::new(),
            interfaces: Vec::new(),
            owner: None,
        }
    }

    pub fn with_responsibility(mut self, responsibility: impl Into<String>) -> Self {
        self.responsibility = responsibility.into();
        self
    }

    pub fn with_boundary_rules(mut self, rules: Vec<String>) -> Self {
        self.boundary_rules = rules;
        self
    }

    pub fn with_interfaces(mut self, interfaces: Vec<DomainInterface>) -> Self {
        self.interfaces = interfaces;
        self
    }

    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }
}

impl DomainInterface {
    pub fn new(name: impl Into<String>, interface_type: InterfaceType) -> Self {
        Self {
            name: name.into(),
            interface_type,
            consumers: Vec::new(),
        }
    }

    pub fn with_consumers(mut self, consumers: Vec<String>) -> Self {
        self.consumers = consumers;
        self
    }
}

impl ProjectMetadata {
    pub fn new(name: impl Into<String>, tech_stack: TechStack) -> Self {
        Self {
            name: name.into(),
            project_type: ProjectType::default(),
            description: None,
            repository: None,
            workspace: WorkspaceInfo::default(),
            tech_stack,
            languages: Vec::new(),
            total_files: 0,
            commands: None,
        }
    }

    pub fn with_type(mut self, project_type: ProjectType) -> Self {
        self.project_type = project_type;
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_workspace(mut self, workspace: WorkspaceInfo) -> Self {
        self.workspace = workspace;
        self
    }

    pub fn with_languages(mut self, languages: Vec<DetectedLanguage>) -> Self {
        self.languages = languages;
        self
    }

    pub fn with_total_files(mut self, total_files: usize) -> Self {
        self.total_files = total_files;
        self
    }

    pub fn with_commands(mut self, commands: ProjectCommands) -> Self {
        self.commands = Some(commands);
        self
    }
}

impl ProjectCommands {
    pub fn new(build: impl Into<String>, test: impl Into<String>) -> Self {
        Self {
            build: build.into(),
            test: test.into(),
            lint: None,
            format: None,
        }
    }

    pub fn with_lint(mut self, lint: impl Into<String>) -> Self {
        self.lint = Some(lint.into());
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{IssueCategory, IssueSeverity};

    fn sample_module(id: &str) -> Module {
        Module {
            id: id.into(),
            name: id.into(),
            paths: vec![format!("src/{}/", id)],
            key_files: vec![],
            dependencies: vec![],
            dependents: vec![],
            responsibility: format!("{} module", id),
            primary_language: "rust".into(),
            metrics: ModuleMetrics::new(0.8, 0.7, 0.3),
            conventions: vec![],
            known_issues: vec![],
            evidence: vec![],
        }
    }

    fn sample_module_with_conventions(id: &str) -> Module {
        Module {
            id: id.into(),
            name: id.into(),
            paths: vec![format!("src/{}/", id)],
            key_files: vec![format!("src/{}/mod.rs", id)],
            dependencies: vec![ModuleDependency::runtime("types")],
            dependents: vec!["cli".into()],
            responsibility: format!("{} module", id),
            primary_language: "rust".into(),
            metrics: ModuleMetrics::new(0.8, 0.7, 0.3),
            conventions: vec![Convention::new(
                "error-handling",
                "Use ? operator for propagation",
            )],
            known_issues: vec![
                KnownIssue::new(
                    "memory-leak",
                    "Unbounded cache growth",
                    IssueSeverity::Medium,
                    IssueCategory::Performance,
                )
                .with_prevention("Add TTL or max size limit"),
            ],
            evidence: vec![EvidenceLocation::new("src/pipeline/mod.rs", 1)],
        }
    }

    fn sample_project() -> ProjectMetadata {
        ProjectMetadata::new("test-project", TechStack::new("rust").with_version("1.92"))
            .with_type(ProjectType::Cli)
            .with_description("A test project")
            .with_workspace(WorkspaceInfo {
                workspace_type: WorkspaceType::SinglePackage,
                root: Some(".".into()),
            })
            .with_total_files(100)
            .with_commands(
                ProjectCommands::new("cargo build", "cargo test")
                    .with_lint("cargo clippy")
                    .with_format("cargo fmt"),
            )
    }

    #[test]
    fn test_module_map_creation() {
        let project = sample_project();
        let modules = vec![sample_module("auth"), sample_module("api")];
        let groups = vec![
            ModuleGroup::new("core", "Core", vec!["auth".into(), "api".into()])
                .with_responsibility("Core processing")
                .with_boundary_rules(vec!["No direct CLI dependency".into()]),
        ];

        let generator = GeneratorInfo::new("test", "1.0.0");
        let map = ModuleMap::new(generator, project, modules, groups);

        assert_eq!(map.schema_version, SCHEMA_VERSION);
        assert!(map.find_module("auth").is_some());
        assert!(map.find_module("nonexistent").is_none());
        assert!(map.find_group_containing("auth").is_some());
    }

    #[test]
    fn test_domain_creation() {
        let domain = Domain::new(
            "identity",
            "Identity Management",
            vec!["auth-group".into(), "user-group".into()],
        )
        .with_responsibility("Handles all identity operations")
        .with_boundary_rules(vec!["External access through API gateway only".into()])
        .with_interfaces(vec![
            DomainInterface::new("IdentityAPI", InterfaceType::Api)
                .with_consumers(vec!["commerce".into()]),
            DomainInterface::new("UserEvents", InterfaceType::Event),
        ])
        .with_owner("identity-team");

        assert_eq!(domain.id, "identity");
        assert_eq!(domain.group_ids.len(), 2);
        assert_eq!(domain.interfaces.len(), 2);
        assert_eq!(domain.owner, Some("identity-team".into()));
    }

    #[test]
    fn test_hierarchical_grouping() {
        let project = sample_project();
        let modules = vec![
            sample_module("auth-core"),
            sample_module("oauth"),
            sample_module("rbac"),
        ];
        let groups = vec![
            ModuleGroup::new(
                "authentication",
                "Authentication",
                vec!["auth-core".into(), "oauth".into()],
            )
            .with_domain("identity"),
            ModuleGroup::new("authorization", "Authorization", vec!["rbac".into()])
                .with_domain("identity"),
        ];
        let domains = vec![Domain::new(
            "identity",
            "Identity",
            vec!["authentication".into(), "authorization".into()],
        )];

        let generator = GeneratorInfo::new("test", "1.0.0");
        let map = ModuleMap::new(generator, project, modules, groups).with_domains(domains);

        assert_eq!(map.domains.len(), 1);
        assert!(map.find_domain("identity").is_some());
        assert_eq!(map.find_groups_in_domain("identity").len(), 2);
        assert!(map.find_domain_containing_group("authentication").is_some());
    }

    #[test]
    fn test_nested_groups() {
        let child_group =
            ModuleGroup::new("oauth-providers", "OAuth Providers", vec!["google".into()])
                .with_parent("authentication", 1);

        assert_eq!(child_group.parent_group_id, Some("authentication".into()));
        assert_eq!(child_group.depth, 1);
    }

    #[test]
    fn test_module_with_conventions_and_issues() {
        let module = sample_module_with_conventions("pipeline");

        assert_eq!(module.conventions.len(), 1);
        assert_eq!(module.conventions[0].name, "error-handling");

        assert_eq!(module.known_issues.len(), 1);
        assert_eq!(module.known_issues[0].severity, IssueSeverity::Medium);
        assert!(module.known_issues[0].prevention.is_some());
    }

    #[test]
    fn test_module_contains_file() {
        let module = sample_module("auth");
        assert!(module.contains_file("src/auth/login.rs"));
        assert!(!module.contains_file("src/api/routes.rs"));
    }

    #[test]
    fn test_priority_score() {
        let metrics = ModuleMetrics::new(0.8, 0.8, 0.5);
        let expected = 0.8 * 0.6 + 0.5 * 0.4;
        assert!((metrics.priority_score() - expected).abs() < 0.001);
    }

    #[test]
    fn test_dependency_graph() {
        let project = sample_project();
        let modules = vec![sample_module("auth"), sample_module("api")];
        let groups = vec![];

        let graph = DependencyGraph {
            edges: vec![DependencyEdge {
                from: "api".into(),
                to: "auth".into(),
                edge_type: crate::types::DependencyType::Runtime,
            }],
            layers: vec![
                ArchitectureLayer {
                    name: "presentation".into(),
                    modules: vec!["cli".into()],
                },
                ArchitectureLayer {
                    name: "domain".into(),
                    modules: vec!["auth".into(), "api".into()],
                },
            ],
        };

        let generator = GeneratorInfo::new("test", "1.0.0");
        let map = ModuleMap::new(generator, project, modules, groups).with_dependency_graph(graph);

        assert!(map.dependency_graph.is_some());
        let graph = map.dependency_graph.unwrap();
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.layers.len(), 2);
    }

    #[test]
    fn test_serialization_with_domains() {
        let project = sample_project();
        let modules = vec![sample_module("auth")];
        let groups = vec![
            ModuleGroup::new("auth-group", "Auth Group", vec!["auth".into()])
                .with_domain("identity"),
        ];
        let domains = vec![
            Domain::new("identity", "Identity", vec!["auth-group".into()])
                .with_interfaces(vec![DomainInterface::new("AuthAPI", InterfaceType::Api)]),
        ];

        let generator = GeneratorInfo::new("claudegen", "0.3.0");
        let map = ModuleMap::new(generator, project, modules, groups).with_domains(domains);

        let json = map.to_json().expect("serialization should succeed");
        assert!(json.contains("\"domains\""));
        assert!(json.contains("\"identity\""));
        assert!(json.contains("\"domain_id\""));

        let parsed: ModuleMap =
            serde_json::from_str(&json).expect("deserialization should succeed");
        assert_eq!(parsed.domains.len(), 1);
        assert_eq!(parsed.domains[0].interfaces.len(), 1);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let project = sample_project();
        let modules = vec![sample_module_with_conventions("pipeline")];
        let groups = vec![];
        let generator = GeneratorInfo::new("claudegen", "0.2.0");
        let map = ModuleMap::new(generator, project, modules, groups);

        let json = map.to_json().expect("serialization should succeed");
        assert!(json.contains("\"schema_version\": \"1.0.0\""));
        assert!(json.contains("\"error-handling\""));
        assert!(json.contains("\"memory-leak\""));

        let parsed: ModuleMap =
            serde_json::from_str(&json).expect("deserialization should succeed");
        assert_eq!(parsed.schema_version, "1.0.0");
        assert_eq!(parsed.modules[0].conventions.len(), 1);
    }
}
