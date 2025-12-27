//! 구조체 기초 - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

// 문제 1: Debug 트레이트를 파생하여 구조체를 정의하세요
/* TODO: derive 애트리뷰트 */
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    // 문제 2: Book 구조체의 인스턴스를 생성하세요
    /* TODO: Book 인스턴스 생성 (title: "Rust 입문", author: "김러스트", pages: 300) */
    let book = Book {
        title: String::from("Rust 입문"),
        author: String::from("김러스트"),
        pages: 300,
    };
    println!("문제 2 결과: {:?}", book);

    // 문제 3: 가변 구조체를 만들고 pages를 350으로 변경하세요
    /* TODO: mut 키워드 추가 */
    let mut book2 = Book {
        title: String::from("고급 Rust"),
        author: String::from("박프로"),
        pages: 400,
    };
    /* TODO: book2.pages를 350으로 변경 */
    book2.pages = 350;
    println!("문제 3 결과: pages = {}", book2.pages);

    // 문제 4: 필드 초기화 축약 문법을 사용하세요
    // 힌트: 변수명과 필드명이 같으면 생략 가능
    let title = String::from("Rust 실전");
    let author = String::from("이개발");
    let pages = 250;
    /* TODO: 축약 문법으로 Book 생성 (title: title -> title) */
    let book3 = Book {
        title,
        author,
        pages,
    };
    println!("문제 4 결과: {:?}", book3);

    // 문제 5: 구조체 업데이트 문법을 사용하세요
    // 힌트: ..book3 으로 나머지 필드 복사
    /* TODO: title만 새로 지정하고 나머지는 book3에서 복사 */
    let book4 = Book {
        title: String::from("Rust 마스터"),
        ..book3
    };
    println!("문제 5 결과: {:?}", book4);

    // 문제 6: 구조체 필드에 개별적으로 접근하세요
    let my_book = Book {
        title: String::from("나의 책"),
        author: String::from("나"),
        pages: 100,
    };
    /* TODO: my_book.title, my_book.author, my_book.pages 출력 */
    println!(
        "문제 6 결과: 제목={}, 저자={}, 페이지={}",
        my_book.title,
        my_book.author,
        my_book.pages,
    );
}
