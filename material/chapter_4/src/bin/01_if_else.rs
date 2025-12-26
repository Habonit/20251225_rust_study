//! 조건문 (if/else) - 논리적 분기
//!
//! if 문은 조건을 만족하는 경우 해당 코드를 실행하도록 분기를 만듭니다.
//! 파이썬과 달리 조건에 괄호가 필요 없고, 중괄호 {}가 필수입니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 if/else
    println!("=== 기본 if/else ===");
    let x = 1.0;
    let y = 10;

    // 타입이 다르면 비교 불가 -> 캐스팅 필요
    if x < (y as f64) {
        println!("x is less than y");
    } else if x == (y as f64) {
        println!("x is equal to y");
    } else {
        println!("x is not less than y");
    }

    // 2. let if - 조건문 결과를 변수에 할당
    println!("\n=== let if ===");
    let a = 5;
    let b = 10;

    // if문의 각 분기 결과를 변수에 바로 할당
    let result = if a < b {
        "a is less than b"
    } else if a == b {
        "a is equal to b"
    } else {
        "a is greater than b"
    };
    println!("{}", result);

    // 3. let if로 숫자 할당
    println!("\n=== let if로 숫자 할당 ===");
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("number = {}", number);

    // 4. 함수에서 if 사용
    println!("\n=== 함수에서 if 사용 ===");
    let password = 1234;
    let is_valid = check_password(password);
    println!("Password valid: {}", is_valid);

    // 5. 비교 연산자
    println!("\n=== 비교 연산자 ===");
    let num = 7;
    println!("{} > 5: {}", num, num > 5);
    println!("{} < 5: {}", num, num < 5);
    println!("{} >= 7: {}", num, num >= 7);
    println!("{} <= 7: {}", num, num <= 7);
    println!("{} == 7: {}", num, num == 7);
    println!("{} != 7: {}", num, num != 7);
}

/// 비밀번호를 확인하는 함수
/// if 문의 결과를 바로 리턴
fn check_password(password: i32) -> bool {
    if password == 1234 {
        true
    } else {
        false
    }
}
