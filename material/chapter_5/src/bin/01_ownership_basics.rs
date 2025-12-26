//! 소유권 기본 개념
//!
//! 러스트는 소유권(Ownership)을 통해 메모리를 관리합니다.
//! 소유권 규칙:
//! 1. 모든 값은 소유자(Owner)가 존재
//! 2. 한 번에 하나의 소유자만 존재
//! 3. 소유자가 스코프를 벗어나면 값은 메모리에서 해제

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 스코프와 소유권
    println!("=== 스코프와 소유권 ===");
    {
        let x = 5;  // x가 스코프에 들어옴
        println!("x = {}", x);
    }  // x가 스코프를 벗어남 -> 메모리에서 해제
    // println!("{}", x);  // 컴파일 에러! x는 더 이상 유효하지 않음

    // 2. String 타입과 소유권
    println!("\n=== String 타입과 소유권 ===");
    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    // 3. 소유권 이동 (Move)
    println!("\n=== 소유권 이동 (Move) ===");
    let s2 = String::from("hello");
    let s3 = s2;  // s2의 소유권이 s3로 이동
    // println!("{}", s2);  // 컴파일 에러! s2는 더 이상 유효하지 않음
    println!("s3 = {}", s3);

    // 4. 복사 (Copy) - 스택에 저장되는 타입
    println!("\n=== 복사 (Copy) ===");
    let a = 5;
    let b = a;  // a가 복사됨 (i32는 Copy 트레이트 구현)
    println!("a = {}, b = {}", a, b);  // 둘 다 사용 가능

    // 5. clone() - 힙 데이터 깊은 복사
    println!("\n=== clone() ===");
    let s4 = String::from("hello");
    let s5 = s4.clone();  // 힙 데이터까지 복사
    println!("s4 = {}, s5 = {}", s4, s5);  // 둘 다 사용 가능

    // 6. 함수와 소유권
    println!("\n=== 함수와 소유권 ===");
    let s6 = String::from("hello");
    takes_ownership(s6);  // s6의 소유권이 함수로 이동
    // println!("{}", s6);  // 컴파일 에러! s6는 더 이상 유효하지 않음

    let num = 5;
    makes_copy(num);  // num이 복사됨
    println!("num = {}", num);  // num은 여전히 사용 가능

    // 7. 소유권 돌려주기
    println!("\n=== 소유권 돌려주기 ===");
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);  // 소유권이 s7 -> 함수 -> s8로 이동
    println!("s8 = {}", s8);
}

/// 소유권을 가져가는 함수
fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}  // s가 스코프를 벗어나면서 drop 호출, 메모리 해제

/// 복사가 일어나는 함수 (i32는 Copy 트레이트 구현)
fn makes_copy(num: i32) {
    println!("makes_copy: {}", num);
}

/// 소유권을 받아서 돌려주는 함수
fn takes_and_gives_back(s: String) -> String {
    println!("takes_and_gives_back: {}", s);
    s  // 소유권을 반환
}
