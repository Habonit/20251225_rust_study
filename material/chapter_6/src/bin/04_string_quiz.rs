//! 문자열 타입 - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

fn main() {
    // 문제 1: &str을 String으로 변환하세요
    let greeting: &str = "Hello";
    /* TODO: to_string() 또는 String::from() 사용 */
    let owned: String = greeting.to_string();
    println!("문제 1 결과: {}", owned);

    // 문제 2: push_str()로 문자열을 추가하세요
    let mut message = String::from("Rust");
    /* TODO: push_str로 " is awesome!" 추가 */
    message.push_str(" is awesome!");
    println!("문제 2 결과: {}", message); // "Rust is awesome!"

    // 문제 3: format! 매크로로 두 문자열을 결합하세요
    let first = "Good";
    let second = "Morning";
    /* TODO: format! 매크로 사용 */
    let combined = format!("{} {}", first, second);
    println!("문제 3 결과: {}", combined); // "Good Morning"

    // 문제 4: push()로 단일 문자를 추가하세요
    let mut word = String::from("Hell");
    /* TODO: push로 'o' 문자 추가 */
    word.push('o');
    println!("문제 4 결과: {}", word); // "Hello"

    // 문제 5: String을 &str로 변환하세요
    let owned_string = String::from("Rust Programming");
    /* TODO: & 또는 as_str() 사용 */
    let borrowed: &str = &owned_string;
    println!("문제 5 결과: {}", borrowed);

    // 문제 6: + 연산자로 두 문자열을 연결하세요
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    /* TODO: + 연산자로 s1과 &s2 연결 */
    let s3 = s1 + &s2;
    println!("문제 6 결과: {}", s3); // "Hello, World!"
}
