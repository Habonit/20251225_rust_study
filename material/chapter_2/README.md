# 2장. 변수 (Variables)

Rust의 변수 선언과 관련된 핵심 개념을 학습합니다.

## 학습 목표

- `println!` 매크로로 값을 출력하는 방법
- `let` 키워드로 변수를 선언하는 방법
- 불변성(immutability)과 `mut` 키워드
- 섀도잉(shadowing)의 개념과 활용
- Rust의 원시 타입(primitive types)
- 타입 캐스팅(`as` 키워드)
- 상수(`const`) 선언

---

## 1. 값 출력하기 (`println!`)

```rust
println!("Hello, world!");
```

- `println!`에서 `!`는 매크로(macro)를 호출한다는 의미
- `{}`를 사용하여 변수 값을 출력할 수 있음

```rust
let x = 10;
println!("x = {}", x);
```

- `{:?}` 이건 debug fksms rust의 표준라이브러리의 trait의 일종이다. 
```rust
let numbers = [1, 2, 3];

// Debug 출력 - 개발자가 "이 변수에 뭐가 들어있지?" 확인용
println!("{:?}", numbers);  // [1, 2, 3]

// Pretty Debug - 복잡한 구조체를 보기 좋게
println!("{:#?}", numbers); // 들여쓰기 포함된 출력

``

> 실습 파일: `src/bin/01_print.rs`

---

## 2. 변수 선언

### 기본 문법

```rust
let 변수명: 타입 = 값;
```

예시:
```rust
let x: i32 = 10;
let y = 10;  // 타입 추론
```

### 작명 규칙

| 구분 | 규칙 | 예시 |
|:-----|:-----|:-----|
| 변수 | snake_case | `let my_variable = 3;` |
| 함수 | snake_case | `fn my_function()` |
| 구조체 | PascalCase | `struct MyStruct` |
| 상수 | SCREAMING_SNAKE_CASE | `const MAX_VALUE: i32 = 100;` |

> 실습 파일: `src/bin/02_variables.rs`

---

## 3. 불변성 (Immutability)

Rust의 모든 변수는 **기본적으로 불변(immutable)** 입니다.

```rust
let x = 1;
x = 2;  // 컴파일 에러!
```

가변 변수로 선언하려면 `mut` 키워드를 사용합니다:

```rust
let mut x = 1;
x = 2;  // OK
```

> 실습 파일: `src/bin/03_mutability.rs`

---

## 4. 섀도잉 (Shadowing)

같은 이름으로 변수를 다시 선언하는 것을 **섀도잉**이라고 합니다.

```rust
let x = "5";       // &str 타입
let x = 6;         // i32 타입으로 재선언
println!("{}", x); // 6 출력
```

- 불변 변수의 값을 변경하는 것과는 다름
- 타입도 변경 가능

> 실습 파일: `src/bin/04_shadowing.rs`

---

## 5. 타입 (Types)

### 원시 타입 목록

| 이름 | 타입 | 설명 |
|:-----|:-----|:-----|
| i8, i16, i32, i64, i128 | 정수 | 부호 있는 정수 |
| u8, u16, u32, u64, u128 | 정수 | 부호 없는 정수 |
| isize, usize | 정수 | 아키텍처 의존 크기 |
| f32, f64 | 실수 | 부동소수점 |
| bool | 불리언 | true / false |
| char | 문자 | 유니코드 스칼라 값 |
| String | 문자열 | 소유권 있는 문자열 |
| &str | 문자열 슬라이스 | 문자열 참조 |

### 타입 추론

```rust
let x = 1;    // i32로 추론
let y = 1.0;  // f64로 추론
```

### 타입 캐스팅 (`as` 키워드)

```rust
let x: f64 = 1.2;
let y = x as i32;  // 1
println!("{} -> {}", x, y);
```

> 실습 파일: `src/bin/05_types.rs`

---

## 6. 상수 (Constants)

상수는 `const` 키워드로 선언하며, **반드시 타입을 명시**해야 합니다.

```rust
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}
```

- 상수는 값을 변경할 수 없음
- 전역 스코프에서 선언 가능
- 컴파일 타임에 값이 결정됨

> 실습 파일: `src/bin/06_constants.rs`

---

## 파일 구조

```
src/bin/
├── 01_print.rs           # 정답: 값 출력하기
├── 01_print_quiz.rs      # 문제: 빈칸 채우기
├── 02_variables.rs       # 정답: 변수 선언
├── 02_variables_quiz.rs  # 문제: 빈칸 채우기
├── 03_mutability.rs      # 정답: 불변성과 mut
├── 03_mutability_quiz.rs # 문제: 빈칸 채우기
├── 04_shadowing.rs       # 정답: 섀도잉
├── 04_shadowing_quiz.rs  # 문제: 빈칸 채우기
├── 05_types.rs           # 정답: 타입과 캐스팅
├── 05_types_quiz.rs      # 문제: 빈칸 채우기
├── 06_constants.rs       # 정답: 상수
└── 06_constants_quiz.rs  # 문제: 빈칸 채우기
```

- **정답 파일** (`XX_name.rs`): 완성된 예제 코드
- **문제 파일** (`XX_name_quiz.rs`): `/* TODO */` 빈칸을 채워서 완성하는 문제지

---

## 실행 방법

### 정답 실행 (학습용)

```bash
cargo run --bin 01_print
cargo run --bin 02_variables
cargo run --bin 03_mutability
cargo run --bin 04_shadowing
cargo run --bin 05_types
cargo run --bin 06_constants
```

### 문제 풀이

1. `*_quiz.rs` 파일을 열어 `/* TODO */` 부분을 채웁니다
2. 주석 처리된 `println!` 문을 해제합니다
3. 실행하여 결과를 확인합니다

```bash
# 문제 실행
cargo run --bin 01_print_quiz

# 정답과 비교
cargo run --bin 01_print
```

---

## 연습문제 (추가)

### 문제 1
다음 코드가 컴파일되지 않는 이유를 설명하세요:
```rust
fn main() {
    let x = 1;
    x = 2;
    println!("{}", x);
}
```

<details>
<summary>정답</summary>

불변 변수에는 새로운 값을 할당할 수 없습니다. `let mut x = 1;`로 수정해야 합니다.

</details>

### 문제 2
다음 코드가 결과 3이 나오도록 타입 캐스팅을 추가하세요:
```rust
fn main() {
    let x = 1.2;
    let y = x;
    let z = 2;
    println!("y + z = {}", y + z);
}
```

<details>
<summary>정답</summary>

```rust
println!("y + z = {}", y as i32 + z);
```

</details>

### 문제 3
다음 코드가 컴파일되도록 수정하세요:
```rust
const PI = 3.14;

fn main() {
    println!("원주율: {}", PI);
}
```

<details>
<summary>정답</summary>

```rust
const PI: f64 = 3.14;
```
상수 선언 시 반드시 타입을 명시해야 합니다.

</details>
