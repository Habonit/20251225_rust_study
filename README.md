# Rust 프로그래밍 학습

## 강의 정보

**강의명**: [세상에서 제일 쉬운 러스트 프로그래밍 강의](https://www.inflearn.com/course/%EC%A0%9C%EC%9D%BC-%EC%89%AC%EC%9A%B4-%EB%9F%AC%EC%8A%A4%ED%8A%B8/dashboard)

**플랫폼**: Inflearn

**목표**: Rust 언어의 기초부터 실전까지 체계적으로 학습하여 시스템 프로그래밍 역량 강화

---

## 학습 진도

| 챕터 | 주제 | 상태 |
|:-----|:-----|:-----|
| Chapter 2 | 출력, 변수, 타입 | ✅ 완료 |
| Chapter 3 | 함수 | ✅ 완료 |
| Chapter 4 | 조건문, 반복문 | ✅ 완료 |
| Chapter 5 | 소유권 | ✅ 완료 |
| Chapter 6 | 열거형과 패턴 매칭 | ✅ 완료 |
| Chapter 7 | 구조체와 트레이트 | ✅ 완료 |

**현재 진도**: Chapter 7 / 전체 강의 (진행 중)

---

## 학습 기록

| 날짜 | 학습 내용 |
|:-----|:---------|
| 2025-12-25 | Chapter 2 ~ Chapter 5 (출력, 변수, 타입, 함수, 조건문, 반복문, 소유권) |
| 2025-12-27 | Chapter 6 ~ Chapter 7 (열거형, 패턴 매칭, 구조체, 트레이트) |

---

## 프로젝트 구조

```
material/
├── pdf/           # 강의 PDF 자료
├── chapter_2/     # 출력, 변수, 타입
├── chapter_3/     # 함수
├── chapter_4/     # 조건문, 반복문
├── chapter_5/     # 소유권
├── chapter_6/     # 열거형과 패턴 매칭
└── chapter_7/     # 구조체와 트레이트
```

각 챕터별 퀴즈 파일을 직접 풀어보며 학습합니다.

---

# 프로젝트 탬플릿 버전 관리 
- autocommit은 사용하려는 언어와 api, 모델을 가지고 하나만 사용하고 나머지는 삭제해도 무방합니다.
- 클로드 코드를 사용하는 경우, docs/core/constitution.md, docs/core/ci_cd.md, docs/core/style.md 를 기반으로 CLAUDE.md를 작성해주세요. 또한 아래 하위 문서는 참조로 걸어주세요. 
    - 또한 docs/core/tdd_constitution_java.md
    - docs/core/tdd_constitution_javascript.md
    - docs/core/tdd_constitution_python.md
    - docs/core/tdd_constitution_rust.md
    - docs/core/tdd_constitution_typescript.md
    - docs/core/review_before_merge.md
    - docs/core/review_before_push.md
    - docs/core/workflow_template_java.yaml
    - docs/core/workflow_template_javascript.yaml
    - docs/core/workflow_template_rust.yaml
    - docs/core/workflow_template_python.yaml
    - docs/core/workflow_template_typescript.yaml

## version/0.*

- initial_docs/*에서 작성된 것들을 ai 기반으로 합친 버전입니다.
- v0.1.0: 초기 버전 취합
- v0.2.0: docs/project/ 프로젝트 문서를 넣을 수 있는 디렉토리 설정
- v0.3.0: Rust/JavaScript 언어 지원 및 개발 철학 강화
  - Rust, JavaScript TDD 방법론 문서 추가 (`tdd_constitution_rust.md`, `tdd_constitution_javascript.md`)
  - Rust, JavaScript 코드 스타일 가이드 추가 (`constitution.md` Section 6, 7)
  - Rust, JavaScript CI 워크플로우 템플릿 추가 (`workflow_template_rust.yaml`, `workflow_template_javascript.yaml`)
  - AI 에이전트 Git 작업 금지 규칙 추가 (`constitution.md` Section 9.1)
  - TDD 필수 원칙 추가 (`constitution.md` Section 9)
  - 문서 호환성 개선 (`review_before_merge.md` 시점 표현 수정, `project/README.md` 보강)
  - MCP 연결 설정 가이드 추가 (`docs/README.md` Notion, Jira 연동)
  - docs_*/ 삭제
- v0.4.0: AutoCommit 기능 개선
  - Diff 크기 제한 확장 (3000자 → 6000자)
  - 브랜치 식별자 자동 추출 기능 추가 (예: `feature/auth/login-page` → `login-page`)
  - 커밋 메시지 템플릿에 브랜치 식별자 반영 (`{branch_id}` 플레이스홀더)
  - 에러 메시지 환경변수명 일치 (`COMMIT_OPENAI_API_KEY`)
  - Python/Node.js 버전 동일하게 적용
- v0.5.0: Autocommit python에 uv 적용

## initial_docs/{이니셜}

- 최초 프로젝트 탬플릿을 작성하기 전까지의 기초 세팅 설정
- 초기 문서들에 대해선 모두 initial_docs/{이니셜} 브랜치로 따서 작성
