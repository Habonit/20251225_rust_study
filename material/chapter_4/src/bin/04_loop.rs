//! loop - 무한 반복문
//!
//! loop는 무한 루프를 만들 때 사용됩니다.
//! break로 루프를 종료하고, break 뒤에 값을 넣어 리턴할 수 있습니다.

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 loop와 break
    println!("=== 기본 loop와 break ===");
    let mut x = 0;
    loop {
        x += 1;
        if x == 5 {
            break;
        }
        print!("{},", x);
    }
    println!();  // 출력: 1,2,3,4,

    // 2. loop에서 값 리턴
    println!("\n=== loop에서 값 리턴 ===");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // 20을 리턴
        }
    };
    println!("result = {}", result);

    // 3. loop로 검색
    println!("\n=== loop로 검색 ===");
    let target = 7;
    let mut num = 0;
    let found = loop {
        num += 1;
        if num == target {
            break true;
        }
        if num > 100 {
            break false;
        }
    };
    println!("{}을(를) 찾음: {}", target, found);

    // 4. 중첩 loop와 라벨
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
            break 'outer;  // 외부 루프 종료
        }
    }

    // 5. loop vs while true
    println!("\n=== loop vs while true ===");
    // while true { ... } 대신 loop { ... } 사용 권장
    // loop가 더 명확하고 값을 리턴할 수 있음
    let mut i = 0;
    let value = loop {
        i += 1;
        if i >= 3 {
            break "완료";
        }
    };
    println!("상태: {}", value);
}
