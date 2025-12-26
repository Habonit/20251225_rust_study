//! 리턴값 - return 키워드와 튜플 리턴 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

/// return 키워드를 사용한 함수
fn add_with_return(a: i32, b: i32) -> i32 {
    /* TODO: return 키워드로 a + b 리턴 */
}

/// return 키워드 생략 (세미콜론 없음!)
/// 힌트: 마지막 표현식에 세미콜론을 붙이지 않으면 자동으로 리턴됩니다
fn add_without_return(a: i32, b: i32) -> i32 {
    /* TODO: return 없이 리턴 */
}

/// 조건에 따라 다른 값 리턴 (early return)
/// 힌트: 음수일 때는 부호를 바꿔서 리턴
fn absolute(n: i32) -> i32 {
    if n < 0 {
        /* TODO: early return으로 -n 리턴 */
    }
    n
}

/// 두 값을 교환하여 리턴 (튜플 리턴)
/// 힌트: 튜플은 (value1, value2) 형태입니다
fn swap(a: i32, b: i32) -> /* TODO: 튜플 리턴 타입 */ {
    /* TODO: b, a 순서로 튜플 리턴 */
}

/// 나눗셈의 몫과 나머지를 함께 리턴
fn div_mod(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = /* TODO: 나머지 연산 */;
    /* TODO: quotient와 remainder를 튜플로 리턴 */
}

/// 최솟값과 최댓값을 함께 리턴
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        /* TODO: (min, max) 순서로 리턴 */
    } else {
        (b, a)
    }
}

/// 리턴값이 없는 함수 (Unit 타입)
fn print_message(msg: &str) {
    println!("{}", msg);
}

/// Unit 타입을 명시적으로 표기
/// 힌트: Unit 타입은 ()로 표기합니다
fn print_message_explicit(msg: &str) -> /* TODO: Unit 타입 */ {
    println!("{}", msg);
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. return 키워드 사용 vs 생략
    println!("=== return 키워드 ===");
    println!("with return: {}", add_with_return(3, 5));
    println!("without return: {}", add_without_return(3, 5));

    // 2. early return
    println!("\n=== early return ===");
    println!("absolute(-5) = {}", absolute(-5));
    println!("absolute(5) = {}", absolute(5));

    // 3. 튜플 리턴
    println!("\n=== 튜플 리턴 ===");
    // 힌트: 튜플 구조분해를 사용합니다
    let /* TODO: (x, y) 형태로 구조분해 */ = swap(1, 2);
    println!("swap(1, 2) = ({}, {})", x, y);

    // 4. 여러 값 리턴
    println!("\n=== 여러 값 리턴 ===");
    let (quotient, remainder) = /* TODO: div_mod(17, 5) 호출 */;
    println!("17 / 5 = {} 나머지 {}", quotient, remainder);

    let (min, max) = min_max(10, 3);
    println!("min_max(10, 3) = (min: {}, max: {})", min, max);

    // 5. 튜플 인덱스로 접근
    println!("\n=== 튜플 인덱스 접근 ===");
    let result = div_mod(20, 7);
    // 힌트: 튜플 요소는 .0, .1로 접근합니다
    println!("몫: {}, 나머지: {}", /* TODO: 인덱스 접근 */, /* TODO */);

    // 6. Unit 타입 함수
    println!("\n=== Unit 타입 함수 ===");
    print_message("암시적 Unit 타입");
    print_message_explicit("명시적 Unit 타입");
}
