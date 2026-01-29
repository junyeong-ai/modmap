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

`modmap` is a language-agnostic schema library for representing codebase structure. It provides standardized types for describing modules, dependencies, conventions, and known issues across any programming language or framework.

### Use Cases

- **Multi-agent systems**: Define domain boundaries for agent coordination
- **Code analysis tools**: Structured output format for codebase analyzers
- **Documentation generators**: Machine-readable module descriptions
- **CI/CD pipelines**: Automated codebase structure validation

---

## Installation

```toml
[dependencies]
modmap = "1.0"
```

---

## Quick Start

### Creating a Module Map

```rust
use modmap::{
    GeneratorInfo, ModuleMap, ModuleGroup, ProjectMetadata,
    TechStack, Module, ModuleMetrics, ModuleDependency,
};

// Define project metadata
let project = ProjectMetadata::new("my-project", TechStack::new("rust").with_version("1.92"))
    .with_description("Example project")
    .with_total_files(50);

// Define modules
let modules = vec![
    Module {
        id: "auth".into(),
        name: "auth".into(),
        paths: vec!["src/auth/".into()],
        key_files: vec!["src/auth/mod.rs".into()],
        dependencies: vec![ModuleDependency::runtime("db")],
        dependents: vec!["api".into()],
        responsibility: "User authentication and session management".into(),
        primary_language: "rust".into(),
        metrics: ModuleMetrics::new(0.85, 0.9, 0.3),
        conventions: vec![],
        known_issues: vec![],
        evidence: vec![],
    },
];

// Create module map
let generator = GeneratorInfo::new("my-analyzer", "1.0.0");
let map = ModuleMap::new(generator, project, modules, vec![]);

// Serialize to JSON
let json = map.to_json()?;
```

### Loading with Version Validation

```rust
use modmap::SchemaRegistry;

let registry = SchemaRegistry::new();
let map = registry.load_module_map(&json_string)?;

println!("Project: {}", map.project.name);
println!("Modules: {}", map.modules.len());
```

---

## Schema Structure

### Root Schema

```
ModuleMap
├── schema_version: "3.0.0"
├── generator: GeneratorInfo
├── project: ProjectMetadata
│   ├── name, description, repository
│   ├── project_type: Application | Library | Service | Cli
│   ├── workspace: WorkspaceInfo
│   ├── tech_stack: TechStack
│   ├── languages: Vec<DetectedLanguage>
│   └── commands: ProjectCommands
├── modules: Vec<Module>
├── groups: Vec<ModuleGroup>
├── dependency_graph: Option<DependencyGraph>
└── generated_at: DateTime<Utc>
```

### Module

```rust
pub struct Module {
    pub id: String,
    pub name: String,
    pub paths: Vec<String>,
    pub key_files: Vec<String>,
    pub dependencies: Vec<ModuleDependency>,
    pub dependents: Vec<String>,
    pub responsibility: String,
    pub primary_language: String,
    pub metrics: ModuleMetrics,        // Flattened in JSON
    pub conventions: Vec<Convention>,
    pub known_issues: Vec<KnownIssue>,
    pub evidence: Vec<EvidenceLocation>,
}
```

### Module Metrics

```rust
let metrics = ModuleMetrics::new(
    0.85,  // coverage_ratio
    0.9,   // value_score
    0.3,   // risk_score
);

// Priority calculation: value * 0.6 + risk * 0.4
let priority = metrics.priority_score();  // 0.66
```

---

## Type Reference

### Enums

| Type | Variants |
|------|----------|
| `WorkspaceType` | SinglePackage, Monorepo, Microservices, MultiPackage |
| `ProjectType` | Application, Library, Service, Cli |
| `DependencyType` | Runtime, Build, Test, Optional |
| `IssueSeverity` | Critical, High, Medium, Low |
| `IssueCategory` | Security, Performance, Correctness, Maintainability, Concurrency, Compatibility |

### Dependency Factory Methods

```rust
ModuleDependency::new("module-id")       // Default (Runtime)
ModuleDependency::runtime("database")    // Runtime dependency
ModuleDependency::build("codegen")       // Build-time dependency
ModuleDependency::test("fixtures")       // Test dependency
ModuleDependency::optional("cache")      // Optional dependency
```

### Convention & Known Issue

```rust
use modmap::{Convention, KnownIssue, IssueSeverity, IssueCategory, EvidenceLocation};

let convention = Convention::new("error-handling", "Use ? operator for propagation")
    .with_rationale("Rust idiom for error handling")
    .with_evidence(vec![EvidenceLocation::new("src/lib.rs", 42)]);

let issue = KnownIssue::new(
    "race-condition",
    "Race condition in session refresh",
    IssueSeverity::High,
    IssueCategory::Concurrency,
)
.with_prevention("Use atomic CAS operations")
.with_evidence(vec![EvidenceLocation::new_range("src/session.rs", 128, 145)]);
```

### Tech Stack

```rust
use modmap::{TechStack, FrameworkInfo, LibraryInfo};

let stack = TechStack::new("rust")
    .with_version("1.92")
    .with_framework(FrameworkInfo::new("tokio", "async runtime").with_version("1.0"))
    .with_build_tool("cargo")
    .with_test_framework("built-in")
    .with_library(LibraryInfo::new("serde", "serialization"));
```

### Evidence Location

```rust
use modmap::EvidenceLocation;

let single = EvidenceLocation::new("src/main.rs", 42);
single.to_reference()  // "src/main.rs:42"

let range = EvidenceLocation::new_range("src/lib.rs", 10, 20);
range.to_reference()  // "src/lib.rs:10-20"
```

---

## Module Groups

Group related modules with boundary rules:

```rust
use modmap::ModuleGroup;

let group = ModuleGroup::new("core", "Core Domain", vec!["auth".into(), "db".into()])
    .with_responsibility("Core business logic")
    .with_boundary_rules(vec!["No direct CLI dependencies".into()]);
```

---

## Dependency Graph

Optional architecture layer representation:

```rust
use modmap::{DependencyGraph, DependencyEdge, ArchitectureLayer, DependencyType};

let graph = DependencyGraph {
    edges: vec![
        DependencyEdge {
            from: "api".into(),
            to: "auth".into(),
            edge_type: DependencyType::Runtime,
        },
    ],
    layers: vec![
        ArchitectureLayer {
            name: "presentation".into(),
            modules: vec!["cli".into(), "api".into()],
        },
        ArchitectureLayer {
            name: "domain".into(),
            modules: vec!["auth".into(), "db".into()],
        },
    ],
};

let map = ModuleMap::new(generator, project, modules, groups)
    .with_dependency_graph(graph);
```

---

## Version Compatibility

Schema uses [SemVer](https://semver.org/). The `SchemaRegistry` validates major version compatibility:

```rust
use modmap::{SchemaRegistry, SchemaError};

let registry = SchemaRegistry::new();

// Current version
println!("Schema version: {}", registry.version());  // 3.x.x

// Load and validate
match registry.load_module_map(&json) {
    Ok(map) => println!("Loaded: {}", map.project.name),
    Err(SchemaError::IncompatibleVersion { found, minimum }) => {
        eprintln!("Version {found} incompatible, requires {minimum}");
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

---

## JSON Schema

All types derive `schemars::JsonSchema` for JSON Schema generation:

```rust
use schemars::schema_for;
use modmap::ModuleMap;

let schema = schema_for!(ModuleMap);
println!("{}", serde_json::to_string_pretty(&schema)?);
```

---

## Serialization

Optimized serde attributes for clean JSON output:

- `#[serde(default)]` - Default values on deserialization
- `#[serde(skip_serializing_if = "Vec::is_empty")]` - Omit empty collections
- `#[serde(flatten)]` - Flatten `ModuleMetrics` into `Module`

---

## License

MIT
