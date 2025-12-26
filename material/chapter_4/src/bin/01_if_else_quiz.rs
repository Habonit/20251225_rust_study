//! 조건문 (if/else) 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 if/else
    // x가 y보다 작으면 "x is less than y" 출력
    println!("=== 기본 if/else ===");
    let x = 5;
    let y = 10;

    /* TODO: if문을 작성하여 x < y이면 "x is less than y" 출력 */
    if x < y {
        println!("x is less than y");
    }

    // 2. let if - 조건문 결과를 변수에 할당
    // a가 b보다 작으면 "smaller", 같으면 "equal", 크면 "bigger" 할당
    println!("\n=== let if ===");

    let a = 7;
    let b = 7;
    /* TODO: let if 문으로 결과 할당 */;
    let result = if a < b {
        "smaller"
    }
    else if a == b {
        "equal"
    }
    else {
        "bigger"
    };
    
    println!("result: {}", result);

    // 3. 삼항 연산자 스타일 (한 줄 if)
    // condition이 true면 100, false면 0을 value에 할당
    println!("\n=== 삼항 연산자 스타일 ===");
    let condition = true;
    /* TODO: if condition { ... } else { ... } */;
    let value = if condition {
        100
    } else {
        0
    };
    println!("value = {}", value);
}
