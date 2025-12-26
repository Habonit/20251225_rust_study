//! 클로저 (Closure) - 익명 함수 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 클로저 (타입 추론)
    println!("=== 기본 클로저 ===");
    // 힌트: 클로저는 |파라미터| 표현식 형태입니다
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // 2. 타입 명시 클로저
    println!("\n=== 타입 명시 클로저 ===");
    // 힌트: |x: 타입| -> 리턴타입 { 표현식 }
    let add_one_typed = |x: i32| -> i32 { x + 1 };
    println!("add_one_typed(5) = {}", add_one_typed(5));

    // 3. 여러 파라미터
    println!("\n=== 여러 파라미터 ===");
    let add = |a: i32, b: i32| -> i32 { a + b };
    // 힌트: 타입 추론을 사용할 수도 있습니다
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("add(3, 4) = {}", add(3, 4));
    println!("multiply(3, 4) = {}", multiply(3, 4));

    // 4. 외부 변수 캡처
    println!("\n=== 외부 변수 캡처 ===");
    let factor = 10;
    // 힌트: 클로저는 외부 변수를 자동으로 캡처합니다
    let multiply_by_factor = |x: i32| -> i32 { x * factor };
    println!("5 * {} = {}", factor, multiply_by_factor(5));

    // 5. 클로저를 변수에 저장하고 나중에 호출
    println!("\n=== 지연 실행 ===");
    let operation = |a: i32, b: i32| {
        println!("계산 중...");
        a + b
    };
    println!("결과: {}", operation(10, 20));

    // 6. 여러 줄 클로저
    println!("\n=== 여러 줄 클로저 ===");
    let complex_operation = |x: i32| {
        let doubled = x * 2;
        let squared = doubled * doubled;
        squared + 1  // 리턴값
    };
    println!("complex_operation(3) = {}", complex_operation(3));

    // 7. 파라미터 없는 클로저
    println!("\n=== 파라미터 없는 클로저 ===");
    // 힌트: || 로 파라미터 없는 클로저를 만듭니다
    let say_hello = || println!("Hello!");
    say_hello();

    let get_number = || 42;
    println!("get_number() = {}", get_number());

    // 8. move 키워드 (소유권 이동)
    println!("\n=== move 클로저 ===");
    let message = String::from("Hello");
    // 힌트: move 키워드로 소유권을 클로저 내부로 이동
    let print_message = move || {
        println!("{}", message);
    };
    print_message();
    // println!("{}", message);  // 에러! message는 이동됨
}