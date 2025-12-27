# Chapter 6: 자료구조와 이터레이터

## 학습 목표

- Rust의 주요 컬렉션 타입(Vec, Array, Tuple, HashMap) 이해
- 문자열 타입(&str, String)의 차이점 파악
- 열거형(Enum)과 Option, Result 활용법 습득
- 이터레이터 패턴과 메서드 체이닝 학습

## 파일 구조

```
chapter_6/
├── Cargo.toml
├── README.md
└── src/bin/
    ├── 01_vector_quiz.rs      # 벡터(Vec) 퀴즈
    ├── 02_array_tuple_quiz.rs # 배열과 튜플 퀴즈
    ├── 03_hashmap_quiz.rs     # 해시맵 퀴즈
    ├── 04_string_quiz.rs      # 문자열 퀴즈
    ├── 05_enum_quiz.rs        # 열거형 퀴즈
    └── 06_iterator_quiz.rs    # 이터레이터 퀴즈
```

각 퀴즈 파일에는 `/* TODO */` 빈칸이 있습니다. 빈칸을 채워서 프로그램을 완성하세요.

## 실행 방법

```bash
# 퀴즈 실행 (TODO를 채운 후)
cargo run --bin 01_vector_quiz
cargo run --bin 02_array_tuple_quiz
cargo run --bin 03_hashmap_quiz
cargo run --bin 04_string_quiz
cargo run --bin 05_enum_quiz
cargo run --bin 06_iterator_quiz
```

## 개념 요약

### 1. 벡터 (Vec)
- 힙에 저장되는 가변 길이 배열
- `Vec::new()` 또는 `vec![]` 매크로로 생성
- `push()`, `pop()`, `remove()` 등으로 요소 관리

### 2. 배열과 튜플
- 배열: 고정 길이, 동일 타입, 스택 저장
- 튜플: 고정 길이, 다른 타입 허용, 인덱스로 접근

### 3. 해시맵 (HashMap)
- 키-값 쌍 저장
- `std::collections::HashMap` 사용
- `insert()`, `get()`, `remove()` 메서드

### 4. 문자열
- `&str`: 스택에 저장, 불변 참조
- `String`: 힙에 저장, 가변 문자열
- `.to_string()`, `String::from()` 으로 변환

### 5. 열거형 (Enum)
- `Option<T>`: Some(값) 또는 None
- `Result<T, E>`: Ok(값) 또는 Err(에러)
- `match`, `if let` 으로 패턴 매칭

### 6. 이터레이터
- `iter()`: 불변 참조 이터레이터
- `into_iter()`: 소유권 이동 이터레이터
- `map()`, `filter()`, `collect()` 체이닝

## 연습 문제 안내

각 `*_quiz.rs` 파일에는 `/* TODO */` 표시가 있습니다.
해당 부분을 채워서 프로그램을 완성하세요.
