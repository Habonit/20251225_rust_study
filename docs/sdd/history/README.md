# SDD History

이 디렉토리는 dev 머지가 완료된 작업 스펙 문서를 보관한다.

---

## 1. 목적

- 완료된 작업의 스펙 및 리뷰 이력 보관
- 과거 작업 참조 및 추적
- 활성 작업 디렉토리(`sdd/`)의 정리

---

## 2. 이동 시점

`prep/{YYYYMMDD}_{작업자}` 브랜치에서 dev로 PR 생성 전에 이동한다.

```
[워크플로우 위치]

... → [7] PR → dev 머지 (feature 브랜치들)
              │
              ▼
      [8] dev 머지 완료
              │
              ▼
      [9] 당번 개발자: dev 머지 전 리뷰
              │
              ▼
      [10] prep 브랜치 생성 + 정리 작업  ← 여기서 history로 이동
              │
              ▼
      [11] Push + PR (prep → dev)
```

---

## 3. 이동 명령어

```bash
# prep 브랜치에서 실행
git checkout dev
git pull origin dev
git checkout -b prep/{YYYYMMDD}_{작업자}

# 완료된 작업 디렉토리를 history로 이동
mkdir -p docs/sdd/history/ && mv docs/sdd/{브랜치 식별자}/ docs/sdd/history/
```

---

## 4. 디렉토리 구조

```
sdd/history/
├── README.md                                    # 이 문서
├── 20250115_PROJ-123_FD-001_honggildong/        # 완료된 작업 1
│   ├── initial_spec.md
│   └── report.md
├── 20250120_PROJ-456_FD-002_kimcheolsu/         # 완료된 작업 2
│   ├── feature_spec.md
│   ├── api_design.md
│   └── report.md
└── ...
```

---

## 5. 참조

- [개발 워크플로우](../../core/ci_cd.md) - 섹션 [10] prep 브랜치 생성
- [SDD 문서 가이드](../README.md)

---
