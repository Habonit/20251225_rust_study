//! while 반복문 - 조건부 반복
//!
//! while 문은 조건이 만족되는 동안 코드가 계속 반복 실행됩니다.
//! 러스트는 증감 연산자(++, --)가 없으므로 += 또는 -= 를 사용합니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 while 루프
    println!("=== 기본 while 루프 ===");
    let mut x = 0;  // mut 키워드 필수!
    while x < 5 {
        print!("{},", x);
        x += 1;  // 증감 연산자 없음, x++ 불가
    }
    println!();  // 출력: 0,1,2,3,4,

    // 2. 역순 카운트다운
    println!("\n=== 역순 카운트다운 ===");
    let mut count = 5;
    while count > 0 {
        print!("{},", count);
        count -= 1;
    }
    println!("발사!");

    // 3. while로 조건 검사
    println!("\n=== 조건 검사 ===");
    let mut num = 1;
    while num <= 100 {
        if num % 15 == 0 {
            println!("100 이하에서 15의 배수: {}", num);
        }
        num += 1;
    }

    // 4. while과 break
    println!("\n=== while과 break ===");
    let mut i = 0;
    while i < 10 {
        if i == 5 {
            println!("5에서 중단!");
            break;
        }
        print!("{},", i);
        i += 1;
    }
    println!();

    // 5. while과 continue
    println!("\n=== while과 continue ===");
    let mut j = 0;
    while j < 10 {
        j += 1;
        if j % 2 == 0 {
            continue;  // 짝수는 건너뜀
        }
        print!("{},", j);
    }
    println!();  // 출력: 1,3,5,7,9,
}
