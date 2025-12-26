//! 가변 레퍼런스 (Mutable Reference)
//!
//! 불변 레퍼런스(&)로는 값을 수정할 수 없습니다.
//! 값을 수정하려면 가변 레퍼런스(&mut)를 사용해야 합니다.
//! 단, 가변 레퍼런스는 한 번에 하나만 존재할 수 있습니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 가변 레퍼런스 기본
    println!("=== 가변 레퍼런스 기본 ===");
    let mut s = String::from("hello");
    change(&mut s);  // 가변 레퍼런스로 전달
    println!("변경 후: {}", s);

    // 2. 가변 레퍼런스로 값 수정
    println!("\n=== 가변 레퍼런스로 값 수정 ===");
    let mut x = 10;
    let x_ref = &mut x;
    *x_ref += 5;  // 역참조하여 값 수정
    println!("x = {}", x);

    // 3. 가변 레퍼런스는 하나만 가능
    println!("\n=== 가변 레퍼런스 제한 ===");
    let mut s2 = String::from("hello");
    {
        let r1 = &mut s2;
        r1.push_str(" world");
        println!("r1 = {}", r1);
    }  // r1 스코프 종료
    // 이제 새로운 가변 레퍼런스 생성 가능
    let r2 = &mut s2;
    r2.push_str("!");
    println!("r2 = {}", r2);

    // 4. 불변 레퍼런스와 가변 레퍼런스
    println!("\n=== 불변/가변 레퍼런스 규칙 ===");
    let mut s3 = String::from("hello");
    let r3 = &s3;  // 불변 레퍼런스
    let r4 = &s3;  // 또 다른 불변 레퍼런스 (OK)
    println!("r3 = {}, r4 = {}", r3, r4);
    // r3, r4 사용 완료 후
    let r5 = &mut s3;  // 이제 가변 레퍼런스 가능
    r5.push_str(" world");
    println!("r5 = {}", r5);

    // 5. 함수에서 가변 레퍼런스 사용
    println!("\n=== 함수에서 가변 레퍼런스 ===");
    let mut message = String::from("Hello");
    append_world(&mut message);
    println!("message = {}", message);

    // 6. 숫자 가변 레퍼런스
    println!("\n=== 숫자 가변 레퍼런스 ===");
    let mut counter = 0;
    increment(&mut counter);
    increment(&mut counter);
    increment(&mut counter);
    println!("counter = {}", counter);
}

/// 문자열에 " world!"를 추가하는 함수
fn change(s: &mut String) {
    s.push_str(" world!");
}

/// 문자열에 " world"를 추가하는 함수
fn append_world(s: &mut String) {
    s.push_str(" world");
}

/// 숫자를 1 증가시키는 함수
fn increment(n: &mut i32) {
    *n += 1;
}
