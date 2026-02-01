# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Build Commands

```bash
cargo build          # Build
cargo test           # Test all
cargo test <name>    # Test specific
cargo clippy         # Lint
cargo fmt            # Format
```

## Overview

`modmap` - Universal module map schema for codebase structure representation.

Two primary use cases:
1. **Codebase Analysis**: Language-agnostic schema for representing module structure
2. **Plugin System**: Schema types for Claude Code plugins (agents, rules, skills)

## Module Structure

```
lib.rs                    # Re-exports all modules
├── types.rs              # Core types (enums, base structs)
├── module_map.rs         # ModuleMap schema (root)
├── manifest.rs           # ProjectManifest (wraps ModuleMap)
├── agent.rs              # Agent definitions
├── rule.rs               # Rule definitions
├── skill.rs              # Skill definitions
└── registry.rs           # Version validation
```

## Core Schema (module_map.rs)

```
ModuleMap (SCHEMA_VERSION = "1.0.0")
├── generator: GeneratorInfo
├── project: ProjectMetadata
│   ├── tech_stack: TechStack
│   ├── workspace: WorkspaceInfo
│   └── commands: ProjectCommands
├── modules: Vec<Module>
│   ├── dependencies: Vec<ModuleDependency>
│   ├── metrics: ModuleMetrics (flattened)
│   ├── conventions: Vec<Convention>
│   └── known_issues: Vec<KnownIssue>
├── groups: Vec<ModuleGroup>
├── domains: Vec<Domain>
└── dependency_graph: Option<DependencyGraph>
```

## Plugin Schema

### agent.rs
- `Agent` - Agent definition with tools, model, instructions
- `AgentColor` - blue | green | purple | orange | red
- `AgentModel` - sonnet | opus | haiku
- `PermissionMode` - default | bypass_permissions | plan_mode

### rule.rs
- `Rule` - Rule with category, trigger, content
- `RuleCategory` - project | tech | framework | module | group | domain
- Priority: project(100) > tech(90) > framework(85) > module(80) > group(70) > domain(60)

### skill.rs
- `Skill` - Skill definition with tools, instructions
- `SkillFile` - Additional bundled files
- `ContextMode` - fork

### manifest.rs
- `ProjectManifest` - Root container with ModuleMap + metadata
- `ModuleContext` - Module-specific rules, skills, conventions
- `GroupContext` / `DomainContext` - Group/domain level contexts

## Key Patterns

```rust
// Factory methods for dependencies
ModuleDependency::runtime("db")
ModuleDependency::build("codegen")
ModuleDependency::test("fixtures")

// Builder pattern - Core types
TechStack::new("rust").with_version("1.92").with_framework(...)
Convention::new("name", "pattern").with_rationale("why")

// Builder pattern - Plugin types (required params in new())
Agent::new("name", "description", "prompt").with_model(AgentModel::Haiku)
Rule::new("name", vec!["content"]).with_paths(vec!["**/*.rs"])
Skill::new("name", "description", "body").with_tools(vec!["Bash"])

// Serde attributes
#[serde(default)]                              // Default on deserialize
#[serde(skip_serializing_if = "Vec::is_empty")] // Omit empty
#[serde(flatten)]                              // Flatten nested struct
```

## Registry Usage

```rust
let registry = SchemaRegistry::new();
let manifest = registry.load(&json_string)?;  // Validates major version
```

## Design Constraints

- All types must work universally across languages/frameworks
- JSON Schema generation via `schemars::JsonSchema`
- SemVer versioning with major version compatibility check
