//! 연관 함수와 메서드 - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

#[derive(Debug)]
struct Calculator {
    value: i32,
}

impl Calculator {
    // 문제 1: 연관 함수 new를 완성하세요 (초기값 0)
    /* TODO: 반환 타입은 Self */
    fn new() -> Self {
        /* TODO: value가 0인 Calculator 반환 */
        Calculator { value: 0 }
    }

    // 문제 2: 값을 더하는 메서드를 완성하세요
    /* TODO: &mut self 사용 */
    fn add(&mut self, num: i32) {
        /* TODO: self.value에 num을 더하기 */
        self.value += num;
    }

    // 문제 3: 값을 빼는 메서드를 완성하세요
    /* TODO: &mut self 사용 */
    fn subtract(&mut self, num: i32) {
        /* TODO: self.value에서 num을 빼기 */
        self.value -= num;
    }

    // 문제 4: 현재 값을 반환하는 메서드를 완성하세요
    /* TODO: &self 사용 */
    fn get_value(&self) -> i32 {
        /* TODO: self.value 반환 */
        self.value
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 문제 5: 연관 함수로 정사각형을 생성하세요
    fn square(size: u32) -> Self {
        /* TODO: width와 height가 모두 size인 Rectangle 반환 */
        Rectangle { width: size, height: size }
    }

    // 문제 6: 넓이를 계산하는 메서드를 완성하세요
    fn area(&self) -> u32 {
        /* TODO: width * height 반환 */
        self.width * self.height
    }
}

fn main() {
    // 문제 1 테스트
    let mut calc = Calculator::new();
    println!("문제 1 결과: 초기값 = {}", calc.get_value()); // 0

    // 문제 2 테스트
    calc.add(10);
    println!("문제 2 결과: 10 더한 후 = {}", calc.get_value()); // 10

    // 문제 3 테스트
    calc.subtract(3);
    println!("문제 3 결과: 3 뺀 후 = {}", calc.get_value()); // 7

    // 문제 4 테스트
    let value = calc.get_value();
    println!("문제 4 결과: 현재 값 = {}", value); // 7

    // 문제 5 테스트
    let square = Rectangle::square(5);
    println!("문제 5 결과: {:?}", square); // Rectangle { width: 5, height: 5 }

    // 문제 6 테스트
    let rect = Rectangle { width: 10, height: 20 };
    println!("문제 6 결과: 넓이 = {}", rect.area()); // 200
}
