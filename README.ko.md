# modmap

**코드베이스 구조 표현을 위한 범용 모듈 맵 스키마**

[![Crates.io](https://img.shields.io/crates/v/modmap.svg)](https://crates.io/crates/modmap)
[![Docs.rs](https://img.shields.io/docsrs/modmap)](https://docs.rs/modmap)
[![Rust](https://img.shields.io/badge/rust-1.92%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/crates/l/modmap.svg)](LICENSE)

[English](README.md) | 한국어

---

## 개요

`modmap`은 코드베이스 구조를 표현하기 위한 언어 무관 스키마 라이브러리입니다. 모든 프로그래밍 언어와 프레임워크에서 모듈, 의존성, 컨벤션, 알려진 이슈를 설명하기 위한 표준화된 타입을 제공합니다.

### 사용 사례

- **멀티 에이전트 시스템**: 에이전트 조율을 위한 도메인 경계 정의
- **코드 분석 도구**: 코드베이스 분석기의 구조화된 출력 형식
- **문서 생성기**: 기계 판독 가능한 모듈 설명
- **CI/CD 파이프라인**: 자동화된 코드베이스 구조 검증

---

## 설치

```toml
[dependencies]
modmap = "1.0"
```

---

## 빠른 시작

### 모듈 맵 생성

```rust
use modmap::{
    GeneratorInfo, ModuleMap, ModuleGroup, ProjectMetadata,
    TechStack, Module, ModuleMetrics, ModuleDependency,
};

// 프로젝트 메타데이터 정의
let project = ProjectMetadata::new("my-project", TechStack::new("rust").with_version("1.92"))
    .with_description("예제 프로젝트")
    .with_total_files(50);

// 모듈 정의
let modules = vec![
    Module {
        id: "auth".into(),
        name: "auth".into(),
        paths: vec!["src/auth/".into()],
        key_files: vec!["src/auth/mod.rs".into()],
        dependencies: vec![ModuleDependency::runtime("db")],
        dependents: vec!["api".into()],
        responsibility: "사용자 인증 및 세션 관리".into(),
        primary_language: "rust".into(),
        metrics: ModuleMetrics::new(0.85, 0.9, 0.3),
        conventions: vec![],
        known_issues: vec![],
        evidence: vec![],
    },
];

// 모듈 맵 생성
let generator = GeneratorInfo::new("my-analyzer", "1.0.0");
let map = ModuleMap::new(generator, project, modules, vec![]);

// JSON으로 직렬화
let json = map.to_json()?;
```

### 버전 검증과 함께 로드

```rust
use modmap::SchemaRegistry;

let registry = SchemaRegistry::new();
let map = registry.load_module_map(&json_string)?;

println!("프로젝트: {}", map.project.name);
println!("모듈: {}", map.modules.len());
```

---

## 스키마 구조

### 루트 스키마

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

### 모듈

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
    pub metrics: ModuleMetrics,        // JSON에서 평탄화됨
    pub conventions: Vec<Convention>,
    pub known_issues: Vec<KnownIssue>,
    pub evidence: Vec<EvidenceLocation>,
}
```

### 모듈 메트릭

```rust
let metrics = ModuleMetrics::new(
    0.85,  // coverage_ratio
    0.9,   // value_score
    0.3,   // risk_score
);

// 우선순위 계산: value * 0.6 + risk * 0.4
let priority = metrics.priority_score();  // 0.66
```

---

## 타입 레퍼런스

### Enum

| 타입 | 값 |
|------|-----|
| `WorkspaceType` | SinglePackage, Monorepo, Microservices, MultiPackage |
| `ProjectType` | Application, Library, Service, Cli |
| `DependencyType` | Runtime, Build, Test, Optional |
| `IssueSeverity` | Critical, High, Medium, Low |
| `IssueCategory` | Security, Performance, Correctness, Maintainability, Concurrency, Compatibility |

### 의존성 팩토리 메소드

```rust
ModuleDependency::new("module-id")       // 기본값 (Runtime)
ModuleDependency::runtime("database")    // 런타임 의존성
ModuleDependency::build("codegen")       // 빌드 타임 의존성
ModuleDependency::test("fixtures")       // 테스트 의존성
ModuleDependency::optional("cache")      // 선택적 의존성
```

### Convention & Known Issue

```rust
use modmap::{Convention, KnownIssue, IssueSeverity, IssueCategory, EvidenceLocation};

let convention = Convention::new("error-handling", "에러 전파에 ? 연산자 사용")
    .with_rationale("Rust 에러 처리 관용구")
    .with_evidence(vec![EvidenceLocation::new("src/lib.rs", 42)]);

let issue = KnownIssue::new(
    "race-condition",
    "세션 갱신 시 레이스 컨디션",
    IssueSeverity::High,
    IssueCategory::Concurrency,
)
.with_prevention("원자적 CAS 연산 사용")
.with_evidence(vec![EvidenceLocation::new_range("src/session.rs", 128, 145)]);
```

### Tech Stack

```rust
use modmap::{TechStack, FrameworkInfo, LibraryInfo};

let stack = TechStack::new("rust")
    .with_version("1.92")
    .with_framework(FrameworkInfo::new("tokio", "async 런타임").with_version("1.0"))
    .with_build_tool("cargo")
    .with_test_framework("built-in")
    .with_library(LibraryInfo::new("serde", "직렬화"));
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

## 모듈 그룹

경계 규칙과 함께 관련 모듈을 그룹화:

```rust
use modmap::ModuleGroup;

let group = ModuleGroup::new("core", "Core Domain", vec!["auth".into(), "db".into()])
    .with_responsibility("핵심 비즈니스 로직")
    .with_boundary_rules(vec!["CLI 직접 의존 금지".into()]);
```

---

## 의존성 그래프

선택적 아키텍처 레이어 표현:

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

## 버전 호환성

스키마는 [SemVer](https://semver.org/)를 사용합니다. `SchemaRegistry`가 major 버전 호환성을 검증합니다:

```rust
use modmap::{SchemaRegistry, SchemaError};

let registry = SchemaRegistry::new();

// 현재 버전
println!("스키마 버전: {}", registry.version());  // 3.x.x

// 로드 및 검증
match registry.load_module_map(&json) {
    Ok(map) => println!("로드됨: {}", map.project.name),
    Err(SchemaError::IncompatibleVersion { found, minimum }) => {
        eprintln!("버전 {found} 비호환, {minimum} 필요");
    }
    Err(e) => eprintln!("에러: {e}"),
}
```

---

## JSON Schema

모든 타입이 JSON Schema 생성을 위해 `schemars::JsonSchema`를 derive합니다:

```rust
use schemars::schema_for;
use modmap::ModuleMap;

let schema = schema_for!(ModuleMap);
println!("{}", serde_json::to_string_pretty(&schema)?);
```

---

## 직렬화

깔끔한 JSON 출력을 위한 최적화된 serde 속성:

- `#[serde(default)]` - 역직렬화 시 기본값
- `#[serde(skip_serializing_if = "Vec::is_empty")]` - 빈 컬렉션 생략
- `#[serde(flatten)]` - `ModuleMetrics`를 `Module`에 평탄화

---

## 라이선스

MIT
