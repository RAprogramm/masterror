<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">
  <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/materror.png" alt="masterror" width="600"/>
  <p><strong>프레임워크에 독립적인 애플리케이션 오류 타입</strong></p>

  <!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
       CI packaging will fail if README.md is stale. -->

  [![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
  [![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
  [![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
  ![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
  ![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
  [![codecov](https://codecov.io/gh/RAprogramm/masterror/graph/badge.svg?token=V9JQDTZLXH)](https://codecov.io/gh/RAprogramm/masterror)

  [![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
  [![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/masterror?branch=main)](https://hitsofcode.com/github/RAprogramm/masterror/view?branch=main)
  [![IMIR](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/assets/badges/imir-badge-simple-public.svg)](https://github.com/RAprogramm/infra-metrics-insight-renderer)

  > 🇬🇧 [Read README in English](README.md)
  > 🇷🇺 [Читайте README на русском языке](README.ru.md)

</div>

> [!IMPORTANT]
> 이 번역은 Claude를 사용하여 생성되었습니다. 오류나 부정확한 내용을 발견하시면 [이슈를 등록](https://github.com/RAprogramm/masterror/issues)해 주세요!

---

## 목차

- [개요](#개요)
- [주요 특징](#주요-특징)
- [워크스페이스 크레이트](#워크스페이스-크레이트)
- [기능 플래그](#기능-플래그)
- [설치](#설치)
- [벤치마크](#벤치마크)
- [코드 커버리지](#코드-커버리지)
- [빠른 시작](#빠른-시작)
- [고급 사용법](#고급-사용법)
- [리소스](#리소스)
- [메트릭](#메트릭)
- [라이선스](#라이선스)

---

## 개요

`masterror`는 몇 가지 헬퍼에서 시작하여 Rust 서비스 전반에 걸쳐 일관되고 관찰 가능한 오류 표면을 구축하기 위한 조합 가능한 크레이트 워크스페이스로 성장했습니다. 코어 크레이트는 프레임워크에 독립적으로 유지되며, 기능 플래그를 통해 무거운 기본값을 가져오지 않고 전송 어댑터, 통합 및 텔레메트리를 활성화합니다. `unsafe` 코드가 없고, MSRV가 고정되어 있으며, 파생 매크로를 통해 도메인 타입이 리덕션 및 메타데이터를 제어할 수 있습니다.

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 주요 특징

- **통합된 분류 체계.** `AppError`, `AppErrorKind` 및 `AppCode`는 보수적인 HTTP/gRPC 매핑, 즉시 사용 가능한 재시도/인증 힌트 및 `ProblemJson`을 통한 RFC7807 출력과 함께 도메인 및 전송 관련 사항을 모델링합니다.
- **네이티브 파생.** `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`, `#[masterror(...)]` 및 `#[provide]`는 소스, 백트레이스, 텔레메트리 프로바이더 및 리덕션 정책을 전달하면서 커스텀 타입을 `AppError`에 연결합니다.
- **타입 기반 텔레메트리.** `Metadata`는 필드별 리덕션 제어 및 `field::*`의 빌더와 함께 구조화된 키/값 컨텍스트(문자열, 정수, 부동 소수점, 기간, IP 주소 및 선택적 JSON)를 저장하므로 수동 `String` 맵 없이 로그를 구조화할 수 있습니다.
- **전송 어댑터.** 선택적 기능은 린 기본 빌드를 오염시키지 않고 Actix/Axum 응답자, `tonic::Status` 변환, WASM/브라우저 로깅 및 OpenAPI 스키마 생성을 제공합니다.
- **실전 검증된 통합.** `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`, `teloxide`, `multipart`, Telegram WebApp SDK 등을 위한 집중적인 매핑을 활성화하세요. 각각은 텔레메트리가 첨부된 분류 체계로 라이브러리 오류를 변환합니다.
- **즉시 사용 가능한 기본값.** `turnkey` 모듈은 박스에서 꺼내자마자 일관된 기준선을 원하는 팀을 위해 즉시 사용 가능한 오류 카탈로그, 헬퍼 빌더 및 추적 계측을 제공합니다.
- **타입 기반 제어 흐름 매크로.** `ensure!` 및 `fail!`은 해피 패스에서 할당이나 포매팅 없이 도메인 오류로 함수를 단락합니다.

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 워크스페이스 크레이트

| 크레이트 | 제공 기능 | 의존성 추가 시점 |
| --- | --- | --- |
| [`masterror`](https://crates.io/crates/masterror) | 코어 오류 타입, 메타데이터 빌더, 전송, 통합 및 프렐루드. | 안정적인 오류 표면을 원하는 애플리케이션 크레이트, 서비스 및 라이브러리. |
| [`masterror-derive`](masterror-derive/README.md) | `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]` 및 `#[provide]`를 지원하는 프로시저 매크로. | `masterror`를 통해 자동으로 가져옴; 매크로 해킹을 위해서만 직접 의존. |
| [`masterror-template`](masterror-template/README.md) | 포매터 분석을 위해 파생 매크로에서 사용하는 공유 템플릿 파서. | 내부 의존성; 다른 곳에서 템플릿 파서가 필요할 때 재사용. |

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 기능 플래그

필요한 것만 선택하세요; 모든 것이 기본적으로 비활성화되어 있습니다.

- **웹 전송:** `axum`, `actix`, `multipart`, `openapi`, `serde_json`.
- **텔레메트리 및 관찰성:** `tracing`, `metrics`, `backtrace`.
- **비동기 및 IO 통합:** `tokio`, `reqwest`, `sqlx`, `sqlx-migrate`, `redis`, `validator`, `config`.
- **메시징 및 봇:** `teloxide`, `telegram-webapp-sdk`.
- **프론트엔드 도구:** WASM/브라우저 콘솔 로깅을 위한 `frontend`.
- **gRPC:** `tonic::Status` 응답을 발행하기 위한 `tonic`.
- **배터리 포함:** 사전 구축된 분류 체계와 헬퍼를 채택하기 위한 `turnkey`.

빌드 스크립트는 아래의 전체 기능 스니펫을 `Cargo.toml`과 동기화 상태로 유지합니다.

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 설치

~~~toml
[dependencies]
masterror = { version = "0.24.19", default-features = false }
# 또는 기능과 함께:
# masterror = { version = "0.24.19", features = [
#   "std", "axum", "actix", "openapi",
#   "serde_json", "tracing", "metrics", "backtrace",
#   "sqlx", "sqlx-migrate", "reqwest", "redis",
#   "validator", "config", "tokio", "multipart",
#   "teloxide", "telegram-webapp-sdk", "tonic", "frontend",
#   "turnkey", "benchmarks"
# ] }
~~~

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 벤치마크

Criterion 벤치마크는 가장 핫한 변환 경로를 커버하므로 출시 전에 성능 저하를 확인할 수 있습니다. 로컬에서 실행하려면:

~~~sh
cargo bench -F benchmarks --bench error_paths
~~~

이 스위트는 두 그룹을 발행합니다:

- `context_into_error/*`는 리덕션 모드와 비리덕션 모드 모두에서 대표적인 메타데이터(문자열, 카운터, 기간, IP)가 포함된 더미 소스 오류를 `Context::into_error`를 통해 승격합니다.
- `problem_json_from_app_error/*`는 결과 `AppError` 값을 소비하여 `ProblemJson::from_app_error`를 통해 RFC 7807 페이로드를 빌드하며, 메시지 리덕션 및 필드 정책이 직렬화에 미치는 영향을 보여줍니다.

변경 사항을 조사할 때 처리량과 더 엄격한 신뢰 구간 간의 균형을 맞추기 위해 `--` 이후에 Criterion CLI 플래그(예: `--sample-size 200` 또는 `--save-baseline local`)를 조정하세요.

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 코드 커버리지

[![codecov](https://codecov.io/gh/RAprogramm/masterror/branch/main/graph/badge.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

커버리지 보고서는 모든 CI 실행에서 자동으로 생성되어 [Codecov](https://app.codecov.io/gh/RAprogramm/masterror)에 업로드됩니다. 이 프로젝트는 신뢰성을 보장하고 회귀를 조기에 포착하기 위해 모든 모듈에 걸쳐 높은 테스트 커버리지를 유지합니다.

<details>
  <summary><b>커버리지 시각화</b></summary>

#### 선버스트 그래프
가장 안쪽 원은 전체 프로젝트를 나타내며, 폴더를 거쳐 개별 파일로 바깥쪽으로 이동합니다. 크기와 색상은 문 수와 커버리지 백분율을 나타냅니다.

[![Sunburst](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/sunburst.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### 그리드 뷰
각 블록은 단일 파일을 나타냅니다. 블록 크기와 색상은 문 수와 커버리지 백분율에 해당합니다.

[![Grid](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/tree.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### 아이시클 차트
상단의 전체 프로젝트에서 시작하여 폴더를 거쳐 개별 파일로 드릴다운하는 계층적 뷰입니다. 크기와 색상은 문 수와 커버리지를 반영합니다.

[![Icicle](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/icicle.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

</details>

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 빠른 시작

<details>
  <summary><b>오류 생성</b></summary>

오류 생성:

~~~rust
use masterror::{AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
let err_with_meta = AppError::service("downstream")
    .with_field(field::str("request_id", "abc123"));
assert_eq!(err_with_meta.metadata().len(), 1);

let err_with_context = AppError::internal("db down")
    .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
assert!(err_with_context.source_ref().is_some());
~~~

프렐루드 사용:

~~~rust
use masterror::prelude::*;

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Flag must be set"));
    }
    Ok(())
}
~~~

</details>

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 고급 사용법

<details>
  <summary><b>타이핑을 희생하지 않고 빠른 실패</b></summary>

`ensure!` 및 `fail!`은 포매팅이 많은 `anyhow::ensure!`/`anyhow::bail!` 헬퍼에 대한 타입 기반 대안을 제공합니다. 가드가 트리거될 때만 오류 표현식을 평가하므로 성공 경로는 할당이 없습니다.

~~~rust
use masterror::{AppError, AppErrorKind, AppResult};

fn guard(flag: bool) -> AppResult<()> {
    masterror::ensure!(flag, AppError::bad_request("flag must be set"));
    Ok(())
}

fn bail() -> AppResult<()> {
    masterror::fail!(AppError::unauthorized("token expired"));
}

assert!(guard(true).is_ok());
assert!(matches!(guard(false).unwrap_err().kind, AppErrorKind::BadRequest));
assert!(matches!(bail().unwrap_err().kind, AppErrorKind::Unauthorized));
~~~

</details>

<details>
  <summary><b>도메인 오류 파생 및 전송에 매핑</b></summary>

`masterror`는 크레이트가 변환, 텔레메트리 및 리덕션을 처리하는 동안 도메인 타입이 표현력을 유지하도록 네이티브 파생을 제공합니다.

~~~rust
use std::io;

use masterror::Error;

#[derive(Debug, Error)]
#[error("I/O failed: {source}")]
pub struct DomainError {
    #[from]
    #[source]
    source: io::Error,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct WrappedDomainError(
    #[from]
    #[source]
    DomainError
);

fn load() -> Result<(), DomainError> {
    Err(io::Error::other("disk offline").into())
}

let err = load().unwrap_err();
assert_eq!(err.to_string(), "I/O failed: disk offline");

let wrapped = WrappedDomainError::from(err);
assert_eq!(wrapped.to_string(), "I/O failed: disk offline");
~~~

- `use masterror::Error;`는 파생 매크로를 범위로 가져옵니다.
- `#[from]`은 래퍼 형태가 유효한지 확인하면서 `From<...>`을 자동으로 구현합니다.
- `#[error(transparent)]`는 `Display`/`source`를 내부 오류로 전달하는 단일 필드 래퍼를 강제합니다.
- `#[app_error(kind = AppErrorKind::..., code = AppCode::..., message)]`는 파생된 오류를 `AppError`/`AppCode`에 매핑합니다. 선택적 `code = ...` 암은 `AppCode` 변환을 발행하고, `message` 플래그는 베어 오류를 생성하는 대신 파생된 `Display` 출력을 공개 메시지로 전달합니다.
- `masterror::error::template::ErrorTemplate`는 `#[error("...")]` 문자열을 파싱하여 리터럴 및 플레이스홀더 세그먼트를 노출하므로 `thiserror`에 의존하지 않고 커스텀 파생을 구현할 수 있습니다.
- `TemplateFormatter`는 `thiserror`의 포매터 감지를 미러링하므로 16진수, 포인터 또는 지수 렌더러에 의존하는 기존 파생이 계속 컴파일됩니다.
- Display 플레이스홀더는 `TemplateFormatter::display_spec()` 및 `TemplateFormatter::format_fragment()`를 통해 원시 포맷 스펙을 보존하므로 파생된 코드는 원본 문자열을 재구성하지 않고 `:>8`, `:.3` 및 기타 디스플레이 전용 옵션을 전달할 수 있습니다.
- `TemplateFormatterKind`는 플레이스홀더가 요청한 포매터 트레이트를 노출하므로 모든 열거형 변형을 수동으로 일치시키지 않고도 요청된 렌더링 동작을 쉽게 분기할 수 있습니다.

</details>

<details>
  <summary><b>텔레메트리, 리덕션 정책 및 변환 첨부</b></summary>

`#[derive(Masterror)]`는 도메인 오류를 [`masterror::Error`]에 연결하고, 메타데이터, 리덕션 정책 및 선택적 전송 매핑을 추가합니다. 동반되는 `#[masterror(...)]` 속성은 텔레메트리 및 리덕션에 대해 명시적으로 유지하면서 `#[app_error]` 구문을 미러링합니다.

~~~rust
use masterror::{
    mapping::HttpMapping, AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy
};

#[derive(Debug, Masterror)]
#[error("user {user_id} missing flag {flag}")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    redact(message, fields("user_id" = hash)),
    telemetry(
        Some(masterror::field::str("user_id", user_id.clone())),
        attempt.map(|value| masterror::field::u64("attempt", value))
    ),
    map.grpc = 5,
    map.problem = "https://errors.example.com/not-found"
)]
struct MissingFlag {
    user_id: String,
    flag: &'static str,
    attempt: Option<u64>,
    #[source]
    source: Option<std::io::Error>
}

let err = MissingFlag {
    user_id: "alice".into(),
    flag: "beta",
    attempt: Some(2),
    source: None
};
let converted: Error = err.into();
assert_eq!(converted.code, AppCode::NotFound);
assert_eq!(converted.kind, AppErrorKind::NotFound);
assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
assert!(converted.metadata().get("user_id").is_some());

assert_eq!(
    MissingFlag::HTTP_MAPPING,
    HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
);
~~~

- `code` / `category`는 공개 [`AppCode`] 및 내부 [`AppErrorKind`]를 선택합니다.
- `message`는 포맷된 [`Display`] 출력을 안전한 공개 메시지로 전달합니다. 메시지를 비공개로 유지하려면 생략하세요.
- `redact(message)`는 전송 경계에서 [`MessageEditPolicy`]를 리덕션 가능으로 전환하고, `fields("name" = hash, "card" = last4)`는 메타데이터 정책(`hash`, `last4`, `redact`, `none`)을 재정의합니다.
- `telemetry(...)`는 `Option<masterror::Field>`로 평가되는 표현식을 허용합니다. 각 채워진 필드는 결과 [`Metadata`]에 삽입됩니다; 필드가 첨부되지 않은 경우 `telemetry()`를 사용하세요.
- `map.grpc` / `map.problem`는 선택적 gRPC 상태 코드(`i32`로) 및 RFC 7807 `type` URI를 캡처합니다. 파생은 다운스트림 통합을 위해 `MyError::HTTP_MAPPING`, `MyError::GRPC_MAPPING` 및 `MyError::PROBLEM_MAPPING`(또는 열거형의 경우 슬라이스 변형)과 같은 테이블을 발행합니다.

모든 익숙한 필드 수준 속성(`#[from]`, `#[source]`, `#[backtrace]`)은 여전히 존중됩니다. 소스 및 백트레이스는 생성된 [`masterror::Error`]에 자동으로 첨부됩니다.

</details>

<details>
  <summary><b>구조화된 텔레메트리 프로바이더 및 AppError 매핑</b></summary>

`#[provide(...)]`는 `std::error::Request`를 통해 타입 기반 컨텍스트를 노출하고, `#[app_error(...)]`는 도메인 오류가 `AppError` 및 `AppCode`로 변환되는 방법을 기록합니다. 파생은 `thiserror`의 구문을 미러링하고 선택적 텔레메트리 전파 및 `masterror` 런타임 타입으로의 직접 변환으로 확장합니다.

~~~rust
use std::error::request_ref;

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TelemetrySnapshot {
    name:  &'static str,
    value: u64,
}

#[derive(Debug, Error)]
#[error("structured telemetry {snapshot:?}")]
#[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
struct StructuredTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot,
}

let err = StructuredTelemetryError {
    snapshot: TelemetrySnapshot {
        name: "db.query",
        value: 42,
    },
};

let snapshot = request_ref::<TelemetrySnapshot>(&err).expect("telemetry");
assert_eq!(snapshot.value, 42);

let app: AppError = err.into();
let via_app = request_ref::<TelemetrySnapshot>(&app).expect("telemetry");
assert_eq!(via_app.name, "db.query");
~~~

선택적 텔레메트리는 존재할 때만 표시되므로 `None`은 프로바이더를 등록하지 않습니다. 호출자가 소유권을 요청할 때 소유된 스냅샷을 여전히 값으로 제공할 수 있습니다:

~~~rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("optional telemetry {telemetry:?}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal)]
struct OptionalTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    telemetry: Option<TelemetrySnapshot>,
}

let noisy = OptionalTelemetryError {
    telemetry: Some(TelemetrySnapshot {
        name: "queue.depth",
        value: 17,
    }),
};
let silent = OptionalTelemetryError { telemetry: None };

assert!(request_ref::<TelemetrySnapshot>(&noisy).is_some());
assert!(request_ref::<TelemetrySnapshot>(&silent).is_none());
~~~

열거형은 변형별 텔레메트리 및 변환 메타데이터를 지원합니다. 각 변형은 자체 `AppErrorKind`/`AppCode` 매핑을 선택하고 파생은 단일 `From<Enum>` 구현을 생성합니다:

~~~rust
#[derive(Debug, Error)]
enum EnumTelemetryError {
    #[error("named {label}")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound)]
    Named {
        label:    &'static str,
        #[provide(ref = TelemetrySnapshot)]
        snapshot: TelemetrySnapshot,
    },
    #[error("optional tuple")]
    #[app_error(kind = AppErrorKind::Timeout, code = AppCode::Timeout)]
    Optional(#[provide(ref = TelemetrySnapshot)] Option<TelemetrySnapshot>),
    #[error("owned tuple")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Owned(#[provide(value = TelemetrySnapshot)] TelemetrySnapshot),
}

let owned = EnumTelemetryError::Owned(TelemetrySnapshot {
    name: "redis.latency",
    value: 3,
});
let app: AppError = owned.into();
assert!(matches!(app.kind, AppErrorKind::Service));
~~~

`thiserror`와 비교하여 익숙한 파생 표면을 유지하면서 수동 글루 없이 구조화된 텔레메트리(`#[provide]`) 및 `AppError`/`AppCode`로의 일급 변환을 얻습니다.

</details>

<details>
  <summary><b>Problem JSON 페이로드 및 재시도/인증 힌트</b></summary>

~~~rust
use masterror::{AppError, AppErrorKind, ProblemJson};
use std::time::Duration;

let problem = ProblemJson::from_app_error(
    AppError::new(AppErrorKind::Unauthorized, "Token expired")
        .with_retry_after_duration(Duration::from_secs(30))
        .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
);

assert_eq!(problem.status, 401);
assert_eq!(problem.retry_after, Some(30));
assert_eq!(problem.grpc.expect("grpc").name, "UNAUTHENTICATED");
~~~

</details>

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 리소스

- 단계별 가이드, `thiserror`/`anyhow`와의 비교 및 문제 해결 레시피는 [오류 처리 위키](https://github.com/RAprogramm/masterror/wiki)를 참조하세요.
- API 세부 정보, 기능별 가이드 및 전송 테이블은 [docs.rs의 크레이트 문서](https://docs.rs/masterror)를 찾아보세요.
- 릴리스 하이라이트 및 마이그레이션 노트는 [`CHANGELOG.md`](CHANGELOG.md)를 확인하세요.
- 이 프로젝트가 따르는 개발 표준 및 모범 사례는 [RustManifest](https://github.com/RAprogramm/RustManifest)를 검토하세요.

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 메트릭

![Metrics](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/metrics/masterror.svg)

<div align="right">

<div align="right">
  <a href="#목차">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="맨 위로" width="50"/>
  </a>
</div>

</div>

---

## 라이선스

MSRV: **1.90** · License: **MIT OR Apache-2.0** · `unsafe` 없음


