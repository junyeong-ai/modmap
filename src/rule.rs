//! Rule schema types for Claude Code plugins

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Rule category for hierarchical organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum RuleCategory {
    /// Project-wide rules (priority 100, always inject)
    #[default]
    Project,
    /// Language/tech-specific rules (priority 90, by extension)
    Tech,
    /// Framework-specific rules (priority 85, by path/keywords)
    Framework,
    /// Module-specific rules (priority 80, by module path)
    Module,
    /// Cross-module group rules (priority 70, by member paths)
    Group,
    /// Domain-specific rules (priority 60, by keyword trigger)
    Domain,
}

impl RuleCategory {
    pub const fn default_priority(self) -> u8 {
        match self {
            Self::Project => 100,
            Self::Tech => 90,
            Self::Framework => 85,
            Self::Module => 80,
            Self::Group => 70,
            Self::Domain => 60,
        }
    }

    pub const fn subdirectory(self) -> &'static str {
        match self {
            Self::Project => "",
            Self::Tech => "tech",
            Self::Framework => "frameworks",
            Self::Module => "modules",
            Self::Group => "groups",
            Self::Domain => "domains",
        }
    }
}

impl std::fmt::Display for RuleCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project => write!(f, "project"),
            Self::Tech => write!(f, "tech"),
            Self::Framework => write!(f, "framework"),
            Self::Module => write!(f, "module"),
            Self::Group => write!(f, "group"),
            Self::Domain => write!(f, "domain"),
        }
    }
}

/// Rule definition for context-aware knowledge injection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    /// Unique identifier (kebab-case)
    pub name: String,
    /// Path patterns for auto-injection (glob syntax)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
    /// Keyword triggers for auto-injection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub triggers: Vec<String>,
    /// Injection priority (higher = injected first, 0-100)
    #[serde(default = "default_priority")]
    pub priority: u8,
    /// Rule category
    #[serde(default)]
    pub category: RuleCategory,
    /// Whether this rule is always injected
    #[serde(default)]
    pub always_inject: bool,
    /// Markdown content lines
    pub content: Vec<String>,
}

fn default_priority() -> u8 {
    50
}

impl Rule {
    pub fn new(name: impl Into<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths: Vec::new(),
            triggers: Vec::new(),
            priority: default_priority(),
            category: RuleCategory::default(),
            always_inject: false,
            content,
        }
    }

    pub fn project(name: impl Into<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths: vec!["**/*".into()],
            triggers: Vec::new(),
            priority: RuleCategory::Project.default_priority(),
            category: RuleCategory::Project,
            always_inject: true,
            content,
        }
    }

    pub fn tech(name: impl Into<String>, paths: Vec<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths,
            triggers: Vec::new(),
            priority: RuleCategory::Tech.default_priority(),
            category: RuleCategory::Tech,
            always_inject: false,
            content,
        }
    }

    pub fn framework(
        name: impl Into<String>,
        paths: Vec<String>,
        triggers: Vec<String>,
        content: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            paths,
            triggers,
            priority: RuleCategory::Framework.default_priority(),
            category: RuleCategory::Framework,
            always_inject: false,
            content,
        }
    }

    pub fn module(name: impl Into<String>, paths: Vec<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths,
            triggers: Vec::new(),
            priority: RuleCategory::Module.default_priority(),
            category: RuleCategory::Module,
            always_inject: false,
            content,
        }
    }

    pub fn group(name: impl Into<String>, paths: Vec<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths,
            triggers: Vec::new(),
            priority: RuleCategory::Group.default_priority(),
            category: RuleCategory::Group,
            always_inject: false,
            content,
        }
    }

    pub fn domain(name: impl Into<String>, triggers: Vec<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            paths: Vec::new(),
            triggers,
            priority: RuleCategory::Domain.default_priority(),
            category: RuleCategory::Domain,
            always_inject: false,
            content,
        }
    }

    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths = paths;
        self
    }

    pub fn with_triggers(mut self, triggers: Vec<String>) -> Self {
        self.triggers = triggers;
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_category(mut self, category: RuleCategory) -> Self {
        self.priority = category.default_priority();
        self.category = category;
        self
    }

    pub fn output_path(&self) -> String {
        let subdir = self.category.subdirectory();
        if subdir.is_empty() {
            format!("{}.md", self.name)
        } else {
            format!("{}/{}.md", subdir, self.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_priorities() {
        assert_eq!(RuleCategory::Project.default_priority(), 100);
        assert_eq!(RuleCategory::Tech.default_priority(), 90);
        assert_eq!(RuleCategory::Framework.default_priority(), 85);
        assert_eq!(RuleCategory::Module.default_priority(), 80);
        assert_eq!(RuleCategory::Group.default_priority(), 70);
        assert_eq!(RuleCategory::Domain.default_priority(), 60);
    }

    #[test]
    fn test_category_subdirectories() {
        assert_eq!(RuleCategory::Project.subdirectory(), "");
        assert_eq!(RuleCategory::Tech.subdirectory(), "tech");
        assert_eq!(RuleCategory::Module.subdirectory(), "modules");
        assert_eq!(RuleCategory::Domain.subdirectory(), "domains");
    }

    #[test]
    fn test_rule_output_paths() {
        assert_eq!(Rule::project("proj", vec![]).output_path(), "proj.md");
        assert_eq!(
            Rule::tech("rust", vec![], vec![]).output_path(),
            "tech/rust.md"
        );
        assert_eq!(
            Rule::module("auth", vec![], vec![]).output_path(),
            "modules/auth.md"
        );
        assert_eq!(
            Rule::domain("security", vec![], vec![]).output_path(),
            "domains/security.md"
        );
    }

    #[test]
    fn test_project_rule_always_injects() {
        let rule = Rule::project("project", vec!["content".into()]);
        assert!(rule.always_inject);
        assert_eq!(rule.priority, 100);
        assert_eq!(rule.category, RuleCategory::Project);
    }

    #[test]
    fn test_rule_serialization() {
        let rule = Rule::tech("rust", vec!["**/*.rs".into()], vec!["# Rust".into()]);
        let json = serde_json::to_string(&rule).unwrap();
        let parsed: Rule = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "rust");
        assert_eq!(parsed.paths, vec!["**/*.rs"]);
    }
}
