// TODO: Change the line below to fix the compiler error.
// const는 컴파일 타임에 평가되므로 타입을 명확히 알아야 함

// let은 타입 추론 가능하지만, const는 항상 독립적이라 추론 어려움

// Rust는 명확성과 안전성을 중시하므로 명시 요구

const NUMBER: i32 = 3;

fn main() {
    println!("Number: {NUMBER}");
}
