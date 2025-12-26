//! match 패턴 매칭 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 match
    // name이 "Alice"면 "Hello, Alice!", 그 외는 "Hello, stranger!" 출력
    println!("=== 기본 match ===");
    let name = "Alice";
    match name {
        /* TODO: "Alice" 패턴 매칭 */ 
        "Alice" => println!("Hello, Alice!"),
        _ => println!("Hello, stranger!"),
    }

    // 2. 범위 매칭으로 학점 계산
    // 90-100: A, 80-89: B, 70-79: C, 나머지: F
    println!("\n=== 범위 매칭 ===");
    let score = 85;
    let grade = match score {
        /* TODO: 90부터 100까지 범위 */ 
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        _ => "F",
    };
    println!("점수 {}: 학점 {}", score, grade);

    // 3. 여러 패턴 매칭 (|)
    // 1,2,3,4,5는 "평일", 6,7은 "주말"
    println!("\n=== 여러 패턴 매칭 ===");
    let day = 6;
    let day_type = match day {
        1..=5 => "평일",
         /* TODO: 6 또는 7 패턴 */
        6..=7 => "주말",
        _ => "잘못된 요일",
    };
    println!("{}번째 날: {}", day, day_type);
}
