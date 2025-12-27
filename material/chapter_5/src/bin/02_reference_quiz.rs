//! 레퍼런스와 소유권 빌리기 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 레퍼런스 사용하기
    // x의 레퍼런스를 y에 저장하고, x와 y 둘 다 출력하세요
    println!("=== 레퍼런스 사용 ===");
    let x = String::from("hello, world");
    /* TODO: x의 레퍼런스 생성 */
    let y : &String = &x;
    println!("x = {}, y = {}", x, y);

    // 2. 함수에 레퍼런스 전달
    // calculate_length 함수에 레퍼런스를 전달하여 s1을 유지하세요
    println!("\n=== 함수에 레퍼런스 전달 ===");
    let s1 = String::from("hello");
    let len = calculate_length(/* TODO: s1의 레퍼런스 전달 */);
    println!("'{}' 길이: {}", s1, len);

    // 3. 여러 불변 레퍼런스
    // s2의 불변 레퍼런스를 여러 개 만들어 출력하세요
    println!("\n=== 여러 불변 레퍼런스 ===");
    let s2 = String::from("rust");
    let r1 = /* TODO: s2의 레퍼런스 */;
    let r2 = /* TODO: s2의 또 다른 레퍼런스 */;
    println!("r1 = {}, r2 = {}", r1, r2);
}

/// 문자열의 길이를 반환하는 함수
/// 파라미터 타입을 레퍼런스로 수정하세요
fn calculate_length(s: /* TODO: &String 타입 */) -> usize {
    s.len()
}
