//! 값 출력하기 - println! 매크로
//!
//! println!은 콘솔에 값을 출력하는 매크로입니다.
//! `!`는 매크로를 호출한다는 의미입니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 문자열 출력
    println!("Hello, world!");

    // 2. 변수 값 출력 - {} 플레이스홀더 사용
    let x: f64 = 1.0;
    let y = 10; // 타입 추론 (i32)
    println!("x = {}, y = {}", x, y);

    // 3. 여러 가지 출력 방법
    let name = "Rust";
    let version = 2021;

    // 순서대로 출력
    println!("{} edition: {}", name, version);

    // 인덱스 지정
    println!("{0}은 {1}년에 출시되었습니다. {0} 최고!", name, version);

    // 이름 지정 (Rust 1.58+)
    println!("{language} {year}", language = "Rust", year = 2021);

    // 4. 디버그 출력 - {:?} 사용
    let numbers = [1, 2, 3, 4, 5];
    println!("배열 디버그 출력: {:?}", numbers);

    // 5. 줄바꿈 없이 출력 - print! 매크로
    print!("줄바꿈 없이 ");
    print!("출력합니다.");
    println!(); // 빈 줄바꿈
}
