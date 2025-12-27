//! 트레이트(Trait) - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

// 문제 1: Area 트레이트를 정의하세요
/* TODO: trait Area { fn area(&self) -> f64; } */
trait Area {
    fn area(&self) -> f64;
}
// 문제 2: Describe 트레이트를 기본 구현과 함께 정의하세요
/* TODO: String::from("설명 없음") 반환 */
trait Describe {
    fn describe(&self) -> String {
        String::from("설명 없음")
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

// 문제 3: Rectangle에 Area 트레이트를 구현하세요
/* TODO: impl Area for Rectangle { ... } */
impl Area for Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 문제 4: Circle에 Area 트레이트를 구현하세요
/* TODO: impl Area for Circle { ... } */
impl Area for Circle{
    fn area(&self) -> f64 {
        const PI: f64 = 3.14;
        PI * self.radius * self.radius
    }
}

// 문제 5: Rectangle에 Describe 트레이트를 구현하세요 (기본 구현 오버라이드)
impl Describe for Rectangle {
    fn describe(&self) -> String {
        /* TODO: format!("가로 {}, 세로 {}인 직사각형", self.width, self.height) */
        format!("가로 {}, 세로 {}인 직사각형", self.width, self.height)
    }
}

// Circle은 기본 구현 사용
impl Describe for Circle {}

// 문제 6: 트레이트를 파라미터로 받는 함수를 완성하세요
/* TODO: shape: &impl Area */
fn print_area(shape: &impl Area) {
    println!("넓이: {}", shape.area());
}

fn main() {
    let rect = Rectangle { width: 10.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };

    // 문제 3 테스트
    println!("문제 3 결과: 직사각형 넓이 = {}", rect.area()); // 50

    // 문제 4 테스트
    println!("문제 4 결과: 원 넓이 = {:.2}", circle.area()); // 약 28.27

    // 문제 5 테스트
    println!("문제 5 결과: {}", rect.describe());

    // 문제 2 테스트 (기본 구현)
    println!("문제 2 결과: {}", circle.describe()); // "설명 없음"

    // 문제 6 테스트
    print!("문제 6 결과: ");
    print_area(&rect);
    print!("문제 6 결과: ");
    print_area(&circle);
}
