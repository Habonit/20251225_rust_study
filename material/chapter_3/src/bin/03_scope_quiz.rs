//! 스코프 (Scope) - 변수의 유효 범위 (퀴즈)
//!
//! 아래 코드의 빈칸(/* TODO */)을 채워서 프로그램을 완성하세요.

/// 함수 내 지역 변수 예시
fn function_scope() {
    let x = 10;
    println!("function_scope 내부: x = {}", x);
}
// x는 function_scope 함수 밖에서 접근 불가

/// 인사하는 함수 (String 파라미터)
fn hello(name: String) {
    /* TODO: format! 매크로로 "Hello {name}" 생성 */
    let greeting = format!("Hello {}", name);
    println!("{}", greeting);
}

/// 메인 함수 - 프로그램의 진입점
fn main() {
    // 1. 기본 스코프
    println!("=== 기본 스코프 ===");
    let outer = 100;

    {
        // 내부 스코프
        let inner = 200;
        // 힌트: 내부 블록에서는 outer와 inner 모두 접근 가능
        /* TODO */
        println!("내부 블록: outer = {}, inner = {}", outer, inner);
    }

    // inner는 여기서 접근 불가
    println!("외부 블록: outer = {}", outer);
    // println!("{}", inner);  // 컴파일 에러!

    // 2. 스코프와 섀도잉
    println!("\n=== 스코프와 섀도잉 ===");
    let x = 10;
    println!("외부: x = {}", x);

    {
        // 힌트: 같은 이름으로 새 변수를 선언하면 섀도잉됩니다
        /* TODO: 내부 스코프에서 x를 20으로 섀도잉 */
        let x = 20;
        println!("내부 (섀도잉): x = {}", x);
    }

    println!("외부 (복원): x = {}", x);  // 원래 값 유지

    // 3. 중첩 스코프
    println!("\n=== 중첩 스코프 ===");
    let a = 1;
    {
        let b = 2;
        {
            let c = 3;
            println!("가장 안쪽: a={}, b={}, c={}", a, b, c);
        }
        println!("중간: a={}, b={}", a, b);
        // println!("{}", c);  // 에러! c는 접근 불가
    }
    println!("가장 바깥: a={}", a);
    // println!("{}", b);  // 에러! b는 접근 불가

    // 4. 함수 스코프
    println!("\n=== 함수 스코프 ===");
    /* TODO: function_scope 함수 호출 */
    function_scope();
    // println!("{}", x);  // function_scope의 x는 접근 불가

    // 5. 파이썬과의 차이점 예시
    println!("\n=== 파이썬과의 차이 ===");
    // 힌트: to_string() 메서드로 String 타입 생성
    /* TODO: "buzzi" 문자열을 String으로 변환 */
    let my_name = "buzzi".to_string();

    {
        // 파이썬에서는 if 블록 내부 변수도 외부에서 접근 가능
        // Rust에서는 블록 내부 변수는 블록 밖에서 접근 불가
        println!("내부 블록: my_name = {}", my_name);
        let inner_name = "mellon";
        println!("내부 블록: inner_name = {}", inner_name);
    }

    // inner_name은 여기서 접근 불가
    hello(my_name);  // my_name은 여전히 접근 가능

    // 6. 블록 표현식
    println!("\n=== 블록 표현식 ===");
    // 힌트: 블록의 마지막 표현식(세미콜론 없이)이 블록의 값이 됩니다
    let result = {
        let x = 10;
        let y = 20;
        /* TODO: x + y를 블록의 결과값으로 (세미콜론 없이) */
        x + y
    };
    println!("블록 표현식 결과: {}", result);
}
