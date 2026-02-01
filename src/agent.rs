//! Agent schema types for Claude Code plugins

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Agent color for UI display
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AgentColor {
    #[default]
    Blue,
    Green,
    Purple,
    Orange,
    Red,
}

impl std::fmt::Display for AgentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blue => write!(f, "blue"),
            Self::Green => write!(f, "green"),
            Self::Purple => write!(f, "purple"),
            Self::Orange => write!(f, "orange"),
            Self::Red => write!(f, "red"),
        }
    }
}

impl std::str::FromStr for AgentColor {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "blue" => Self::Blue,
            "green" => Self::Green,
            "purple" => Self::Purple,
            "orange" => Self::Orange,
            "red" => Self::Red,
            _ => Self::Blue,
        })
    }
}

/// Model selection for agent
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AgentModel {
    Sonnet,
    Opus,
    Haiku,
    #[default]
    Inherit,
}

impl std::fmt::Display for AgentModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sonnet => write!(f, "sonnet"),
            Self::Opus => write!(f, "opus"),
            Self::Haiku => write!(f, "haiku"),
            Self::Inherit => write!(f, "inherit"),
        }
    }
}

impl std::str::FromStr for AgentModel {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "sonnet" => Self::Sonnet,
            "opus" => Self::Opus,
            "haiku" => Self::Haiku,
            "inherit" => Self::Inherit,
            _ => Self::Inherit,
        })
    }
}

/// Permission mode for agent operations
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    #[default]
    Default,
    AcceptEdits,
    DontAsk,
    BypassPermissions,
    Plan,
}

impl std::fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::AcceptEdits => write!(f, "acceptEdits"),
            Self::DontAsk => write!(f, "dontAsk"),
            Self::BypassPermissions => write!(f, "bypassPermissions"),
            Self::Plan => write!(f, "plan"),
        }
    }
}

impl std::str::FromStr for PermissionMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().replace('_', "").as_str() {
            "acceptedits" => Self::AcceptEdits,
            "dontask" => Self::DontAsk,
            "bypasspermissions" => Self::BypassPermissions,
            "plan" => Self::Plan,
            _ => Self::Default,
        })
    }
}

/// Consensus role configuration for multi-agent coordination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ConsensusRole {
    /// Priority in consensus (higher = more weight)
    pub priority: u8,
    /// Whether this agent can veto decisions
    #[serde(default)]
    pub can_veto: bool,
    /// Required approval threshold (0.0-1.0)
    #[serde(default = "default_vote_threshold")]
    pub vote_threshold: f64,
}

fn default_vote_threshold() -> f64 {
    0.67
}

impl Default for ConsensusRole {
    fn default() -> Self {
        Self {
            priority: 50,
            can_veto: false,
            vote_threshold: 0.67,
        }
    }
}

impl ConsensusRole {
    pub fn new(priority: u8) -> Self {
        Self {
            priority,
            can_veto: false,
            vote_threshold: 0.67,
        }
    }

    pub fn with_veto(mut self) -> Self {
        self.can_veto = true;
        self
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.vote_threshold = threshold;
        self
    }
}

/// Example for agent prompt
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AgentExample {
    pub context: String,
    pub user: String,
    pub assistant: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commentary: Option<String>,
}

impl AgentExample {
    pub fn new(
        context: impl Into<String>,
        user: impl Into<String>,
        assistant: impl Into<String>,
    ) -> Self {
        Self {
            context: context.into(),
            user: user.into(),
            assistant: assistant.into(),
            commentary: None,
        }
    }

    pub fn with_commentary(mut self, commentary: impl Into<String>) -> Self {
        self.commentary = Some(commentary.into());
        self
    }
}

/// Agent definition for Claude Code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Agent {
    /// Unique identifier (kebab-case)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// UI color
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<AgentColor>,
    /// Allowed tools
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<String>,
    /// Disallowed tools
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disallowed_tools: Vec<String>,
    /// Model override
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<AgentModel>,
    /// Permission mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_mode: Option<PermissionMode>,
    /// Skills this agent can use
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    /// Consensus role for multi-agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consensus: Option<ConsensusRole>,
    /// System prompt
    pub prompt: String,
    /// Example interactions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<AgentExample>,
}

impl Agent {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        prompt: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            color: None,
            tools: Vec::new(),
            disallowed_tools: Vec::new(),
            model: None,
            permission_mode: None,
            skills: Vec::new(),
            consensus: None,
            prompt: prompt.into(),
            examples: Vec::new(),
        }
    }

    pub fn with_color(mut self, color: AgentColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_tools(mut self, tools: Vec<String>) -> Self {
        self.tools = tools;
        self
    }

    pub fn with_disallowed_tools(mut self, tools: Vec<String>) -> Self {
        self.disallowed_tools = tools;
        self
    }

    pub fn with_model(mut self, model: AgentModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn with_permission_mode(mut self, mode: PermissionMode) -> Self {
        self.permission_mode = Some(mode);
        self
    }

    pub fn with_skills(mut self, skills: Vec<String>) -> Self {
        self.skills = skills;
        self
    }

    pub fn with_consensus(mut self, consensus: ConsensusRole) -> Self {
        self.consensus = Some(consensus);
        self
    }

    pub fn with_example(mut self, example: AgentExample) -> Self {
        self.examples.push(example);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new("reviewer", "Code review agent", "You review code.");
        assert_eq!(agent.name, "reviewer");
        assert!(agent.tools.is_empty());
    }

    #[test]
    fn test_agent_builder() {
        let agent = Agent::new("test", "desc", "prompt")
            .with_color(AgentColor::Green)
            .with_tools(vec!["Read".into(), "Grep".into()])
            .with_model(AgentModel::Sonnet)
            .with_skills(vec!["code-review".into()]);

        assert_eq!(agent.color, Some(AgentColor::Green));
        assert_eq!(agent.tools, vec!["Read", "Grep"]);
        assert_eq!(agent.model, Some(AgentModel::Sonnet));
        assert_eq!(agent.skills, vec!["code-review"]);
    }

    #[test]
    fn test_consensus_role() {
        let role = ConsensusRole::new(80).with_veto().with_threshold(0.75);
        assert_eq!(role.priority, 80);
        assert!(role.can_veto);
        assert!((role.vote_threshold - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_agent_color_parse() {
        assert_eq!("blue".parse::<AgentColor>().unwrap(), AgentColor::Blue);
        assert_eq!("GREEN".parse::<AgentColor>().unwrap(), AgentColor::Green);
        assert_eq!("unknown".parse::<AgentColor>().unwrap(), AgentColor::Blue);
    }

    #[test]
    fn test_agent_model_parse() {
        assert_eq!("sonnet".parse::<AgentModel>().unwrap(), AgentModel::Sonnet);
        assert_eq!("OPUS".parse::<AgentModel>().unwrap(), AgentModel::Opus);
        assert_eq!(
            "unknown".parse::<AgentModel>().unwrap(),
            AgentModel::Inherit
        );
    }

    #[test]
    fn test_permission_mode_parse() {
        assert_eq!(
            "acceptedits".parse::<PermissionMode>().unwrap(),
            PermissionMode::AcceptEdits
        );
        assert_eq!(
            "dontask".parse::<PermissionMode>().unwrap(),
            PermissionMode::DontAsk
        );
    }

    #[test]
    fn test_agent_example() {
        let example = AgentExample::new("context", "user input", "assistant response")
            .with_commentary("This shows proper handling");

        assert_eq!(example.context, "context");
        assert_eq!(
            example.commentary,
            Some("This shows proper handling".into())
        );
    }

    #[test]
    fn test_agent_serialization() {
        let agent = Agent::new("test", "desc", "prompt")
            .with_tools(vec!["Read".into()])
            .with_consensus(ConsensusRole::new(70));

        let json = serde_json::to_string(&agent).unwrap();
        let parsed: Agent = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "test");
        assert_eq!(parsed.tools, vec!["Read"]);
        assert!(parsed.consensus.is_some());
    }
}
