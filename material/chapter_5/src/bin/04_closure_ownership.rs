//! 클로저와 소유권
//!
//! 클로저는 환경을 캡처할 수 있습니다.
//! - 불변 소유권 대여: 외부 변수를 불변으로 빌림
//! - 가변 소유권 대여: 외부 변수를 가변으로 빌림
//! - 소유권 가져가기: move 키워드로 소유권 이동

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 불변 소유권 대여
    println!("=== 불변 소유권 대여 ===");
    let multiplier = 5;
    let func = |x: i32| -> i32 { x * multiplier };  // multiplier를 불변으로 빌림
    for i in 1..=3 {
        println!("{} * {} = {}", i, multiplier, func(i));
    }
    println!("multiplier 여전히 사용 가능: {}", multiplier);

    // 2. 가변 소유권 대여
    println!("\n=== 가변 소유권 대여 ===");
    let mut counter = 0;
    let mut increment = || {
        counter += 1;  // counter를 가변으로 빌림
        counter
    };
    println!("counter: {}", increment());
    println!("counter: {}", increment());
    println!("counter: {}", increment());
    // 클로저 사용 종료 후 counter 사용 가능
    println!("최종 counter: {}", counter);

    // 3. move 키워드 - 소유권 이동
    println!("\n=== move 키워드 ===");
    let greeting = String::from("Hello");
    let print_greeting = move || {
        println!("greeting: {}", greeting);  // greeting 소유권이 클로저로 이동
    };
    print_greeting();
    // println!("{}", greeting);  // 컴파일 에러! greeting 소유권이 이동됨

    // 4. 클로저 리턴하는 함수에서 move 사용
    println!("\n=== 클로저 리턴 함수 ===");
    let mult = factory(5);
    println!("factory(5)(1) = {}", mult(1));
    println!("factory(5)(2) = {}", mult(2));
    println!("factory(5)(3) = {}", mult(3));

    // 5. Copy 타입과 move
    println!("\n=== Copy 타입과 move ===");
    let num = 42;  // i32는 Copy 트레이트 구현
    let print_num = move || {
        println!("num: {}", num);
    };
    print_num();
    println!("num 여전히 사용 가능: {}", num);  // Copy되므로 사용 가능

    // 6. clone 사용하여 소유권 문제 해결
    println!("\n=== clone으로 소유권 문제 해결 ===");
    let name = String::from("Alice");
    let name_clone = name.clone();  // 복제
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
/// move가 없으면 factor가 함수 종료 후 사라져 에러 발생
fn factory(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor  // factor의 소유권을 클로저로 이동
}
