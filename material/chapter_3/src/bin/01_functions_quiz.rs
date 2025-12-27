//! 함수 선언 - fn 키워드 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

/// 두 정수를 더하는 함수
/// 힌트: 파라미터 타입과 리턴 타입을 명시해야 합니다
/* TODO: fn 키워드로 add 함수 선언 */
fn add(num1: /* TODO */, num2: /* TODO */) -> /* TODO */ {
    return num1 + num2;
}

/// 두 정수를 빼는 함수
/// 힌트: return 키워드 없이 마지막 표현식으로 리턴할 수 있습니다
fn subtract(num1: i32, num2: i32) -> i32 {
    /* TODO: return 없이 뺄셈 결과 리턴 */
}

/// 두 정수를 곱하는 함수
fn multiply(a: i32, b: i32) -> i32 {
    /* TODO */
}

/// 두 실수를 나누는 함수
/// 힌트: 실수 타입은 f64를 사용합니다
fn divide(a: /* TODO */, b: /* TODO */) -> /* TODO */ {
    a / b
}

// =====================
// println! vs format! 차이
// =====================
// println!: 콘솔에 출력, 반환값 없음 (Unit 타입)
// format!:  출력 없음, String 반환
//
// 예시:
// println!("Hello, {}", name);  // 화면에 출력됨
// let s = format!("Hello, {}", name);  // s에 문자열 저장
// =====================

/// 인사 메시지를 출력하는 함수 (리턴값 없음)
/// 힌트: println!은 출력만 하고 값을 반환하지 않음
fn greet(name: &str) {
    /* TODO: println!으로 "안녕하세요, {name}님!" 출력 */
}

/// 문자열을 반환하는 함수
/// 힌트: format!은 println!과 같은 문법이지만 String을 반환함
fn get_greeting(name: &str) -> String {
    /* TODO: format! 매크로로 인사 문자열 생성 후 리턴 */
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 함수 호출
    println!("=== 기본 함수 호출 ===");
    println!("add(1, 2) = {}", /* TODO: add 함수 호출 */);
    println!("subtract(10, 3) = {}", subtract(10, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("divide(10.0, 3.0) = {:.2}", divide(10.0, 3.0));

    // 2. 리턴값 없는 함수
    println!("\n=== 리턴값 없는 함수 ===");
    /* TODO: greet 함수에 "철수" 전달하여 호출 */

    // 3. 문자열 반환 함수
    println!("\n=== 문자열 반환 함수 ===");
    let message = get_greeting("영희");
    println!("{}", message);

    // 4. 함수 결과를 변수에 저장
    println!("\n=== 함수 결과 활용 ===");
    let sum = add(10, 20);
    let product = /* TODO: sum에 2를 곱하는 함수 호출 */;
    println!("(10 + 20) * 2 = {}", product);

    // 5. 중첩 함수 호출
    println!("\n=== 중첩 함수 호출 ===");
    // multiply(2, 3)의 결과와 4를 더하기
    println!("add(multiply(2, 3), 4) = {}", /* TODO: 중첩 호출 */);
}
