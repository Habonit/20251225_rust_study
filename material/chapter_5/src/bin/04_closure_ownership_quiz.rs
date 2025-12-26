//! 클로저와 소유권 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 클로저에서 환경 캡처 (불변 대여)
    // multiplier를 클로저에서 사용하고, 클로저 호출 후에도 multiplier 사용
    println!("=== 불변 소유권 대여 ===");
    let multiplier = 5;
    /* TODO: x * multiplier */
    let func = |x: i32| -> i32 { x * multiplier };
    println!("func(3) = {}", func(3));
    println!("multiplier = {}", multiplier);  // 여전히 사용 가능

    // 2. move 키워드 사용
    // factory 함수를 완성하여 클로저를 반환하세요
    println!("\n=== move 키워드 ===");
    let mult = factory(10);
    println!("factory(10)(5) = {}", mult(5));  // 기대 출력: 50

    // 3. clone을 사용한 소유권 문제 해결
    // name을 clone하여 두 클로저에서 각각 사용하세요
    println!("\n=== clone으로 소유권 문제 해결 ===");
    let name = String::from("Alice");
    /* TODO: name을 clone */
    let name_clone = name.clone();
    let closure1 = move || {
        println!("closure1: {}", name);
    };
    let closure2 = move || {
        println!("closure2: {}", name_clone);
    };
    closure1();
    closure2();
}

/// 클로저를 리턴하는 팩토리 함수
/// move 키워드를 사용하여 factor의 소유권을 클로저로 이동하세요
fn factory(factor: i32) -> impl Fn(i32) -> i32 {
    /* TODO: move |x| x * factor */
    move |x| x * factor
}
