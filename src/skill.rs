//! Skill schema types for Claude Code plugins

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Context mode for skill execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ContextMode {
    Fork,
}

impl std::fmt::Display for ContextMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fork => write!(f, "fork"),
        }
    }
}

impl std::str::FromStr for ContextMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fork" => Ok(Self::Fork),
            _ => Err(format!("unknown context mode: {s}")),
        }
    }
}

/// Additional file bundled with a skill
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SkillFile {
    pub name: String,
    pub content: String,
}

impl SkillFile {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }
}

/// Skill definition for Claude Code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Skill {
    /// Unique identifier (kebab-case)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Semantic version
    #[serde(default = "default_version")]
    pub version: String,
    /// Allowed tools (comma-separated in output)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_tools: Vec<String>,
    /// Model override
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Context execution mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ContextMode>,
    /// Agent to delegate to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Whether user can invoke via /skill-name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_invocable: Option<bool>,
    /// Hint shown in CLI
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub argument_hint: Option<String>,
    /// Disable automatic model invocation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_model_invocation: Option<bool>,
    /// Markdown body content
    pub body: String,
    /// Additional files bundled with skill
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_files: Vec<SkillFile>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl Skill {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version: default_version(),
            allowed_tools: Vec::new(),
            model: None,
            context: None,
            agent: None,
            user_invocable: None,
            argument_hint: None,
            disable_model_invocation: None,
            body: body.into(),
            additional_files: Vec::new(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn with_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = tools;
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_context(mut self, context: ContextMode) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_agent(mut self, agent: impl Into<String>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    pub fn with_user_invocable(mut self, invocable: bool) -> Self {
        self.user_invocable = Some(invocable);
        self
    }

    pub fn with_argument_hint(mut self, hint: impl Into<String>) -> Self {
        self.argument_hint = Some(hint.into());
        self
    }

    pub fn with_disable_model_invocation(mut self, disable: bool) -> Self {
        self.disable_model_invocation = Some(disable);
        self
    }

    pub fn with_additional_file(mut self, file: SkillFile) -> Self {
        self.additional_files.push(file);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new(
            "code-review",
            "Review code for issues",
            "# Code Review\n...",
        );
        assert_eq!(skill.name, "code-review");
        assert_eq!(skill.version, "1.0.0");
    }

    #[test]
    fn test_skill_builder() {
        let skill = Skill::new("test", "desc", "body")
            .with_tools(vec!["Read".into(), "Grep".into()])
            .with_user_invocable(true)
            .with_model("sonnet");

        assert_eq!(skill.allowed_tools, vec!["Read", "Grep"]);
        assert_eq!(skill.user_invocable, Some(true));
        assert_eq!(skill.model, Some("sonnet".into()));
    }

    #[test]
    fn test_context_mode_display() {
        assert_eq!(ContextMode::Fork.to_string(), "fork");
    }

    #[test]
    fn test_context_mode_parse() {
        assert_eq!("fork".parse::<ContextMode>().unwrap(), ContextMode::Fork);
        assert!("invalid".parse::<ContextMode>().is_err());
    }

    #[test]
    fn test_skill_serialization() {
        let skill = Skill::new("test", "desc", "body").with_tools(vec!["Read".into()]);
        let json = serde_json::to_string(&skill).unwrap();
        let parsed: Skill = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.allowed_tools, vec!["Read"]);
    }
}
