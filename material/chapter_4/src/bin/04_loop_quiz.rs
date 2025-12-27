//! loop 무한 반복문 퀴즈
//!
//! 빈 칸을 채워 코드를 완성하세요.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 loop와 break
    // 1부터 4까지 출력 후 5에서 종료
    println!("=== 기본 loop와 break ===");
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            /* TODO: 루프 종료 */
            break;
        }
        print!("{},", x);
    }
    println!();  // 기대 출력: 1,2,3,4,

    // 2. loop에서 값 리턴
    // counter가 10이 되면 counter * 2를 리턴
    println!("\n=== loop에서 값 리턴 ===");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            /* TODO: counter * 2 값을 리턴하며 종료 */
            break counter * 2;
        }
    };
    println!("result = {}", result);  // 기대 출력: result = 20

    // 3. 중첩 loop와 라벨
    // 외부 루프 2회, 각 외부 루프마다 내부 루프 2회 실행 후 'outer 라벨로 종료
    println!("\n=== 중첩 loop와 라벨 ===");
    let mut count = 0;
    'outer: loop {
        println!("외부 루프");
        let mut inner_count = 0;
        loop {
            println!("  내부 루프");
            inner_count += 1;
            if inner_count == 2 {
                break;  // 내부 루프만 종료
            }
        }
        count += 1;
        if count == 2 {
            /* TODO: 외부 루프 종료 (라벨 사용) */
            break 'outer;
        }
    }
}
