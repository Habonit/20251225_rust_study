//! 리턴값 - return 키워드와 튜플 리턴
//!
//! Rust에서는 마지막 표현식이 자동으로 리턴됩니다.
//! 여러 값을 리턴할 때는 튜플을 사용합니다.

/// return 키워드를 사용한 함수
fn add_with_return(a: i32, b: i32) -> i32 {
    return a + b;
}

/// return 키워드 생략 (세미콜론 없음!)
fn add_without_return(a: i32, b: i32) -> i32 {
    a + b  // 마지막 표현식이 리턴값
}

/// 조건에 따라 다른 값 리턴 (early return)
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n;  // 조건부 early return
    }
    n  // 기본 리턴
}

/// 두 값을 교환하여 리턴 (튜플 리턴)
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

/// 나눗셈의 몫과 나머지를 함께 리턴
fn div_mod(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

/// 최솟값과 최댓값을 함께 리턴
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

/// 리턴값이 없는 함수 (Unit 타입)
fn print_message(msg: &str) {
    println!("{}", msg);
}

/// Unit 타입을 명시적으로 표기
fn print_message_explicit(msg: &str) -> () {
    println!("{}", msg);
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. return 키워드 사용 vs 생략
    println!("=== return 키워드 ===");
    println!("with return: {}", add_with_return(3, 5));
    println!("without return: {}", add_without_return(3, 5));

    // 2. early return
    println!("\n=== early return ===");
    println!("absolute(-5) = {}", absolute(-5));
    println!("absolute(5) = {}", absolute(5));

    // 3. 튜플 리턴
    println!("\n=== 튜플 리턴 ===");
    let (x, y) = swap(1, 2);
    println!("swap(1, 2) = ({}, {})", x, y);

    // 4. 여러 값 리턴
    println!("\n=== 여러 값 리턴 ===");
    let (quotient, remainder) = div_mod(17, 5);
    println!("17 / 5 = {} 나머지 {}", quotient, remainder);

    let (min, max) = min_max(10, 3);
    println!("min_max(10, 3) = (min: {}, max: {})", min, max);

    // 5. 튜플 인덱스로 접근
    println!("\n=== 튜플 인덱스 접근 ===");
    let result = div_mod(20, 7);
    println!("몫: {}, 나머지: {}", result.0, result.1);

    // 6. Unit 타입 함수
    println!("\n=== Unit 타입 함수 ===");
    print_message("암시적 Unit 타입");
    print_message_explicit("명시적 Unit 타입");
}
