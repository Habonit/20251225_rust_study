//! 튜플 구조체 - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

// 문제 1: RGB 색상을 나타내는 튜플 구조체를 정의하세요 (u8 3개)
/* TODO: struct Rgb(u8, u8, u8); */
/* TODO: 세 개의 u8 타입 */
struct Rgb(
    u8,
    u8,
    u8
);

// 문제 2: 2D 좌표를 나타내는 튜플 구조체를 정의하세요 (f64 2개)
/* TODO: struct Coordinate(f64, f64); */
/* TODO: 두 개의 f64 타입 */
struct Coordinate(
    f64,
    f64
);

impl Coordinate {
    // 문제 3: 연관 함수 new를 구현하세요
    fn new(x: f64, y: f64) -> Self {
        /* TODO: Coordinate(x, y) 반환 */
        Coordinate(x, y)
    }

    // 문제 4: x 좌표를 반환하는 메서드를 구현하세요
    fn x(&self) -> f64 {
        /* TODO: self.0 반환 */
        self.0
    }

    // 문제 5: y 좌표를 반환하는 메서드를 구현하세요
    fn y(&self) -> f64 {
        /* TODO: self.1 반환 */
        self.1
    }
}

fn main() {
    // 문제 1 테스트: RGB 인스턴스 생성
    /* TODO: Rgb(255, 0, 0) */
    let red = Rgb(255, 0, 0);
    println!("문제 1 결과: R={}, G={}, B={}", red.0, red.1, red.2);

    // 문제 2 테스트: 구조분해로 값 추출
    let Rgb(r, g, b) = Rgb(100, 150, 200);
    /* TODO: let Rgb(r, g, b) = color; */
    println!("문제 2 결과: R={}, G={}, B={}", r, g, b);

    // 문제 3 테스트: new 함수 사용
    let point = Coordinate::new(3.5, 7.2);
    println!("문제 3 결과: ({}, {})", point.0, point.1);

    // 문제 4, 5 테스트: x(), y() 메서드 사용
    let coord = Coordinate::new(10.0, 20.0);
    println!("문제 4 결과: x = {}", coord.x());
    println!("문제 5 결과: y = {}", coord.y());

    // 문제 6: 인덱스로 튜플 필드에 접근하세요
    let point3d = (1.0, 2.0, 3.0); // 일반 튜플
    /* TODO: point3d.0, point3d.1, point3d.2 */
    println!("문제 6 결과: x={}, y={}, z={}",
        point3d.0,
        point3d.1,
        point3d.2
    );
}
