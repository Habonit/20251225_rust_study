//! 이터레이터 - 퀴즈
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

fn main() {
    // 문제 1: map()을 사용하여 각 요소에 10을 더하세요
    let numbers = vec![1, 2, 3, 4, 5];
    /* TODO: map 사용하여 각 요소에 10 더하기 */
    let added: Vec<i32> = numbers.iter()
        .map(|x : &i32| -> i32 { x + 10 })
        .collect();
    println!("문제 1 결과: {:?}", added); // [11, 12, 13, 14, 15]

    // 문제 2: filter()를 사용하여 3보다 큰 요소만 선택하세요
    /* TODO: filter 사용하여 3보다 큰 요소 선택 */
    let values = vec![1, 5, 2, 8, 3, 9];
    let filtered: Vec<&i32> = values.iter()
        .filter(|x : &&i32| -> bool { **x > 3 })
        .collect();
    println!("문제 2 결과: {:?}", filtered); // [5, 8, 9]

    // 문제 3: filter와 map을 체이닝하여 홀수만 선택 후 제곱하세요
    /* TODO: 홀수만 filter */
    /* TODO: 제곱으로 map */
    let nums = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = nums.iter()
        .filter(|x : &&i32| -> bool { **x % 2 == 1})
        .map(|x : &i32| -> i32 { *x * *x})
        .collect();
    println!("문제 3 결과: {:?}", result); // [1, 9, 25]

    // 문제 4: sum()을 사용하여 벡터의 합계를 구하세요
    let scores = vec![85, 90, 78, 92];
    /* TODO: iter()와 sum() 사용 */
    let total: i32 = scores.iter().sum();
    println!("문제 4 결과: 합계 = {}", total); // 345

    // 문제 5: for 루프와 iter()를 사용하여 요소를 출력하세요
    let fruits = vec!["사과", "바나나", "오렌지"];
    println!("문제 5 결과:");
    /* TODO: fruit 변수 */
    /* TODO: fruits의 이터레이터 */ 
    for fruit in fruits.iter() {
        println!("  {}", fruit);
    }

    // 문제 6: enumerate()를 사용하여 인덱스와 값을 함께 출력하세요
    let colors = vec!["빨강", "초록", "파랑"];
    println!("문제 6 결과:");
    /* TODO: 인덱스 */
    /* TODO: 값 */
    /* TODO: enumerate 사용 */
    for (idx, color) in colors.iter().enumerate() {
        println!("  {}: {}", idx, color);
    }
}
