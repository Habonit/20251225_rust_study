# Chapter 4: 조건문과 반복문

## 학습 목표

이 챕터에서는 러스트의 제어 흐름을 학습합니다:

- **조건문** (if/else): 분기 처리와 let if 패턴
- **for 반복문**: 범위 순회와 break/continue
- **while 반복문**: 조건부 반복
- **loop**: 무한 반복과 값 리턴
- **match**: 강력한 패턴 매칭

## 파일 구조

```
chapter_4/
├── Cargo.toml
├── README.md
└── src/bin/
    ├── 01_if_else.rs          # 조건문 예제
    ├── 01_if_else_quiz.rs     # 조건문 퀴즈
    ├── 02_for_loop.rs         # for 반복문 예제
    ├── 02_for_loop_quiz.rs    # for 반복문 퀴즈
    ├── 03_while_loop.rs       # while 반복문 예제
    ├── 03_while_loop_quiz.rs  # while 반복문 퀴즈
    ├── 04_loop.rs             # loop 예제
    ├── 04_loop_quiz.rs        # loop 퀴즈
    ├── 05_match.rs            # match 패턴매칭 예제
    └── 05_match_quiz.rs       # match 퀴즈
```

## 실행 방법

```bash
# 예제 실행
cargo run --bin 01_if_else
cargo run --bin 02_for_loop
cargo run --bin 03_while_loop
cargo run --bin 04_loop
cargo run --bin 05_match

# 퀴즈 실행 (TODO 채운 후)
cargo run --bin 01_if_else_quiz
cargo run --bin 02_for_loop_quiz
cargo run --bin 03_while_loop_quiz
cargo run --bin 04_loop_quiz
cargo run --bin 05_match_quiz
```

## 핵심 개념

### 1. if/else 조건문

```rust
// 기본 if/else (괄호 없음, 중괄호 필수)
if x < y {
    println!("x is less than y");
} else if x == y {
    println!("x is equal to y");
} else {
    println!("x is greater than y");
}

// let if - 조건문 결과를 변수에 할당
let result = if a < b { "smaller" } else { "bigger" };
```

### 2. for 반복문

```rust
// a..b - 마지막 값 제외
for i in 0..5 {
    print!("{},", i);  // 0,1,2,3,4,
}

// a..=b - 마지막 값 포함
for i in 0..=5 {
    print!("{},", i);  // 0,1,2,3,4,5,
}
```

### 3. while 반복문

```rust
// 러스트에는 ++, -- 연산자 없음!
let mut x = 0;
while x < 5 {
    print!("{},", x);
    x += 1;  // x++ 불가
}
```

### 4. loop (무한 반복)

```rust
// loop에서 값 리턴
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // break로 값 리턴
    }
};

// 중첩 loop와 라벨
'outer: loop {
    loop {
        break 'outer;  // 외부 루프 종료
    }
}
```

### 5. match 패턴 매칭

```rust
// 기본 match (모든 케이스 처리 필수)
let grade = match score {
    90..=100 => "A",       // 범위 매칭
    80..=89 => "B",
    _ => "F",              // 나머지 모든 경우
};

// 여러 패턴 (|)
match day {
    1 | 2 | 3 | 4 | 5 => "평일",
    6 | 7 => "주말",
    _ => "잘못된 요일",
}

// 조건 가드
match num {
    n if n < 0 => "음수",
    n if n % 2 == 0 => "짝수",
    _ => "홀수",
}
```

## 주의사항

1. **if 조건에 괄호 불필요**: `if (x < y)` → `if x < y`
2. **중괄호 필수**: 단일 문장이라도 `{}`가 필요
3. **증감 연산자 없음**: `x++` 불가, `x += 1` 사용
4. **mut 키워드**: 값을 변경하려면 `mut` 필수
5. **match 완전성**: 모든 가능한 케이스를 처리해야 함 (`_` 사용)
