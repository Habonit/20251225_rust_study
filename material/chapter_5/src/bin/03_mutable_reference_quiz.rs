//! 가변 레퍼런스 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 가변 변수 선언 및 수정
    // s를 가변 변수로 선언하고 borrow_object에 가변 레퍼런스로 전달하세요
    println!("=== 가변 레퍼런스 사용 ===");
    let /* TODO: mut 키워드 */ s = String::from("hello, ");
    borrow_object(/* TODO: 가변 레퍼런스로 전달 */);
    println!("결과: {}", s);

    // 2. 가변 레퍼런스로 값 수정
    // x_ref를 사용하여 x 값을 10 증가시키세요
    println!("\n=== 가변 레퍼런스로 값 수정 ===");
    let mut x = 5;
    let x_ref = &mut x;
    /* TODO: *x_ref로 역참조하여 10 더하기 */
    println!("x = {}", x);  // 기대 출력: x = 15

    // 3. 함수에서 가변 레퍼런스 사용
    // append_greeting 함수를 완성하세요
    println!("\n=== 함수에서 가변 레퍼런스 ===");
    let mut message = String::from("Hello");
    append_greeting(&mut message);
    println!("message = {}", message);  // 기대 출력: Hello, World!
}

/// 문자열에 "world"를 추가하는 함수
fn borrow_object(s: &mut String) {
    s.push_str("world");
}

/// 문자열에 ", World!"를 추가하는 함수
/// 파라미터 타입을 가변 레퍼런스로 수정하고 push_str을 사용하세요
fn append_greeting(s: /* TODO: 가변 레퍼런스 타입 */) {
    /* TODO: s에 ", World!" 추가 */
}
