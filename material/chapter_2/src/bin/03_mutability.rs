//! 불변성 (Immutability) - mut 키워드
//!
//! Rust의 모든 변수는 기본적으로 불변(immutable)입니다.
//! 가변 변수를 선언하려면 mut 키워드를 사용합니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 불변 변수 (기본)
    let x = 1;
    println!("불변 변수 x = {}", x);
    // x = 2;  // 컴파일 에러! cannot assign twice to immutable variable

    // 2. 가변 변수 (mut 키워드 사용)
    let mut y = 1;
    println!("가변 변수 y (초기값) = {}", y);

    y = 2;  // OK - mut 키워드가 있으므로 값 변경 가능
    println!("가변 변수 y (변경 후) = {}", y);

    // 3. 가변 변수의 연산
    let mut counter = 0;
    println!("counter 초기값: {}", counter);

    counter += 1;  // counter = counter + 1
    println!("counter += 1: {}", counter);

    counter += 10;
    println!("counter += 10: {}", counter);

    // 4. 불변 vs 가변의 차이 이해하기
    // 불변 변수는 안전성을 보장합니다.
    // 값이 변경되지 않으므로 예측 가능한 코드 작성이 가능합니다.
    let immutable_value = 100;
    // 이 값은 프로그램 전체에서 100임을 보장받습니다.
    println!("불변 값은 항상 동일: {}", immutable_value);

    // 5. mut는 필요할 때만 사용하세요
    // Rust 컴파일러는 불필요한 mut에 대해 경고를 표시합니다.
    let mut _unused_mut = 5;  // 경고: variable does not need to be mutable
    // _unused_mut를 변경하지 않으면 컴파일러가 경고를 보냅니다.
}
