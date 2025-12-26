//! for 반복문 - 범위 순회
//!
//! for 문은 값들을 차례로 순회할 때 사용합니다.
//! 러스트에서는 a..b (마지막 제외) 또는 a..=b (마지막 포함) 문법을 사용합니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 for 루프 (a..b - 마지막 값 제외)
    println!("=== 기본 for 루프 (6..10) ===");
    for i in 6..10 {
        print!("{},", i);
    }
    println!();  // 출력: 6,7,8,9,

    // 2. 마지막 값 포함 (a..=b)
    println!("\n=== 마지막 값 포함 (6..=10) ===");
    for i in 6..=10 {
        print!("{},", i);
    }
    println!();  // 출력: 6,7,8,9,10,

    // 3. 범위를 변수에 저장
    println!("\n=== 범위를 변수에 저장 ===");
    let num_range = 1..5;
    for i in num_range {
        print!("{},", i);
    }
    println!();

    // 4. 0부터 시작
    println!("\n=== 0부터 시작 ===");
    for i in 0..5 {
        print!("{},", i);
    }
    println!();  // 출력: 0,1,2,3,4,

    // 5. break와 continue
    println!("\n=== break와 continue ===");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;  // 짝수일 때는 스킵
        } else if i == 7 {
            break;  // 7일 때는 루프 종료
        }
        print!("{},", i);
    }
    println!();  // 출력: 1,3,5,

    // 6. 중첩 for 루프
    println!("\n=== 중첩 for 루프 (피라미드) ===");
    for i in 1..=3 {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}
