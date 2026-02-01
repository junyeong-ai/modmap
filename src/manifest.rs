use std::collections::HashMap;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ModuleMap;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ModuleContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conventions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl ModuleContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.skills = skills;
        self
    }

    pub fn with_conventions(mut self, conventions: Vec<String>) -> Self {
        self.conventions = conventions;
        self
    }

    pub fn with_issues(mut self, issues: Vec<String>) -> Self {
        self.issues = issues;
        self
    }

    pub fn with_group(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    pub fn with_domain(mut self, domain_id: impl Into<String>) -> Self {
        self.domain_id = Some(domain_id.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
            && self.skills.is_empty()
            && self.conventions.is_empty()
            && self.issues.is_empty()
            && self.group_id.is_none()
            && self.domain_id.is_none()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GroupContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_modules: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

impl GroupContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn with_members(mut self, member_modules: Vec<String>) -> Self {
        self.member_modules = member_modules;
        self
    }

    pub fn with_domain(mut self, domain_id: impl Into<String>) -> Self {
        self.domain_id = Some(domain_id.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
            && self.constraints.is_empty()
            && self.member_modules.is_empty()
            && self.domain_id.is_none()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DomainContext {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraints: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_groups: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<String>,
}

impl DomainContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_constraints(mut self, constraints: Vec<String>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn with_groups(mut self, member_groups: Vec<String>) -> Self {
        self.member_groups = member_groups;
        self
    }

    pub fn with_interfaces(mut self, interfaces: Vec<String>) -> Self {
        self.interfaces = interfaces;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
            && self.constraints.is_empty()
            && self.member_groups.is_empty()
            && self.interfaces.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TrackedFile {
    pub path: String,
    pub hash: String,
    pub modified: i64,
}

impl TrackedFile {
    pub fn new(path: impl Into<String>, hash: impl Into<String>, modified: i64) -> Self {
        Self {
            path: path.into(),
            hash: hash.into(),
            modified,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectManifest {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub generator: String,
    pub project: ModuleMap,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agents: Vec<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub modules: HashMap<String, ModuleContext>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub groups: HashMap<String, GroupContext>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub domains: HashMap<String, DomainContext>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracked: Vec<TrackedFile>,
}

impl ProjectManifest {
    pub fn new(project: ModuleMap) -> Self {
        Self {
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            generator: "claudegen".to_string(),
            project,
            rules: Vec::new(),
            skills: Vec::new(),
            agents: Vec::new(),
            modules: HashMap::new(),
            groups: HashMap::new(),
            domains: HashMap::new(),
            tracked: Vec::new(),
        }
    }

    pub fn with_generator(mut self, generator: impl Into<String>) -> Self {
        self.generator = generator.into();
        self
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.skills = skills;
        self
    }

    pub fn with_agents(mut self, agents: Vec<String>) -> Self {
        self.agents = agents;
        self
    }

    pub fn with_modules(mut self, modules: HashMap<String, ModuleContext>) -> Self {
        self.modules = modules;
        self
    }

    pub fn with_groups(mut self, groups: HashMap<String, GroupContext>) -> Self {
        self.groups = groups;
        self
    }

    pub fn with_domains(mut self, domains: HashMap<String, DomainContext>) -> Self {
        self.domains = domains;
        self
    }

    pub fn with_tracked(mut self, tracked: Vec<TrackedFile>) -> Self {
        self.tracked = tracked;
        self
    }

    pub fn get_module_context(&self, module_id: &str) -> Option<&ModuleContext> {
        self.modules.get(module_id)
    }

    pub fn get_group_context(&self, group_id: &str) -> Option<&GroupContext> {
        self.groups.get(group_id)
    }

    pub fn get_domain_context(&self, domain_id: &str) -> Option<&DomainContext> {
        self.domains.get(domain_id)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GeneratorInfo, ModuleMap, ProjectMetadata, TechStack};

    fn sample_module_map() -> ModuleMap {
        let generator = GeneratorInfo::new("claudegen", "1.0.0");
        let project = ProjectMetadata::new("test-project", TechStack::new("rust"));
        ModuleMap::new(generator, project, vec![], vec![])
    }

    #[test]
    fn test_manifest_creation() {
        let manifest = ProjectManifest::new(sample_module_map());

        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.generator, "claudegen");
        assert!(manifest.modules.is_empty());
        assert!(manifest.groups.is_empty());
        assert!(manifest.domains.is_empty());
        assert!(manifest.tracked.is_empty());
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest = ProjectManifest::new(sample_module_map());

        let json = manifest.to_json().unwrap();
        let parsed = ProjectManifest::from_json(&json).unwrap();

        assert_eq!(parsed.version, "1.0.0");
        assert_eq!(parsed.project.project.name, "test-project");
    }

    #[test]
    fn test_flat_resource_lists() {
        let manifest = ProjectManifest::new(sample_module_map())
            .with_rules(vec!["rules/project.md".into(), "rules/tech/rust.md".into()])
            .with_skills(vec!["skills/code-review/SKILL.md".into()])
            .with_agents(vec!["agents/reviewer.md".into()]);

        assert_eq!(manifest.rules.len(), 2);
        assert_eq!(manifest.skills.len(), 1);
        assert_eq!(manifest.agents.len(), 1);

        let json = manifest.to_json().unwrap();
        assert!(json.contains("rules/project.md"));
        assert!(json.contains("skills/code-review/SKILL.md"));
        assert!(json.contains("agents/reviewer.md"));
    }

    #[test]
    fn test_module_context() {
        let ctx = ModuleContext::new()
            .with_rules(vec!["rules/modules/auth.md".into()])
            .with_skills(vec!["code-review".into(), "implement".into()])
            .with_conventions(vec!["bcrypt: Use cost factor 12".into()])
            .with_issues(vec!["[MEDIUM] token-refresh: May fail under load".into()])
            .with_group("authentication")
            .with_domain("identity");

        assert_eq!(ctx.rules.len(), 1);
        assert_eq!(ctx.skills.len(), 2);
        assert_eq!(ctx.conventions.len(), 1);
        assert_eq!(ctx.issues.len(), 1);
        assert_eq!(ctx.group_id, Some("authentication".into()));
        assert_eq!(ctx.domain_id, Some("identity".into()));
        assert!(!ctx.is_empty());
    }

    #[test]
    fn test_group_context() {
        let ctx = GroupContext::new()
            .with_rules(vec!["rules/groups/auth-group.md".into()])
            .with_constraints(vec!["Must use bcrypt for passwords".into()])
            .with_members(vec!["auth-core".into(), "oauth".into()])
            .with_domain("identity");

        assert_eq!(ctx.rules.len(), 1);
        assert_eq!(ctx.constraints.len(), 1);
        assert_eq!(ctx.member_modules.len(), 2);
        assert_eq!(ctx.domain_id, Some("identity".into()));
        assert!(!ctx.is_empty());
    }

    #[test]
    fn test_domain_context() {
        let ctx = DomainContext::new()
            .with_rules(vec!["rules/domains/identity.md".into()])
            .with_constraints(vec!["External access through gateway only".into()])
            .with_groups(vec!["authentication".into(), "authorization".into()])
            .with_interfaces(vec!["IdentityAPI".into(), "UserEvents".into()]);

        assert_eq!(ctx.rules.len(), 1);
        assert_eq!(ctx.constraints.len(), 1);
        assert_eq!(ctx.member_groups.len(), 2);
        assert_eq!(ctx.interfaces.len(), 2);
        assert!(!ctx.is_empty());
    }

    #[test]
    fn test_manifest_with_hierarchical_contexts() {
        let mut modules = HashMap::new();
        modules.insert(
            "auth-core".to_string(),
            ModuleContext::new()
                .with_rules(vec!["rules/modules/auth-core.md".into()])
                .with_group("authentication")
                .with_domain("identity"),
        );

        let mut groups = HashMap::new();
        groups.insert(
            "authentication".to_string(),
            GroupContext::new()
                .with_rules(vec!["rules/groups/authentication.md".into()])
                .with_members(vec!["auth-core".into()])
                .with_domain("identity"),
        );

        let mut domains = HashMap::new();
        domains.insert(
            "identity".to_string(),
            DomainContext::new()
                .with_rules(vec!["rules/domains/identity.md".into()])
                .with_groups(vec!["authentication".into()]),
        );

        let manifest = ProjectManifest::new(sample_module_map())
            .with_modules(modules)
            .with_groups(groups)
            .with_domains(domains);

        assert!(manifest.get_module_context("auth-core").is_some());
        assert!(manifest.get_group_context("authentication").is_some());
        assert!(manifest.get_domain_context("identity").is_some());

        let json = manifest.to_json().unwrap();
        assert!(json.contains("auth-core"));
        assert!(json.contains("authentication"));
        assert!(json.contains("identity"));
    }

    #[test]
    fn test_tracked_file() {
        let file = TrackedFile::new("src/auth/mod.rs", "abc123def456", 1706529600);

        assert_eq!(file.path, "src/auth/mod.rs");
        assert_eq!(file.hash, "abc123def456");
        assert_eq!(file.modified, 1706529600);
    }

    #[test]
    fn test_manifest_with_tracked_files() {
        let tracked = vec![
            TrackedFile::new("src/auth/mod.rs", "abc123", 1706529600),
            TrackedFile::new("src/auth/session.rs", "def456", 1706529700),
        ];

        let manifest = ProjectManifest::new(sample_module_map()).with_tracked(tracked);

        assert_eq!(manifest.tracked.len(), 2);
        assert_eq!(manifest.tracked[0].path, "src/auth/mod.rs");
    }

    #[test]
    fn test_empty_fields_omitted_in_json() {
        let manifest = ProjectManifest::new(sample_module_map());

        let json = manifest.to_json().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("rules").is_none());
        assert!(parsed.get("skills").is_none());
        assert!(parsed.get("agents").is_none());
        assert!(parsed.get("modules").is_none());
        assert!(parsed.get("groups").is_none());
        assert!(parsed.get("domains").is_none());
        assert!(parsed.get("tracked").is_none());
    }
}
