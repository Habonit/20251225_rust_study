//! 상수 (Constants) - const 키워드
//!
//! 상수는 한 번 선언되면 값이 바뀌지 않는 변수입니다.
//! const 키워드로 선언하며, 반드시 타입을 명시해야 합니다.

// 전역 상수 선언 (함수 외부에서 선언 가능)
const THRESHOLD: i32 = 10;
const PI: f64 = 3.141592653589793;
const MAX_USERS: u32 = 1_000_000;  // 숫자 구분자 사용 가능
const APP_NAME: &str = "Rust 학습";

/// 주어진 숫자가 THRESHOLD보다 큰지 확인하는 함수
///
/// # Arguments
/// * `n` - 비교할 숫자
///
/// # Returns
/// * `bool` - n이 THRESHOLD보다 크면 true
fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

/// 원의 넓이를 계산하는 함수
///
/// # Arguments
/// * `radius` - 원의 반지름
///
/// # Returns
/// * `f64` - 원의 넓이
fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 전역 상수 사용
    println!("=== 전역 상수 ===");
    println!("앱 이름: {}", APP_NAME);
    println!("THRESHOLD: {}", THRESHOLD);
    println!("PI: {}", PI);
    println!("최대 사용자 수: {}", MAX_USERS);

    // 2. 상수를 함수에서 사용
    println!("\n=== 함수에서 상수 사용 ===");
    println!("5는 큰 수인가? {}", is_big(5));
    println!("15는 큰 수인가? {}", is_big(15));

    let radius = 5.0;
    println!("반지름 {}인 원의 넓이: {:.2}", radius, circle_area(radius));

    // 3. 지역 상수 (함수 내부에서도 선언 가능)
    const LOCAL_CONST: i32 = 42;
    println!("\n지역 상수: {}", LOCAL_CONST);

    // 4. let과 const의 차이
    println!("\n=== let vs const ===");

    // let: 런타임에 값 결정 가능
    let runtime_value = std::time::SystemTime::now();
    println!("런타임 값 (let): {:?}", runtime_value);

    // const: 컴파일 타임에 값이 결정되어야 함
    // const COMPILE_TIME: std::time::SystemTime = std::time::SystemTime::now();
    // 위 코드는 에러! - 상수는 컴파일 타임에 값이 결정되어야 합니다.

    // 5. 상수는 섀도잉 불가
    // const THRESHOLD: i32 = 20;  // 같은 스코프에서 재선언 불가

    // 6. 상수 표현식
    const SECONDS_PER_MINUTE: u32 = 60;
    const MINUTES_PER_HOUR: u32 = 60;
    const SECONDS_PER_HOUR: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

    println!("\n1시간 = {}초", SECONDS_PER_HOUR);

    // 7. 상수 vs 불변 변수 정리
    println!("\n=== 상수 vs 불변 변수 ===");
    println!("const: 컴파일 타임 상수, 타입 필수, 전역 선언 가능");
    println!("let: 런타임 값 가능, 타입 추론 가능, 함수 스코프");
}
