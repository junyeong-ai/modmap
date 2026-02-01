# modmap

**코드베이스 구조 표현을 위한 범용 모듈 맵 스키마**

[![CI](https://github.com/junyeong-ai/modmap/actions/workflows/ci.yml/badge.svg)](https://github.com/junyeong-ai/modmap/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/modmap.svg)](https://crates.io/crates/modmap)
[![Docs.rs](https://img.shields.io/docsrs/modmap)](https://docs.rs/modmap)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/crates/l/modmap.svg)](LICENSE)

[English](README.md) | 한국어

---

## 개요

`modmap`은 코드베이스 구조를 표현하기 위한 언어 무관 스키마 라이브러리입니다. 다음을 위한 표준화된 타입을 제공합니다:

1. **코드베이스 분석**: 모듈, 의존성, 컨벤션, 알려진 이슈
2. **플러그인 시스템**: Claude Code 플러그인용 에이전트, 규칙, 스킬

---

## 설치

```toml
[dependencies]
modmap = "1.1"
```

---

## 모듈 레퍼런스

| 모듈 | 설명 | 주요 타입 |
|-----|------|----------|
| `module_map` | 핵심 스키마 | ModuleMap, Module, ModuleGroup, Domain |
| `types` | 기본 타입 | DependencyType, Convention, KnownIssue, TechStack |
| `manifest` | 프로젝트 매니페스트 | ProjectManifest, ModuleContext |
| `agent` | 에이전트 정의 | Agent, AgentModel, AgentColor |
| `rule` | 규칙 정의 | Rule, RuleCategory |
| `skill` | 스킬 정의 | Skill, SkillFile, ContextMode |
| `registry` | 버전 검증 | SchemaRegistry, SchemaError |

---

## 빠른 시작

### 모듈 맵 생성

```rust
use modmap::{
    GeneratorInfo, ModuleMap, ProjectMetadata,
    TechStack, Module, ModuleMetrics, ModuleDependency,
};

let project = ProjectMetadata::new("my-project", TechStack::new("rust"))
    .with_description("예제 프로젝트");

let modules = vec![Module {
    id: "auth".into(),
    name: "인증".into(),
    paths: vec!["src/auth/".into()],
    dependencies: vec![ModuleDependency::runtime("db")],
    responsibility: "사용자 인증".into(),
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

### 매니페스트로 로드

```rust
use modmap::SchemaRegistry;

let registry = SchemaRegistry::new();
let manifest = registry.load(&json_string)?;

println!("프로젝트: {}", manifest.project.project.name);
```

---

## 플러그인 타입

### 에이전트 (Agent)

```rust
use modmap::{Agent, AgentModel, AgentColor};

let agent = Agent::new(
    "code-reviewer",
    "베스트 프랙티스를 위한 코드 리뷰",
    "보안과 성능에 집중하여 코드를 리뷰합니다.",
)
.with_model(AgentModel::Haiku)
.with_color(AgentColor::Green)
.with_tools(vec!["Read".into(), "Grep".into(), "Glob".into()]);
```

### 규칙 (Rule)

```rust
use modmap::{Rule, RuleCategory};

let rule = Rule::new(
    "rust-conventions",
    vec!["에러 전파에 `?` 연산자 사용.".into()],
)
.with_category(RuleCategory::Tech)
.with_paths(vec!["**/*.rs".into()]);
```

**카테고리별 규칙 우선순위:**

| 카테고리 | 우선순위 | 트리거 |
|---------|---------|--------|
| Project | 100 | 항상 주입 |
| Tech | 90 | 파일 확장자별 |
| Framework | 85 | 경로/키워드별 |
| Module | 80 | 모듈 경로별 |
| Group | 70 | 멤버 경로별 |
| Domain | 60 | 키워드 트리거 |

### 스킬 (Skill)

```rust
use modmap::{Skill, SkillFile};

let skill = Skill::new(
    "deploy",
    "프로덕션 배포",
    "구성된 CI/CD 파이프라인을 사용하여 배포합니다.",
)
.with_tools(vec!["Bash".into(), "Read".into()])
.with_additional_file(SkillFile::new("config.yaml", "env: production"));
```

---

## 매니페스트

`ProjectManifest`는 `ModuleMap`을 추가 메타데이터와 함께 래핑합니다:

```rust
use modmap::{ProjectManifest, ModuleContext};
use std::collections::HashMap;

let manifest = ProjectManifest::new(module_map)
    .with_modules(HashMap::from([(
        "auth".to_string(),
        ModuleContext::new()
            .with_rules(vec!["security-rules".into()])
            .with_skills(vec!["auth-debug".into()]),
    )]));

---

## 핵심 타입

### 의존성

```rust
ModuleDependency::runtime("database")    // 런타임 의존성
ModuleDependency::build("codegen")       // 빌드 타임 의존성
ModuleDependency::test("fixtures")       // 테스트 의존성
ModuleDependency::optional("cache")      // 선택적 의존성
```

### 컨벤션 & 알려진 이슈

```rust
use modmap::{Convention, KnownIssue, IssueSeverity, IssueCategory};

let convention = Convention::new("error-handling", "? 연산자 사용")
    .with_rationale("Rust 관용구");

let issue = KnownIssue::new(
    "race-condition",
    "세션 갱신 시 레이스 컨디션",
    IssueSeverity::High,
    IssueCategory::Concurrency,
);
```

### 기술 스택

```rust
use modmap::{TechStack, FrameworkInfo};

let stack = TechStack::new("rust")
    .with_version("1.92")
    .with_framework(FrameworkInfo::new("tokio", "async 런타임"))
    .with_build_tool("cargo");
```

---

## 버전 호환성

스키마는 [SemVer](https://semver.org/)를 사용합니다. Major 버전이 일치해야 합니다:

```rust
use modmap::{SchemaRegistry, SchemaError};

let registry = SchemaRegistry::new();
match registry.load(&json) {
    Ok(manifest) => println!("로드됨: {}", manifest.project.project.name),
    Err(SchemaError::IncompatibleVersion { found, required_major }) => {
        eprintln!("버전 {found} 비호환, major {required_major} 필요");
    }
    Err(e) => eprintln!("에러: {e}"),
}
```

---

## 라이선스

MIT
