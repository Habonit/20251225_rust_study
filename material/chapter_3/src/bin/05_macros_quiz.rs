//! 매크로 (Macro) - 코드를 생성하는 코드 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

/// 가변 길이 인수의 합을 계산하는 매크로
/// 힌트: macro_rules! 로 매크로를 정의합니다
macro_rules! get_sum {
    // 쉼표로 구분된 임의의 개수의 식을 입력으로 받음
    // $($x:expr),* 는 0개 이상의 표현식을 매치합니다
    ($($x:expr),*) => {{
        // 식들을 벡터에 담음
        let args = vec![$($x),*];
        // 벡터의 합을 계산
        args.iter().sum::<i32>()
    }};
}

/// 디버그 출력 매크로
macro_rules! debug_print {
    ($val:expr) => {
        // 힌트: stringify!는 표현식을 문자열로 변환합니다
        /* TODO: stringify! 매크로 사용 */
        println!("[DEBUG] {} = {:?}", stringify!($val), $val);
    };
}

/// 여러 변수를 한 번에 출력하는 매크로
macro_rules! print_vars {
    // 힌트: $name:ident는 식별자를 매치합니다
    ($($name:ident),*) => {
        $(
            println!("{} = {}", stringify!($name), $name);
        )*
    };
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 표준 라이브러리 매크로
    println!("=== 표준 라이브러리 매크로 ===");

    // println! - 가변 개수 인수
    println!("인수 1개");
    // 힌트: {}는 포맷 플레이스홀더입니다
    /* TODO: 42 */
    println!("인수 2개: {}", 42);
    println!("{} + {} = {}", 1, 2, 3);

    // vec! - 벡터 생성
    // 힌트: vec![값들] 또는 vec![초기값; 개수]
    /* TODO: 1, 2, 3을 담은 벡터 생성 */
    let v1 = vec![1, 2, 3];
    let v2 = vec![0; 5];  // [0, 0, 0, 0, 0]
    println!("vec!: {:?}, {:?}", v1, v2);

    // format! - 문자열 포맷팅
    let name = "Rust";
    // 힌트: format!은 println!과 같은 형식이지만 String을 반환합니다
    /* TODO: format! 매크로로 "Hello, {name}!" 생성 */
    let formatted = format!("Hello, {}!", name);
    println!("format!: {}", formatted);

    // 2. 사용자 정의 매크로: get_sum!
    println!("\n=== get_sum! 매크로 ===");
    // 힌트: 매크로 호출은 함수 호출과 비슷하지만 !가 붙습니다
    /* TODO: get_sum! 매크로 호출 */
    println!("get_sum!(1, 2) = {}", get_sum!(1, 2));
    println!("get_sum!(1, 2, 3) = {}", get_sum!(1, 2, 3));
    println!("get_sum!(1, 2, 3, 4, 5) = {}", get_sum!(1, 2, 3, 4, 5));

    // 3. 사용자 정의 매크로: debug_print!
    println!("\n=== debug_print! 매크로 ===");
    let x = 42;
    let numbers = vec![1, 2, 3];
    /* TODO: debug_print! 매크로로 x 출력 */
    debug_print!(x);
    debug_print!(numbers);
    debug_print!(1 + 2 * 3);

    // 4. 사용자 정의 매크로: print_vars!
    println!("\n=== print_vars! 매크로 ===");
    let a = 10;
    let b = 20;
    let c = 30;
    /* TODO: print_vars! 매크로로 a, b, c 한번에 출력 */
    print_vars!(a, b, c);

    // 5. 매크로와 함수의 차이
    println!("\n=== 매크로 vs 함수 ===");
    println!("매크로: 컴파일 타임에 코드 생성");
    println!("함수: 런타임에 실행");
    println!("매크로: 가변 개수 인수 가능");
    println!("함수: 고정된 개수의 인수");

    // 6. 유용한 표준 매크로
    println!("\n=== 유용한 표준 매크로 ===");

    // dbg! - 디버깅용 출력
    // 힌트: dbg!는 표현식의 값을 출력하고 그 값을 반환합니다
    /* TODO: dbg! 매크로로 2 + 2 계산 */
    let result = dbg!(2 + 2);
    println!("result = {}", result);

    // assert! - 조건 확인
    // 힌트: 조건이 false면 패닉 발생
    /* TODO: assert! 매크로로 1 + 1 == 2 확인 */
    assert!(1 + 1 == 2);
    println!("assert! 통과");

    // panic! - 프로그램 중단 (주석 처리)
    // panic!("이 코드는 실행되면 패닉!");

    // todo! - 미구현 표시 (주석 처리)
    // todo!("이 기능은 아직 구현되지 않음");

    // unreachable! - 도달 불가 코드 표시
    let value = 1;
    match value {
        1 => println!("1입니다"),
        // 힌트: unreachable!은 절대 실행되지 않아야 하는 코드에 사용
        /* TODO: unreachable! 매크로 사용 */
        _ => unreachable!(),
    }
}
