# modmap

**Universal Module Map Schema for Codebase Structure Representation**

[![CI](https://github.com/junyeong-ai/modmap/actions/workflows/ci.yml/badge.svg)](https://github.com/junyeong-ai/modmap/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/modmap.svg)](https://crates.io/crates/modmap)
[![Docs.rs](https://img.shields.io/docsrs/modmap)](https://docs.rs/modmap)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/crates/l/modmap.svg)](LICENSE)

English | [한국어](README.ko.md)

---

## Overview

`modmap` is a language-agnostic schema library for representing codebase structure. It provides standardized types for:

1. **Codebase Analysis**: Modules, dependencies, conventions, known issues
2. **Plugin System**: Agents, rules, skills for Claude Code plugins

---

## Installation

```toml
[dependencies]
modmap = "1.1"
```

---

## Module Reference

| Module | Description | Key Types |
|--------|-------------|-----------|
| `module_map` | Core schema | ModuleMap, Module, ModuleGroup, Domain |
| `types` | Base types | DependencyType, Convention, KnownIssue, TechStack |
| `manifest` | Project manifest | ProjectManifest, ModuleContext |
| `agent` | Agent definitions | Agent, AgentModel, AgentColor |
| `rule` | Rule definitions | Rule, RuleCategory |
| `skill` | Skill definitions | Skill, SkillFile, ContextMode |
| `registry` | Version validation | SchemaRegistry, SchemaError |

---

## Quick Start

### Creating a Module Map

```rust
use modmap::{
    GeneratorInfo, ModuleMap, ProjectMetadata,
    TechStack, Module, ModuleMetrics, ModuleDependency,
};

let project = ProjectMetadata::new("my-project", TechStack::new("rust"))
    .with_description("Example project");

let modules = vec![Module {
    id: "auth".into(),
    name: "Authentication".into(),
    paths: vec!["src/auth/".into()],
    dependencies: vec![ModuleDependency::runtime("db")],
    responsibility: "User authentication".into(),
    ..Default::default()
}];

let map = ModuleMap::new(
    GeneratorInfo::new("analyzer", "1.0.0"),
    project,
    modules,
    vec![],
);

let json = map.to_json()?;
```

### Loading with Manifest

```rust
use modmap::SchemaRegistry;

let registry = SchemaRegistry::new();
let manifest = registry.load(&json_string)?;

println!("Project: {}", manifest.project.project.name);
```

---

## Plugin Types

### Agent

```rust
use modmap::{Agent, AgentModel, AgentColor};

let agent = Agent::new("code-reviewer")
    .with_description("Reviews code for best practices")
    .with_model(AgentModel::Haiku)
    .with_color(AgentColor::Green)
    .with_tools(vec!["Read".into(), "Grep".into(), "Glob".into()])
    .with_instructions("Review code focusing on security and performance.");
```

### Rule

```rust
use modmap::{Rule, RuleCategory};

let rule = Rule::new("rust-conventions")
    .with_category(RuleCategory::Tech)
    .with_globs(vec!["**/*.rs".into()])
    .with_content("Use `?` operator for error propagation. Prefer `impl Trait` over generics.");
```

**Rule Priority by Category:**

| Category | Priority | Trigger |
|----------|----------|---------|
| Project | 100 | Always inject |
| Tech | 90 | By file extension |
| Framework | 85 | By path/keywords |
| Module | 80 | By module path |
| Group | 70 | By member paths |
| Domain | 60 | By keyword trigger |

### Skill

```rust
use modmap::{Skill, SkillFile};

let skill = Skill::new("deploy")
    .with_description("Deploy to production")
    .with_tools(vec!["Bash".into(), "Read".into()])
    .with_instructions("Deploy using the configured CI/CD pipeline.")
    .with_file(SkillFile::new("config.yaml", "env: production"));
```

---

## Manifest

`ProjectManifest` wraps `ModuleMap` with additional metadata:

```rust
use modmap::{ProjectManifest, ModuleContext};

let manifest = ProjectManifest::new(module_map)
    .with_module_context("auth", ModuleContext::new()
        .with_rules(vec!["security-rules".into()])
        .with_skills(vec!["auth-debug".into()]));
```

---

## Core Types

### Dependencies

```rust
ModuleDependency::runtime("database")    // Runtime dependency
ModuleDependency::build("codegen")       // Build-time dependency
ModuleDependency::test("fixtures")       // Test dependency
ModuleDependency::optional("cache")      // Optional dependency
```

### Convention & Known Issue

```rust
use modmap::{Convention, KnownIssue, IssueSeverity, IssueCategory};

let convention = Convention::new("error-handling", "Use ? operator")
    .with_rationale("Rust idiom");

let issue = KnownIssue::new(
    "race-condition",
    "Race condition in session refresh",
    IssueSeverity::High,
    IssueCategory::Concurrency,
);
```

### Tech Stack

```rust
use modmap::{TechStack, FrameworkInfo};

let stack = TechStack::new("rust")
    .with_version("1.92")
    .with_framework(FrameworkInfo::new("tokio", "async runtime"))
    .with_build_tool("cargo");
```

---

## Version Compatibility

Schema uses [SemVer](https://semver.org/). Major version must match:

```rust
use modmap::{SchemaRegistry, SchemaError};

let registry = SchemaRegistry::new();
match registry.load(&json) {
    Ok(manifest) => println!("Loaded: {}", manifest.project.project.name),
    Err(SchemaError::IncompatibleVersion { found, required_major }) => {
        eprintln!("Version {found} incompatible, requires major {required_major}");
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

---

## License

MIT
