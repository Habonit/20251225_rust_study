//! while 반복문 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 while 루프
    // 0부터 4까지 출력
    println!("=== 기본 while 루프 ===");
    let mut x = 0;  // mut 키워드 필수!
    while x < 5 /* TODO: x < 5 조건 */ {
        print!("{},", x);
        x += 1;
        /* TODO: x를 1 증가 (러스트에는 x++ 없음) */
    }
    println!();  // 기대 출력: 0,1,2,3,4,

    // 2. 역순 카운트다운
    // 5부터 1까지 역순 출력 후 "발사!" 출력
    println!("\n=== 역순 카운트다운 ===");
    let mut count = 5;
    while count > 0/* TODO: count > 0 조건 */ {
        print!("{},", count);
        count -= 1;
        /* TODO: count를 1 감소 */
    }
    println!("발사!");  // 기대 출력: 5,4,3,2,1,발사!

    // 3. break로 루프 탈출
    // 0부터 시작해서 5에서 멈추기
    println!("\n=== break로 루프 탈출 ===");
    let mut i = 0;
    while i < 10 {
        if i == 5 {
            println!("5에서 중단!");
            /* TODO: 루프 탈출 */
            break;
        }
        print!("{},", i);
        i += 1;
    }
    println!();
}
