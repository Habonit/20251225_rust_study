//! 변수 선언 - let 키워드와 타입 추론
//!
//! Rust에서 변수는 let 키워드로 선언합니다.
//! 대부분의 경우 컴파일러가 타입을 추론할 수 있습니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 타입을 명시한 변수 선언
    let x: i32 = 10;
    let y: f64 = 1.0;
    println!("타입 명시: x = {}, y = {}", x, y);

    // 2. 타입 추론을 사용한 변수 선언
    let a = 42;      // i32로 추론
    let b = 3.14;    // f64로 추론
    let c = true;    // bool로 추론
    let d = 'A';     // char로 추론
    println!("타입 추론: a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    // 3. 문자열 타입
    let s1: &str = "문자열 슬라이스";  // &str (문자열 슬라이스)
    let s2: String = String::from("String 타입"); // String (소유권 있는 문자열)
    println!("문자열: s1 = {}, s2 = {}", s1, s2);

    // 4. 작명 규칙 예시
    let snake_case_variable = 100;        // 변수: snake_case
    const SCREAMING_SNAKE: i32 = 999;     // 상수: SCREAMING_SNAKE_CASE
    println!("작명 규칙: {} / {}", snake_case_variable, SCREAMING_SNAKE);

    // 5. 여러 변수 동시 선언 (튜플 디스트럭처링)
    let (first, second, third) = (1, 2, 3);
    println!("튜플 디스트럭처링: {}, {}, {}", first, second, third);
}
