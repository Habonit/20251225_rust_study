//! 벡터(Vec) - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

fn main() {
    // 문제 1: 빈 벡터를 생성하고 1, 2, 3을 추가하세요
    /* TODO: 빈 벡터 생성 */
    let mut numbers: Vec<i32> = Vec::new();
    /* TODO: push 메서드로 1 추가 */
    /* TODO: push 메서드로 2 추가 */
    /* TODO: push 메서드로 3 추가 */
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("문제 1 결과: {:?}", numbers); // [1, 2, 3]

    // 문제 2: vec! 매크로로 ["a", "b", "c"] 벡터를 생성하세요
    /* TODO: vec! 매크로 사용 */
    let letters = vec!["a", "b", "c"];
    println!("문제 2 결과: {:?}", letters); // ["a", "b", "c"]

    // 문제 3: get() 메서드로 안전하게 요소에 접근하세요
    let scores = vec![85, 90, 78];
    /* TODO: get(1)의 결과를 패턴 매칭 */
    if let Some(score) = scores.get(1) {
        println!("문제 3 결과: 두 번째 점수는 {}입니다.", score);
    }

    // 문제 4: pop()으로 마지막 요소를 제거하고 출력하세요
    let mut stack = vec![10, 20, 30];
    /* TODO: pop 메서드 사용 */
    let popped : i32;
    if let Some(value) = stack.pop() {
        popped = value;
    } else {
        popped = 0;
    }
    println!("문제 4 결과: pop된 값 = {:?}, 남은 벡터 = {:?}", popped, stack);

    // 문제 5: 동일한 값 0으로 채워진 길이 5의 벡터를 생성하세요
    /* TODO: vec![값; 길이] 형태 사용 */
    let zeros = vec![0; 5];
    println!("문제 5 결과: {:?}", zeros); // [0, 0, 0, 0, 0]

    // 문제 6: remove()로 인덱스 1의 요소를 제거하세요
    let mut fruits = vec!["사과", "바나나", "오렌지"];
    /* TODO: remove 메서드로 인덱스 1 제거 */
    let removed = fruits.remove(1);
    println!("문제 6 결과: 제거된 값 = {}, 남은 벡터 = {:?}", removed, fruits);
}
