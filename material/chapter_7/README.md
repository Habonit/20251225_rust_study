# Chapter 7: 구조체 (Struct)

## 학습 목표

- 구조체(struct)의 정의와 인스턴스 생성 방법 이해
- 연관 함수와 메서드의 차이점 파악
- 튜플 구조체의 활용법 습득
- 트레이트(trait)를 통한 메서드 공유 이해
- 파생(derive) 트레이트 활용법 학습

## 파일 구조

```
chapter_7/
├── Cargo.toml
├── README.md
└── src/bin/
    ├── 01_struct_basics_quiz.rs # 구조체 기초 퀴즈
    ├── 02_impl_method_quiz.rs   # 연관 함수와 메서드 퀴즈
    ├── 03_tuple_struct_quiz.rs  # 튜플 구조체 퀴즈
    ├── 04_trait_quiz.rs         # 트레이트 퀴즈
    └── 05_derive_quiz.rs        # 파생 트레이트 퀴즈
```

각 퀴즈 파일에는 `/* TODO */` 빈칸이 있습니다. 빈칸을 채워서 프로그램을 완성하세요.

## 실행 방법

```bash
# 퀴즈 실행 (TODO를 채운 후)
cargo run --bin 01_struct_basics_quiz
cargo run --bin 02_impl_method_quiz
cargo run --bin 03_tuple_struct_quiz
cargo run --bin 04_trait_quiz
cargo run --bin 05_derive_quiz
```

## 개념 요약

### 1. 구조체 기초
- `struct` 키워드로 구조체 정의
- 필드에 이름과 타입 지정
- 인스턴스 생성 시 모든 필드에 값 할당 필요

### 2. 연관 함수와 메서드
- 연관 함수: `self`를 사용하지 않음 (예: `new`)
- 메서드: `&self` 또는 `&mut self`를 첫 번째 파라미터로 받음
- `impl` 블록 안에서 정의

### 3. 튜플 구조체
- 필드에 이름 없이 타입만 지정
- 인덱스로 필드 접근 (예: `point.0`, `point.1`)

### 4. 트레이트 (Trait)
- 여러 구조체에서 공통으로 사용할 메서드 정의
- `trait` 키워드로 정의, `impl Trait for Struct`로 구현
- 구조체 상속 대신 사용

### 5. 파생 트레이트 (Derive)
- `#[derive(...)]` 애트리뷰트로 기본 구현 제공
- `Debug`: `{:?}` 포맷터 지원
- `Clone`: `.clone()` 메서드로 명시적 복사
- `Copy`: 값 전달 시 자동 복사

## 연습 문제 안내

각 `*_quiz.rs` 파일에는 `/* TODO */` 표시가 있습니다.
해당 부분을 채워서 프로그램을 완성하세요.
