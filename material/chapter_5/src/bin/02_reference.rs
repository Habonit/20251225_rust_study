//! 레퍼런스와 소유권 빌리기 (Borrowing)
//!
//! 소유권을 이동하지 않고 값을 참조할 수 있습니다.
//! & 키워드를 사용하면 레퍼런스(참조)를 만들 수 있습니다.
//! 이를 "빌리기(borrowing)"라고 합니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 레퍼런스
    println!("=== 기본 레퍼런스 ===");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1로 레퍼런스 전달
    println!("'{}' 길이: {}", s1, len);  // s1 여전히 사용 가능

    // 2. 레퍼런스 변수
    println!("\n=== 레퍼런스 변수 ===");
    let x = String::from("hello");
    let y = &x;  // y는 x의 레퍼런스
    println!("x = {}, y = {}", x, y);  // 둘 다 사용 가능

    // 3. 여러 개의 불변 레퍼런스
    println!("\n=== 여러 개의 불변 레퍼런스 ===");
    let s2 = String::from("hello");
    let r1 = &s2;
    let r2 = &s2;
    let r3 = &s2;
    println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);

    // 4. 함수에서 레퍼런스 사용
    println!("\n=== 함수에서 레퍼런스 사용 ===");
    let greeting = String::from("Hello, World!");
    print_string(&greeting);
    println!("원본: {}", greeting);  // 원본 여전히 사용 가능

    // 5. 스코프 내 레퍼런스
    println!("\n=== 스코프 내 레퍼런스 ===");
    let outer = String::from("outer");
    {
        let inner_ref = &outer;  // 외부 변수의 레퍼런스
        println!("inner_ref = {}", inner_ref);
    }  // inner_ref 스코프 종료, 소유권은 outer에 그대로
    println!("outer = {}", outer);

    // 6. 레퍼런스와 역참조
    println!("\n=== 레퍼런스와 역참조 ===");
    let num = 42;
    let num_ref = &num;
    println!("레퍼런스: {}", num_ref);
    println!("역참조: {}", *num_ref);  // * 로 역참조
}

/// 문자열의 길이를 반환하는 함수
/// &String 타입으로 레퍼런스를 받아 소유권을 가져가지 않음
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s가 스코프를 벗어나도 소유권이 없으므로 아무 일도 일어나지 않음

/// 문자열을 출력하는 함수
fn print_string(s: &String) {
    println!("print_string: {}", s);
}  // 레퍼런스만 반환, 원본 소유권은 유지됨
