# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

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

Language/framework-agnostic schema definitions that can be used by any tool requiring structured codebase representation.

## Schema Structure

```
types.rs
├── DependencyType        # Runtime | Build | Test | Optional
├── ModuleDependency      # Typed dependency with factory methods
├── DetectedLanguage      # Builder pattern
├── EvidenceLocation      # new(), new_range()
├── WorkspaceType         # SinglePackage | Monorepo | Microservices | MultiPackage
└── is_path_in_scope()    # Scope boundary check

module_map.rs
├── ModuleMap             # Root schema
├── ProjectMetadata       # + conventions (project-wide rules)
├── Module                # + dependencies: Vec<ModuleDependency>
├── ModuleMetrics         # priority_score() = value*0.6 + risk*0.4
└── ModuleGroup           # max_agents_hint, leader_module

registry.rs
├── SchemaError           # Version/JSON parse errors
└── SchemaRegistry        # Version-validated JSON loading
```

## Key Patterns

- `#[serde(default)]` on structs with `Default` derive
- `#[serde(flatten)]` for `ModuleMetrics` in `Module`
- Factory methods: `new()`, `runtime()`, `build()`, `test()`, `optional()`
- JSON Schema generation via `schemars::JsonSchema`

## Design Constraint

All types must work universally across all languages, frameworks, and project structures.
