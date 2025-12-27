# 3장. 함수 (Functions)

Rust의 함수 선언 기본 개념을 학습합니다.

## 학습 목표

- `fn` 키워드로 함수를 선언하는 방법
- 파라미터와 리턴 타입 명시
- `return` 키워드 생략과 표현식
- 튜플을 사용한 여러 값 리턴
- 스코프(scope)와 변수 유효 범위
- 클로저(Closure) - 익명 함수
- 매크로(Macro) - 코드를 생성하는 코드

---

## 1. 함수 선언

### 기본 문법

```rust
fn 함수명(파라미터: 타입) -> 리턴타입 {
    // 함수 본문
}
```

### 파이썬 vs 러스트

| 파이썬 | 러스트 |
|:------|:------|
| `def add(a: int, b: int) -> int:` | `fn add(a: i32, b: i32) -> i32 {` |
| 타입 힌트 선택 | 타입 명시 **필수** |

```rust
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
```

### println! vs format!

두 매크로는 문법은 같지만 **용도가 다릅니다**:

| 매크로 | 동작 | 반환값 |
|:------|:-----|:------|
| `println!` | 콘솔에 **출력** | `()` (Unit, 없음) |
| `format!` | 출력 **안 함** | `String` |

```rust
// println! - 화면에 출력, 반환값 없음
fn greet(name: &str) {
    println!("Hello, {}!", name);  // 출력됨
}

// format! - 출력 없이 String 반환
fn get_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // 문자열 반환
}

fn main() {
    greet("철수");                    // "Hello, 철수!" 출력
    let msg = get_greeting("영희");   // msg에 문자열 저장
    println!("{}", msg);              // 이제야 출력됨
}
```

**사용 시점:**
- `println!`: 디버깅, 로그, 사용자에게 즉시 보여줄 때
- `format!`: 문자열을 변수에 저장하거나 함수에서 반환할 때

> 실습 파일: `src/bin/01_functions_quiz.rs`

---

## 2. 리턴값

### return 키워드 생략

마지막 표현식은 `return`과 세미콜론 없이 값을 반환합니다:

```rust
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2  // return 생략, 세미콜론 없음!
}
```

### 여러 값 리턴 (튜플)

```rust
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let (x, y) = swap(1, 2);
    println!("{}, {}", x, y);  // 2, 1
}
```

### 리턴값이 없는 함수

```rust
fn print_hello() {
    println!("Hello");
}

// 또는 명시적으로
fn print_hello() -> () {
    println!("Hello");
}
```

> 실습 파일: `src/bin/02_return_quiz.rs`

---

## 3. 스코프 (Scope)

러스트에서 스코프는 `{}` 중괄호 기준으로 구분됩니다.

```rust
fn main() {
    let x = 10;

    {
        let y = 20;  // 내부 스코프에서만 유효
        println!("x = {}, y = {}", x, y);
    }

    // println!("{}", y);  // 에러! y는 스코프 밖
}
```

### 파이썬과의 차이

| 파이썬 | 러스트 |
|:------|:------|
| 함수 단위 스코프 | `{}` 블록 단위 스코프 |
| if 내부 변수도 외부에서 접근 가능 | 블록 밖에서 접근 불가 |

> 실습 파일: `src/bin/03_scope_quiz.rs`

---

## 4. 클로저 (Closure)

클로저는 익명 함수로, 변수에 저장하거나 다른 함수에 전달할 수 있습니다.

### 기본 문법

```rust
let 변수명 = |파라미터| 표현식;
```

### 예시

```rust
// 타입 추론
let add_one = |x| x + 1;

// 타입 명시
let add_one = |x: i32| -> i32 { x + 1 };

println!("{}", add_one(5));  // 6
```

### 파이썬 lambda vs 러스트 클로저

| 파이썬 | 러스트 |
|:------|:------|
| `lambda x: x + 1` | `\|x\| x + 1` |

> 실습 파일: `src/bin/04_closure_quiz.rs`

---

## 5. 매크로 (Macro)

매크로는 컴파일 타임에 코드를 생성하는 메타프로그래밍 기법입니다.

### 함수와의 차이

| 특징 | 함수 | 매크로 |
|:-----|:-----|:------|
| 실행 시점 | 런타임 | 컴파일 타임 |
| 인수 개수 | 고정 | 가변 |
| 호출 방법 | `func()` | `macro!()` |

### 자주 사용하는 표준 매크로

```rust
// println! - 포맷 출력
println!("Hello, {}!", name);

// vec! - 벡터 생성
let v = vec![1, 2, 3];

// format! - 문자열 포맷팅
let s = format!("x = {}", x);

// dbg! - 디버그 출력
let result = dbg!(2 + 2);

// assert! - 조건 확인
assert!(1 + 1 == 2);
```

> 실습 파일: `src/bin/05_macros_quiz.rs`

---

## 파일 구조

```
src/bin/
├── 01_functions_quiz.rs  # 함수 선언
├── 02_return_quiz.rs     # 리턴값
├── 03_scope_quiz.rs      # 스코프
├── 04_closure_quiz.rs    # 클로저
└── 05_macros_quiz.rs     # 매크로
```

각 퀴즈 파일에는 `/* TODO */` 빈칸이 있습니다. 빈칸을 채워서 프로그램을 완성하세요.

---

## 실행 방법

```bash
# 퀴즈 실행 (TODO를 채운 후)
cargo run --bin 01_functions_quiz
cargo run --bin 02_return_quiz
cargo run --bin 03_scope_quiz
cargo run --bin 04_closure_quiz
cargo run --bin 05_macros_quiz
```

---

## 연습문제 (추가)

### 문제 1
다음 함수의 빈칸을 채우세요:
```rust
fn multiply_numbers(?) {?}

fn main() {
    let result = multiply_numbers(3, 4);
    println!("The product is: {}", result); // 12
}
```

<details>
<summary>정답</summary>

```rust
fn multiply_numbers(a: i32, b: i32) -> i32 {
    a * b
}
```

</details>

### 문제 2
다음 클로저의 빈칸을 채우세요:
```rust
fn main() {
    let multiply = |?| -> ? { ? };
    println!("{}", multiply(3, 4)); // 12
}
```

<details>
<summary>정답</summary>

```rust
let multiply = |a: i32, b: i32| -> i32 { a * b };
```

</details>
