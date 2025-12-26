# CLAUDE.md

이 문서는 Claude Code가 프로젝트에서 작업할 때 준수해야 할 핵심 규칙이다.

---

## 0. 프로젝트 목적

이 프로젝트는 **개발 언어 학습**을 목적으로 한다.

- 다양한 프로그래밍 언어(Rust, Python 등)의 문법과 패턴을 익히기 위한 공간
- 실무 프로젝트가 아닌 학습/실험 목적
- 코드 품질보다 학습 과정에 초점

---

## 1. AI 에이전트 Git 작업 금지

Claude Code는 다음 Git/GitHub 작업을 **절대 수행하지 않는다**:

| 금지 작업 | 예시 명령어 |
|:---------|:-----------|
| 커밋 생성 | `git commit`, `git commit -m` |
| 브랜치 생성/전환/삭제 | `git checkout`, `git branch`, `git switch` |
| 원격 저장소 작업 | `git push`, `git pull`, `git fetch` |
| 머지/리베이스 | `git merge`, `git rebase` |
| 히스토리 변경 | `git reset`, `git revert`, `git cherry-pick` |
| GitHub CLI 작업 | `gh pr create`, `gh issue` |
| 스테이징 | `git add`, `git restore --staged` |

**허용 작업**:
- 코드 작성, 수정, 삭제
- 파일 생성, 이동, 삭제
- 테스트 실행
- 빌드/린트 실행
- 문서 작성

---

## 2. 개발 철학

1. **단순성 우선**: 복잡한 로직보다 이해하기 쉬운 코드
2. **명확한 구조**: 코드 구조는 일관되고 예측 가능해야 함

---

## 3. 코드 스타일 핵심

### 3.1 공통 규칙

- **타입 힌트/타입 정의**: 모든 함수에 타입 명시 필수
- **문서화**: 모든 public 함수/메서드에 Docstring/Javadoc/JSDoc 작성
- **주석 언어**: 한글로 작성 (기술 용어는 영어 허용)

### 3.2 언어별 들여쓰기

| 언어 | 들여쓰기 |
|:-----|:---------|
| Python | 스페이스 4칸 |
| Java | 스페이스 4칸 |
| Rust | 스페이스 4칸 |
| TypeScript/JavaScript | 스페이스 2칸 |

### 3.3 네이밍 컨벤션

| 언어 | 함수/변수 | 클래스/타입 | 상수 |
|:-----|:---------|:-----------|:-----|
| Python | snake_case | PascalCase | UPPER_SNAKE_CASE |
| Java | camelCase | PascalCase | UPPER_SNAKE_CASE |
| Rust | snake_case | PascalCase | UPPER_SNAKE_CASE |
| TypeScript | camelCase | PascalCase | UPPER_SNAKE_CASE |
| JavaScript | camelCase | PascalCase | UPPER_SNAKE_CASE |

---

## 4. 예외 처리 원칙

- **구체적 예외 타입 명시**: bare except (Python), 포괄적 Exception catch (Java) 금지
- **TypeScript**: `unknown` 타입 + 타입 가드 사용 필수
- **사용자 친화적 메시지**: 한글로 에러 메시지 제공
- **Graceful Degradation**: 일부 실패 시에도 서비스 계속 제공

---

## 5. 패키지 관리

| 언어 | 도구 | 의존성 파일 |
|:-----|:-----|:-----------|
| Python | UV | `pyproject.toml` |
| Java | Gradle/Maven | `build.gradle` / `pom.xml` |
| Rust | Cargo | `Cargo.toml` |
| JavaScript/TypeScript | npm | `package.json` |

---

## 6. 환경 변수

- API Key 등 민감 정보: `.env` 파일에 보관
- `.env` 파일은 `.gitignore`에 포함
- 환경변수를 소스 파일에 하드코딩 금지

---

## 7. 학습 자료 생성 규칙

사용자가 챕터별 학습 자료를 요청하면 다음 형식을 따른다.

### 7.1 디렉토리 구조

```
material/chapter_N/
├── Cargo.toml
├── README.md
└── src/bin/
    ├── 01_concept.rs       (정답 파일)
    ├── 01_concept_quiz.rs  (퀴즈 파일)
    ├── 02_concept.rs
    ├── 02_concept_quiz.rs
    └── ...
```

### 7.2 참조 자료

- PDF 강의 자료: `material/pdf/chN.pdf`
- PDF 내용을 기반으로 개념별 파일 구성

### 7.3 Cargo.toml 설정

```toml
[package]
name = "chapter_N"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "01_concept"
path = "src/bin/01_concept.rs"

[[bin]]
name = "01_concept_quiz"
path = "src/bin/01_concept_quiz.rs"
# ... 각 파일마다 [[bin]] 추가
```

### 7.4 README.md 구성

1. 챕터 제목 및 학습 목표
2. 파일 구조 설명 (정답/퀴즈 쌍)
3. 개념별 학습 내용 요약
4. 실행 방법 (`cargo run --bin 파일명`)
5. 연습 문제 안내

### 7.5 정답 파일 형식

```rust
//! 개념 설명 - 부제목
//!
//! 상세 설명

/// 함수/구조체 설명
/// # Arguments, # Returns 등 문서화
fn example() {
    // 한글 주석으로 설명
    println!("예제 코드");
}

fn main() {
    // 섹션별로 구분하여 예제 실행
    println!("=== 섹션 제목 ===");
    // 예제 코드...
}
```

### 7.6 퀴즈 파일 형식

```rust
//! 개념 설명 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

fn example() {
    // 힌트: 설명
    let x = /* TODO */;
    let y = /* TODO: 구체적인 힌트 */;
}
```

- `/* TODO */`: 단순 빈칸
- `/* TODO: 힌트 */`: 힌트가 포함된 빈칸
- 정답 파일과 동일한 구조 유지
- 빌드되지 않아도 됨 (학습자가 완성해야 함)

### 7.7 파일 네이밍

- `NN_concept.rs`: 숫자 2자리 + 언더스코어 + 개념명
- `NN_concept_quiz.rs`: 정답 파일명 + `_quiz`
- 개념명은 snake_case 사용

---
