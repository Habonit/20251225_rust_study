//! 파생 트레이트(Derive) - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

// 문제 1: Debug 트레이트를 파생하세요
/* TODO: #[derive(Debug)] */
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

// 문제 2: Debug와 Clone을 함께 파생하세요
/* TODO: #[derive(Debug, Clone)] */
#[derive(Debug, Clone)]
struct Article {
    headline: String,
    content: String,
}

// 문제 3: Debug, Clone, Copy를 모두 파생하세요 (힙 데이터 없음)
/* TODO: #[derive(Debug, Clone, Copy)] */
#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

// 문제 4: Debug, Clone, PartialEq를 파생하세요
/* TODO: #[derive(Debug, Clone, PartialEq)] */
#[derive(Debug, Clone, PartialEq)]
struct Score {
    value: i32,
}

fn main() {
    // 문제 1 테스트: Debug로 출력
    let book = Book {
        title: String::from("Rust 입문"),
        pages: 300,
    };
    println!("문제 1 결과: {:?}", book);

    // 문제 2 테스트: clone() 메서드 사용
    let article = Article {
        headline: String::from("오늘의 뉴스"),
        content: String::from("러스트가 인기를 끌고 있습니다."),
    };
    /* TODO: article.clone() */
    let article_copy = article.clone();
    println!("문제 2 결과: 원본 = {:?}", article);
    println!("문제 2 결과: 복사본 = {:?}", article_copy);

    // 문제 3 테스트: Copy로 자동 복사
    let coord1 = Coordinate { x: 10, y: 20 };
    let coord2 = coord1; // Copy 트레이트로 자동 복사
    println!("문제 3 결과: coord1 = {:?}", coord1); // coord1 여전히 사용 가능해야 함
    println!("문제 3 결과: coord2 = {:?}", coord2);

    // 문제 4 테스트: PartialEq로 비교
    let score1 = Score { value: 100 };
    let score2 = Score { value: 100 };
    let score3 = Score { value: 50 };
    /* TODO: score1 == score2, score1 == score3 */
    println!("문제 4 결과: score1 == score2 -> {}", score1 == score2);
    println!("문제 4 결과: score1 == score3 -> {}", score1 == score3);

    // 문제 5: Clone과 Copy의 차이를 이해하세요
    // String이 있어서 Copy 불가능한 구조체
    let _book2 = Book {
        title: String::from("고급 Rust"),
        pages: 500,
    };
    // let book3 = _book2; // 이렇게 하면 _book2 사용 불가 (Move)
    // println!("{:?}", _book2); // 에러 발생
    println!("문제 5: Book은 Copy가 불가능합니다 (String 포함)");

    // 문제 6: Pretty Debug 출력
    let book4 = Book {
        title: String::from("Rust 마스터"),
        pages: 600,
    };
    println!("문제 6 결과 (Pretty Debug):");
    println!("{:#?}", book4);
}
