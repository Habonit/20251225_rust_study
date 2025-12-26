# SDD (Spec-Driven Development) 문서

이 디렉토리는 스펙 주도 개발에 사용하는 브랜치별 작업 스펙과 리뷰 리포트를 관리한다.

---

## 1. 개념

### 1.1 project/ vs sdd/

| 디렉토리 | 역할 | 예시 |
|:--------|:-----|:-----|
| `docs/project/` | **큰 틀** - 프로젝트 전체 개념, 아키텍처 | 시스템 설계, 도메인 모델 |
| `docs/sdd/` | **작은 단위** - 개별 작업 스펙 | 특정 기능 구현 스펙 |

```
docs/project/                     ← 큰 틀 (상위 레벨)
    architecture.md
    domain_model.md
           │
           │ 참조
           ▼
docs/sdd/{브랜치 식별자}/          ← 작은 단위 (하위 레벨)
    feature_spec.md
    report.md
```

### 1.2 SDD 흐름

1. `project/`의 큰 틀 문서를 참조하여 작업 범위 파악
2. `sdd/{브랜치 식별자}/`에 구체적인 스펙 작성
3. Claude에게 스펙 문서를 제공하여 코드 생성 요청
4. 작업 완료 후 리뷰 리포트 생성

---

## 2. 디렉토리 구조

```
sdd/
├── README.md                              # 이 문서
├── {브랜치 식별자}/                        # 브랜치별 작업 스펙 디렉토리
│   ├── *.md                               # 스펙 문서 (여러 개 가능)
│   └── report.md                          # 리뷰 리포트
└── history/                               # 머지 완료된 문서 보관
    └── {브랜치 식별자}/
```

---

## 3. 문서 유형

| 문서 | 설명 | 생성 시점 |
|:-----|:-----|:-----|
| `{브랜치 식별자}/` | 작업 스펙 디렉토리 | 브랜치 생성 후 |
| `{브랜치 식별자}/*.md` | 스펙 문서 (여러 개 가능) | 개발 시작 전 |
| `{브랜치 식별자}/report.md` | Claude 리뷰 리포트 | Push 전 리뷰 후 |

---

## 4. 문서 작성 규칙

### 4.1 디렉토리명

브랜치 식별자를 디렉토리명으로 사용한다.

```
# 브랜치: feature/20250115_PROJ-123_FD-001_honggildong
# 브랜치 식별자: 20250115_PROJ-123_FD-001_honggildong

sdd/
└── 20250115_PROJ-123_FD-001_honggildong/
    ├── initial_spec.md      # 초기 스펙
    ├── api_design.md        # API 설계 (필요 시)
    └── report.md            # 리뷰 리포트
```

### 4.2 작성 참조

| 항목 | 참조 문서 |
|:-----|:-----|
| 기획 문서 | `project/` |
| 문서 스타일 | `core/style.md` |
| 제안서 템플릿 | `core/improvement_proposal_template.md` |
| 프로젝트 큰 틀 | `project/*.md` |

---

## 5. 문서 라이프사이클

```
[1] 브랜치 생성
         │
         ▼
[2] 스펙 디렉토리 생성 ─────────────────────────────────┐
    sdd/{브랜치 식별자}/                                │
         │                                            │
         ▼                                            │
[3] 스펙 문서 작성 (project/ 참조)                       │
    sdd/{브랜치 식별자}/*.md                            │
         │                                            │
         ▼                                            │
[4] Claude에게 스펙 기반 코드 생성 요청                   │
         │                                            │
         ▼                                            │
[5] 개발 진행 (TDD)                                    │
         │                                            │
         ▼                                            │
[6] Push 전 리뷰 → 리포트 생성                          │
    sdd/{브랜치 식별자}/report.md                       │
         │                                            │
         ▼                                            │
[7] PR → dev 머지                                     │
         │                                            │
         ▼                                            │
[8] dev 머지 준비 (prep 브랜치)                         │
    └── sdd/{브랜치 식별자}/ → sdd/history/ 이동 ◄──────┘
```

---

## 6. history 디렉토리

머지가 완료된 디렉토리는 `history/`로 이동하여 보관한다.

```bash
# prep 브랜치에서 실행
mkdir -p docs/sdd/history/ && mv docs/sdd/{브랜치 식별자}/ docs/sdd/history/
```

- dev 머지 전 prep 브랜치에서 이동
- 과거 스펙 및 리뷰 이력 조회 용도

---

## 7. 참조

- [프로젝트 문서 (큰 틀)](../project/README.md)
- [문서 스타일 가이드](../core/style.md)
- [제안서 템플릿](../core/improvement_proposal_template.md)
- [개발 워크플로우](../core/ci_cd.md)

---
