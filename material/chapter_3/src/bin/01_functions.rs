//! 함수 선언 - fn 키워드
//!
//! Rust에서 함수는 fn 키워드로 선언합니다.
//! 파라미터와 리턴 타입을 반드시 명시해야 합니다.

/// 두 정수를 더하는 함수
///
/// # Arguments
/// * `num1` - 첫 번째 정수
/// * `num2` - 두 번째 정수
///
/// # Returns
/// * `i32` - 두 정수의 합
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

/// 두 정수를 빼는 함수
fn subtract(num1: i32, num2: i32) -> i32 {
    num1 - num2  // return 생략 가능 (세미콜론 없음!)
}

/// 두 정수를 곱하는 함수
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 두 실수를 나누는 함수
fn divide(a: f64, b: f64) -> f64 {
    a / b
}

/// 인사 메시지를 출력하는 함수 (리턴값 없음)
/// println! 매크로는 콘솔에 직접 출력하고 아무것도 반환하지 않습니다.
fn greet(name: &str) {
    println!("안녕하세요, {}님!", name);
}

/// 문자열을 반환하는 함수
/// format! 매크로는 출력하지 않고 String을 반환합니다.
fn get_greeting(name: &str) -> String {
    format!("안녕하세요, {}님!", name)
}

// =====================
// println! vs format! 비교
// =====================
// println!("Hello, {}", name);  // 콘솔에 출력, 반환값 없음 (Unit 타입)
// format!("Hello, {}", name);   // 출력 없음, String 반환
//
// 사용 시점:
// - println!: 디버깅, 사용자 출력 등 화면에 바로 보여줄 때
// - format!: 문자열을 변수에 저장하거나 함수에서 반환할 때

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 함수 호출
    println!("=== 기본 함수 호출 ===");
    println!("add(1, 2) = {}", add(1, 2));
    println!("subtract(10, 3) = {}", subtract(10, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("divide(10.0, 3.0) = {:.2}", divide(10.0, 3.0));

    // 2. 리턴값 없는 함수
    println!("\n=== 리턴값 없는 함수 ===");
    greet("철수");

    // 3. 문자열 반환 함수
    println!("\n=== 문자열 반환 함수 ===");
    let message = get_greeting("영희");
    println!("{}", message);

    // 4. 함수 결과를 변수에 저장
    println!("\n=== 함수 결과 활용 ===");
    let sum = add(10, 20);
    let product = multiply(sum, 2);
    println!("(10 + 20) * 2 = {}", product);

    // 5. 중첩 함수 호출
    println!("\n=== 중첩 함수 호출 ===");
    println!("add(multiply(2, 3), 4) = {}", add(multiply(2, 3), 4));
}
