//! 연습문제 6: 상수 (Constants)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.
//! 실행 방법: cargo run --bin 06_constants_quiz

// =====================
// 문제 1: 전역 상수 선언
// 원주율 PI를 3.14159로 선언하세요.
// (상수는 반드시 타입을 명시해야 합니다!)
// =====================
/* TODO: const PI를 선언하세요 (f64 타입) */
const PI: f64 = 3.14159;

// =====================
// 문제 2: 정수 상수 선언
// 최대 점수 MAX_SCORE를 100으로 선언하세요.
// =====================
/* TODO: const MAX_SCORE를 선언하세요 (i32 타입) */
const MAX_SCORE: i32 = 100;

// =====================
// 문제 3: 상수 표현식
// 1분의 초(60)와 1시간의 분(60)을 상수로 선언하고,
// 이를 곱하여 1시간의 초를 상수로 선언하세요.
// =====================
/* TODO: SECONDS_PER_MINUTE를 60으로 선언하세요 (u32) */
const SECONDS_PER_MINUTE: u32 = 60;
/* TODO: MINUTES_PER_HOUR를 60으로 선언하세요 (u32) */
const MINUTES_PER_HOUR: u32 = 60;
/* TODO: SECONDS_PER_HOUR를 위 두 상수의 곱으로 선언하세요 (u32) */
const SECONDS_PER_HOUR: u32 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

/// 주어진 점수가 합격인지 확인하는 함수
/// 합격 기준: 60점 이상
fn is_passing(score: i32) -> bool {
    // =====================
    // 문제 4: 함수 내 상수 사용
    // 합격 기준 상수 PASSING_SCORE를 60으로 선언하고,
    // score가 이 기준 이상인지 반환하세요.
    // =====================
    /* TODO: 지역 상수 PASSING_SCORE를 선언하세요 */
    const PASSING_SCORE: i32 = 60;
    /* TODO: score >= PASSING_SCORE를 반환하세요 */
    score >= PASSING_SCORE
}

/// 원의 넓이를 계산하는 함수
fn circle_area(radius: f64) -> f64 {
    // =====================
    // 문제 5: 전역 상수 PI 사용
    // PI * radius * radius를 반환하세요.
    // =====================
    /* TODO: 원의 넓이 공식을 구현하세요 */
    PI * radius * radius
}

fn main() {
    // 문제 1, 2 테스트
    println!("PI = {}", PI);
    println!("MAX_SCORE = {}", MAX_SCORE);

    // 문제 3 테스트
    println!("1시간 = {}초", SECONDS_PER_HOUR);

    // 문제 4 테스트
    println!("75점 합격 여부: {}", is_passing(75));
    println!("50점 합격 여부: {}", is_passing(50));

    // 문제 5 테스트
    let radius = 5.0;
    println!("반지름 {}인 원의 넓이: {:.2}", radius, circle_area(radius));
}
