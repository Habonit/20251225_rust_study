//! 소유권 기본 개념 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 소유권 이동 (Move)
    // s1의 값을 s2로 이동시키세요
    println!("=== 소유권 이동 ===");
    let s1 = String::from("hello");
    /* TODO: s1의 소유권을 s2로 이동 */
    let s2 = s1;
    // s1은 더 이상 사용 불가
    println!("s2 = {}", s2);

    // 2. clone을 사용한 깊은 복사
    // s3를 복사하여 s4를 만들고, s3와 s4 둘 다 사용 가능하게 하세요
    println!("\n=== clone 사용 ===");
    let s3 = String::from("hello");
    /* TODO: s3를 clone하여 복사 */
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 3. 소유권 돌려주기
    // take_ownership 함수를 수정하여 소유권을 돌려받으세요
    println!("\n=== 소유권 돌려주기 ===");
    let s5 = String::from("hello, world");
    let s6 = take_ownership(s5);
    println!("s6 = {}", s6);
}

/// 소유권을 받아서 돌려주는 함수
/// 반환 타입과 반환값을 작성하세요
fn take_ownership(s: String) -> String /* TODO: 반환 타입 작성 */ {
    println!("{}", s);
    /* TODO: s를 반환 */
    s
}
