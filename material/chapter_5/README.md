# Chapter 5: 소유권 (Ownership)

## 학습 목표

이 챕터에서는 러스트의 핵심 개념인 소유권을 학습합니다:

- **소유권 기본**: 스코프, 이동(Move), 복사(Copy)
- **레퍼런스**: 소유권 빌리기 (Borrowing)
- **가변 레퍼런스**: 값을 수정할 수 있는 참조
- **클로저와 소유권**: 환경 캡처, move 키워드

## 소유권 규칙

1. **모든 값은 소유자(Owner)가 존재**
2. **한 번에 하나의 소유자만 존재** - 두 개의 소유자가 동시에 존재할 수 없음
3. **소유자가 스코프를 벗어나면 값은 메모리에서 해제**

## 파일 구조

```
chapter_5/
├── Cargo.toml
├── README.md
└── src/bin/
    ├── 01_ownership_basics_quiz.rs  # 소유권 기본 퀴즈
    ├── 02_reference_quiz.rs         # 레퍼런스 퀴즈
    ├── 03_mutable_reference_quiz.rs # 가변 레퍼런스 퀴즈
    └── 04_closure_ownership_quiz.rs # 클로저와 소유권 퀴즈
```

각 퀴즈 파일에는 `/* TODO */` 빈칸이 있습니다. 빈칸을 채워서 프로그램을 완성하세요.

## 실행 방법

```bash
# 퀴즈 실행 (TODO를 채운 후)
cargo run --bin 01_ownership_basics_quiz
cargo run --bin 02_reference_quiz
cargo run --bin 03_mutable_reference_quiz
cargo run --bin 04_closure_ownership_quiz
```

## 핵심 개념

### 1. 소유권 이동 (Move)

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1의 소유권이 s2로 이동
// println!("{}", s1);  // 컴파일 에러! s1은 더 이상 유효하지 않음
println!("{}", s2);  // OK
```

### 2. 복사 (Copy)

스택에 저장되는 타입(i32, f64, bool 등)은 Copy 트레이트가 구현되어 자동 복사됩니다.

```rust
let a = 5;
let b = a;  // a가 복사됨
println!("{} {}", a, b);  // 둘 다 사용 가능
```

### 3. clone() - 깊은 복사

힙 데이터를 명시적으로 복사합니다.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 힙 데이터까지 복사
println!("{} {}", s1, s2);  // 둘 다 사용 가능
```

### 4. 레퍼런스 (Reference) - 소유권 빌리기

```rust
let s = String::from("hello");
let len = calculate_length(&s);  // 레퍼런스 전달
println!("{} {}", s, len);  // s 여전히 사용 가능

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### 5. 가변 레퍼런스 (Mutable Reference)

```rust
let mut s = String::from("hello");
change(&mut s);  // 가변 레퍼런스 전달
println!("{}", s);  // "hello world"

fn change(s: &mut String) {
    s.push_str(" world");
}
```

**가변 레퍼런스 규칙:**
- 한 번에 하나의 가변 레퍼런스만 존재 가능
- 불변 레퍼런스가 있는 동안 가변 레퍼런스 생성 불가

### 6. 클로저와 환경 캡처

```rust
// 불변 대여
let multiplier = 5;
let func = |x| x * multiplier;

// 가변 대여
let mut counter = 0;
let mut inc = || { counter += 1; };

// 소유권 이동 (move)
let name = String::from("hello");
let print = move || println!("{}", name);
// name은 더 이상 사용 불가
```

### 7. move 키워드

클로저가 환경 변수의 소유권을 가져가도록 합니다.

```rust
fn factory(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // factor 소유권이 클로저로 이동
}
```

## 주의사항

1. **String vs &str**: String은 힙에 저장, &str은 불변 레퍼런스
2. **이동 vs 복사**: 힙 타입은 이동, Copy 트레이트 타입은 복사
3. **가변 레퍼런스 제한**: 동시에 여러 개 불가
4. **댕글링 참조 방지**: 러스트는 컴파일 타임에 댕글링 참조를 방지
