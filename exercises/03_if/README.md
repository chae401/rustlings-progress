# If

`if`, the most basic (but still surprisingly versatile!) type of control flow, is what you'll learn here.

## Further information

- [Control Flow - if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)

---

## Rust If

Rust는 표현식 기반 언어(expression-oriented language) 이기 때문에 `if` 도 표현식(expression)이고 값을 반환할 수 있다.

## 기본 사용법

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("조건은 참입니다");
    } else {
        println!("조건은 거짓입니다");
    }
}

```

- Rust는 `0` 이나 `null`, 빈 문자열을 false로 간주하지 않고 반드시 `true` 나 `false` 여야 한다.
- 모든 분기의 반환 타입은 동일해야 한다.
- 조건문은 들여쓰기 대신 중괄호 `{}` 를 필수로 사용한다.

## 여러 조건 분기

```rust
let number = 6;

if number % 4 == 0 {
    println!("4의 배수입니다");
} else if number % 3 == 0 {
    println!("3의 배수입니다");
} else if number % 2 == 0 {
    println!("2의 배수입니다");
} else {
    println!("어떤 배수도 아닙니다");
}

```

## 표현식

- Rust에서 if는 표현식이기 때문에 값을 반환할 수 있다.
- `if`와 `else` 블록의 마지막 표현식의 타입이 동일해야 한다.

```
let condition = true;
let number = if condition {
    5
} else {
    6
};

println!("The number is: {number}");

```

```
let condition = true;

let number = if condition {
    5
} else {
    "six" // ❌ 에러: expected integer, found &str
};

```

## 과거 언어들과의 차별점

- if 를 statement가 아닌 expression으로 사용
  - JS, Python의 삼항 연산자
- 타입 체크가 엄격 - 조건식은 bool만 가능
