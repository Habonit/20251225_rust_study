//! 섀도잉 (Shadowing)
//!
//! 같은 이름으로 변수를 다시 선언하는 것을 섀도잉이라고 합니다.
//! 불변 변수의 값을 변경하는 것과는 다릅니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본적인 섀도잉
    let x = 5;
    println!("x 첫 번째 선언: {}", x);

    let x = 6;  // x를 새로운 값으로 섀도잉
    println!("x 섀도잉 후: {}", x);

    // 2. 섀도잉으로 타입 변경 가능
    let spaces = "   ";       // &str 타입
    println!("spaces (문자열): \"{}\"", spaces);

    let spaces = spaces.len();  // usize 타입으로 변경
    println!("spaces (길이): {}", spaces);

    // 3. mut와 섀도잉의 차이
    // mut는 같은 타입 내에서만 값 변경 가능
    let mut y = 10;
    y = 20;  // OK - 같은 타입
    // y = "hello";  // 에러! 타입 변경 불가

    // 섀도잉은 타입도 변경 가능
    let y = "hello";  // 새로운 변수로 섀도잉 (타입 변경됨)
    println!("y 섀도잉 (타입 변경): {}", y);

    // 4. 스코프와 섀도잉
    let outer = 10;
    println!("외부 스코프 outer: {}", outer);

    {
        // 내부 스코프에서 섀도잉
        let outer = 20;
        println!("내부 스코프 outer: {}", outer);
    }

    // 내부 스코프를 벗어나면 원래 값으로 복원
    println!("외부 스코프로 돌아옴 outer: {}", outer);

    // 5. 연산 결과를 같은 이름에 저장
    let number = 5;
    let number = number + 1;  // 6
    let number = number * 2;  // 12
    println!("연산 후 number: {}", number);
}
