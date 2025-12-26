//! match - 패턴 매칭
//!
//! match는 다른 언어의 switch...case와 유사하지만 더 강력합니다.
//! _ 는 나머지 모든 경우를 나타내며, 모든 케이스를 처리해야 합니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 match
    println!("=== 기본 match ===");
    let name = "John";
    match name {
        "John" => println!("Hello, John!"),
        "Mary" => println!("Hello, Mary!"),
        _ => println!("Hello, stranger!"),  // 나머지 경우
    }

    // 2. match로 값 리턴
    println!("\n=== match로 값 리턴 ===");
    let name = "Mary";
    let greeting = match name {
        "John" => "Hello, John!",
        "Mary" => "Hello, Mary!",
        _ => "Hello, stranger!",
    };
    println!("{}", greeting);

    // 3. 숫자 매칭
    println!("\n=== 숫자 매칭 ===");
    let number = 3;
    let text = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("{} = {}", number, text);

    // 4. 범위 매칭
    println!("\n=== 범위 매칭 ===");
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("점수 {}: 학점 {}", score, grade);

    // 5. 여러 패턴 매칭 (|)
    println!("\n=== 여러 패턴 매칭 ===");
    let day = 6;
    let day_type = match day {
        1 | 2 | 3 | 4 | 5 => "평일",
        6 | 7 => "주말",
        _ => "잘못된 요일",
    };
    println!("{}번째 날: {}", day, day_type);

    // 6. match와 조건 가드
    println!("\n=== match와 조건 가드 ===");
    let num = 4;
    let description = match num {
        n if n < 0 => "음수",
        n if n == 0 => "영",
        n if n % 2 == 0 => "양수 짝수",
        _ => "양수 홀수",
    };
    println!("{}: {}", num, description);
}
