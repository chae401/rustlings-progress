# Functions

Here, you'll learn how to write functions and how the Rust compiler can help you debug errors even
in more complex code.

## Further information

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

---

## Rust 함수

### 1. 함수 정의 및 기본 문법

Rust에서 함수는 다음과 같이 정의된다:

```
fn 함수이름(매개변수: 타입, ...) -> 반환타입 {
    // 실행할 코드
}

```

### 2. 파라미터는 반드시 타입 명시

- Rust는 정적 타입 시스템을 가지고 있어 모든 파라미터에 타입을 명시해야 한다.
  - 정적 타입 시스템: 타입이 고정되어 있고, 컴파일 타임에 타입이 결정되고 검증되는 시스템
- 타입 생략 시 컴파일 에러 발생

### 3. 반환값

- `-> 반환타입` 구문을 통해 명시한다.
- return 키워드는 optional
- 마지막 줄에 `;` 없 ㅇ이 표현식만 쓰면 자동 반환

| 구문                 | 의미                                      |
| -------------------- | ----------------------------------------- |
| `return 5;`          | 명시적 반환                               |
| `5` (세미콜론 없음)  | 암시적 반환 (선호됨)                      |
| `5;` (세미콜론 있음) | 세미콜론이 붙으면 단순 실행문 → `()` 반환 |

### 4. Statement vs Expressions

- Rust는 표현식 기반 언어이다. 즉, 값을 반환하는 코드 블록을 자유롭게 사용할 수 있다.

- Statement
  - 어떤 동작을 수행하지만 값을 반환하지 않음.
  - e.g. 변수 선언 `let x = 5;`
- Expression
  - 어떤 값을 계산해서 반환
  - e.g. `5+6`, `x+1`, 함수 호출 등
  - `if`, `match`, 블록 `{}` 도 모두 표현식이 될 수 있다.

### 5. 블록 `{}` 도 값이다.

```
let y = {
    let x = 3;
    x + 1 // 마지막 줄이 세미콜론 없이 표현식이면 이 블록은 `4`를 반환
};
println!("y: {y}"); // y: 4

```

### 6. 함수에서 디버깅 포인트

- 세미콜론(;) 주의: 반환하려는 줄에 붙이면 안된다.
- 타입 명시 -> 컴파일러가 지적해줄 예정
- 변수 이름 실수 -> 컴파일 에러 발생 시 해당 라인 알려줌.
  => Rust는 컴파일 타임에 많은 실수를 잡아줌으로써 런티임 오류를 줄이는 철학을 가지고 있다.

### 7. 함수를 1급 객체인가?

- Rust의 함수는 1급 객체(first-class citizen)이다.
  - 변수에 담을 수 있다.
  - 다른 함수의 인자로 전달 가능하다.
  - 반환값으로 사용할 수 있다.

```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply(f: fn(i32) -> i32, val: i32) -> i32 {
    f(val)
}

fn main() {
    let result = apply(add_one, 5);
    println!("Result: {result}"); // 6
}

```

## 정리

| 항목             | 핵심 내용                                  |
| ---------------- | ------------------------------------------ |
| 함수 정의        | `fn` 키워드 + 명확한 타입                  |
| 반환 방식        | 표현식 사용 → `return` 없이도 반환 가능    |
| 표현식 기반 언어 | `{}` 블록, `if`, `match` 등도 모두 값      |
| 세미콜론         | 세미콜론 붙이면 문, 없으면 표현식          |
| 함수는 1급 객체  | 변수로 저장, 인자로 전달 가능              |
| Rust 철학        | 타입 명시 + 의도 표현 + 컴파일 타임 안정성 |
