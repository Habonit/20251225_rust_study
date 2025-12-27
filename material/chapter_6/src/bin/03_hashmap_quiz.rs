//! 해시맵(HashMap) - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

use std::collections::HashMap;

fn main() {
    // 문제 1: 빈 해시맵을 생성하고 "apple" -> 3, "banana" -> 5를 추가하세요
     /* TODO: 빈 해시맵 생성 */
    let mut fruits: HashMap<&str, i32> = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("banana", 5);
    /* TODO: "apple"과 3을 insert */
    /* TODO: "banana"와 5를 insert */
    println!("문제 1 결과: {:?}", fruits);

    // 문제 2: get() 메서드로 "apple"의 값을 조회하세요
    /* TODO: get("apple") 결과 패턴 매칭 */ 
    if let Some(count) = fruits.get("apple") {
        println!("문제 2 결과: 사과 개수는 {}개입니다.", count);
    }

    // 문제 3: entry().or_insert()로 "orange"가 없으면 10을 추가하세요
    /* TODO: entry와 or_insert 사용 */
    fruits.entry("orange").or_insert(10);
    println!("문제 3 결과: {:?}", fruits);

    // 문제 4: 기존 키 "apple"의 값을 100으로 업데이트하세요
    /* TODO: insert로 기존 값 덮어쓰기 */
    fruits.insert("apple", 100);
    println!("문제 4 결과: {:?}", fruits);

    // 문제 5: remove()로 "banana"를 삭제하세요
    /* TODO: remove 메서드 사용 */
    let removed = fruits.remove("banana");
    println!("문제 5 결과: 삭제된 값 = {:?}, 남은 해시맵 = {:?}", removed, fruits);

    // 문제 6: 해시맵을 순회하며 모든 키-값 쌍을 출력하세요
    /* TODO: 키 */
    /* TODO: 값 */
    let scores: HashMap<&str, i32> = HashMap::from([
        ("국어", 85),
        ("수학", 90),
        ("영어", 78),
    ]);
    println!("문제 6 결과:");
    for (subject, score) in &scores {
        println!("  {}: {}점", subject, score);
    }
}
