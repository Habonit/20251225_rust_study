//! 열거형(Enum) - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

fn main() {
    // 문제 1: Option에서 match로 값을 추출하세요
    /* TODO: Some 패턴 */
    /* TODO: None 패턴 */ 
    let maybe_value: Option<i32> = Some(100);
    match maybe_value {
        Some(v) => println!("문제 1 결과: 값은 {}입니다.", v),
        None => println!("값이 없습니다."),
    }

    // 문제 2: if let으로 Option 값을 처리하세요
    let name: Option<&str> = Some("Rust");
    /* TODO: Some 패턴 */
    if let  Some(n)= name {
        println!("문제 2 결과: 이름은 {}입니다.", n);
    }

    // 문제 3: Result의 Ok와 Err를 처리하세요
    let result: Result<i32, &str> = Ok(42);
    /* TODO: Ok 패턴 */
    /* TODO: Err 패턴 */
    match result {
        Ok(value) => println!("문제 3 결과: 성공! 값은 {}", value),
        Err(e) => println!("실패: {}", e),
    }

    // 문제 4: unwrap_or()로 기본값을 제공하세요
    let empty: Option<i32> = None;
    /* TODO: unwrap_or로 기본값 0 제공 */
    let value = empty.unwrap_or(0);
    println!("문제 4 결과: {}", value); // 0

    // 문제 5: Result에서 unwrap_or()를 사용하세요
    let error_result: Result<i32, &str> = Err("오류 발생");
    /* TODO: unwrap_or로 기본값 -1 제공 */
    let safe_value = error_result.unwrap_or(-1);
    println!("문제 5 결과: {}", safe_value); // -1

    // 문제 6: Option의 is_some()과 is_none()을 사용하세요
    let some_value: Option<i32> = Some(10);
    let none_value: Option<i32> = None;
    /* TODO */
    /* TODO */
    println!("문제 6 결과:");
    println!("  some_value.is_some() = {}", some_value.is_some());  // true
    println!("  none_value.is_none() = {}", none_value.is_none());  // true
}
