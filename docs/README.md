# 문서 가이드

이 디렉토리는 프로젝트의 모든 문서를 관리한다.

---

## 1. 디렉토리 구조

```
docs/
├── README.md                 # 이 문서
├── api_spec.md               # API 명세서
├── error_spec.md             # 에러 명세서
├── version_change_log.md     # 버전 변경 이력
├── core/                     # 핵심 규칙 및 템플릿 (수정 거의 없음)
├── project/                  # 프로젝트 개념/아키텍처 문서 (큰 틀)
└── sdd/                      # 스펙 주도 개발 문서 (작은 단위)
    ├── {브랜치 식별자}/      # 브랜치별 작업 스펙
    └── history/              # 완료된 작업 보관
```

---

## 2. 문서 설명

| 문서/디렉토리 | 설명 | 수정 빈도 |
|:-----|:-----|:-----|
| `api_spec.md` | 프로젝트 API 엔드포인트 명세 | 기능 추가 시 |
| `error_spec.md` | 프로젝트 에러 코드 및 처리 방법 | 에러 추가 시 |
| `version_change_log.md` | 버전별 변경 이력 | 릴리즈 시 |
| `core/` | 프로젝트 규칙, 컨벤션, 템플릿 문서 | 거의 없음 |
| `project/` | 프로젝트 개념, 아키텍처, 설계 문서 (큰 틀) | 설계 변경 시 |
| `sdd/` | 스펙 주도 개발 문서 (작은 단위, Claude에게 제공) | 매 개발 사이클 |

---

## 3. 상세 설명

### 3.1 api_spec.md

프로젝트에서 구현한 API 엔드포인트를 정의한다.

- Notion 기능 정의서의 기능 ID와 매핑
- 요청/응답 형식, 에러 코드 포함
- 템플릿: `core/api_spec_template.md`

### 3.2 error_spec.md

프로젝트에서 처리하는 에러를 정의한다.

- Notion 에러 명세서의 에러 ID와 매핑
- 에러 상황, 처리 코드 포함
- 템플릿: `core/error_spec_template.md`

### 3.3 version_change_log.md

버전별 변경 사항을 기록한다.

- Notion 버전 계획서와 매핑
- 릴리즈 시 업데이트
- 템플릿: `core/version_change_log_template.md`

### 3.4 core/

프로젝트의 핵심 규칙과 템플릿을 포함한다.

- **수정이 거의 일어나지 않는 문서**를 관리
- 프로젝트 규칙, CI/CD 워크플로우 등
- 언어별 TDD 방법론: Python, Java, TypeScript, Rust, JavaScript
- 상세: [core/README.md](./core/README.md)

### 3.5 project/

프로젝트의 **큰 틀**을 정의하는 개념적/아키텍처 문서를 포함한다.

- 전체 시스템 설계, 도메인 모델, 기술 스택 결정 등
- `sdd/`의 개별 작업이 참조하는 상위 레벨 문서
- 상세: [project/README.md](./project/README.md)

### 3.6 sdd/

**스펙 주도 개발(Spec-Driven Development)** 문서를 관리한다.

- `{브랜치 식별자}/`: 브랜치별 작업 스펙 디렉토리
  - Claude에게 제공하여 코드 생성 요청
  - 작업 완료 후 `report.md` 생성
- `history/`: 완료된 작업 보관
- `project/`의 큰 틀을 기반으로 **작은 단위**의 구체적인 스펙 작성
- 상세: [sdd/README.md](./sdd/README.md)

---

## 4. MCP 연결 설정

프로젝트에서 Notion과 Jira를 Claude와 연동하여 사용하려면 아래 MCP 설정이 필요하다.

### 4.1 Notion 연결

```bash
claude mcp add notion -e OPENAPI_MCP_HEADERS='{"Authorization":"Bearer ntn_**","Notion-Version":"2022-06-28"}' -- npx -y @notionhq/notion-mcp-server
```

- `ntn_**` 부분에 실제 Notion API 토큰을 입력한다.

### 4.2 Jira 연결

```bash
claude mcp add atlassian -t sse https://mcp.atlassian.com/v1/sse
```

- Atlassian SSE 방식으로 연결한다.

---

## 5. 참조

- [Core 문서 가이드](./core/README.md)
- [Project 문서 가이드](./project/README.md)
- [SDD 문서 가이드](./sdd/README.md)

---
