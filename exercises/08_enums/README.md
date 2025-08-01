# Enums

Rust allows you to define types called "enums" which enumerate possible values.
Enums are a feature in many languages, but their capabilities differ in each language. Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
Useful in combination with enums is Rust's "pattern matching" facility, which makes it easy to run different code for different values of an enumeration.

## Further information

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Pattern syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

---

# Enums

Rust의 **열거형(Enums)**은 가능한 값들을 나열하여 정의할 수 있는 사용자 정의 타입이다. 많은 프로그래밍 언어에서 열거형 기능을 제공하지만, Rust의 열거형은 F\#, OCaml, Haskell과 같은 함수형 언어의 \*\*대수적 데이터 타입(Algebraic Data Types)\*\*과 가장 유사하다.

열거형과 함께 사용하면 매우 유용한 기능이 바로 Rust의 \*\*패턴 매칭(Pattern Matching)\*\*이다. 패턴 매칭은 열거형의 각 값에 따라 다른 코드를 쉽게 실행할 수 있도록 돕는 강력한 도구다.

---

## Enum의 종류와 기본 문법

### 1\. 기본 열거형 정의 및 사용

열거형은 `enum` 키워드를 사용하여 정의하며, 그 안에 가능한 \*\*배리언트(variants)\*\*들을 나열한다. 각 배리언트는 열거형 타입의 가능한 한 형태를 나타낸다.

#### 1.1. 정의하기

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

#### 1.2. 인스턴스 생성 및 사용하기

각 배리언트는 열거형의 인스턴스가 된다. `::` 연산자를 사용하여 특정 배리언트에 접근한다.

```rust
fn main() {
    let four = IpAddrKind::V4; // V4 배리언트의 인스턴스
    let six = IpAddrKind::V6;  // V6 배리언트의 인스턴스

    route(IpAddrKind::V4); // 함수에 전달
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // ...
}
```

---

### 2\. 열거형에 값 연결하기 (Associated Data)

Rust의 열거형은 단순히 이름만 나열하는 것이 아니라, 각 배리언트에 특정 데이터를 직접 연결할 수 있다. 이는 각 배리언트가 자신만의 고유한 타입을 가진 구조체나 튜플처럼 동작할 수 있게 한다.

#### 2.1. 정의하기

각 배리언트 뒤에 괄호를 사용하여 연결할 데이터의 타입을 정의한다.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8), // 튜플처럼 네 개의 u8 값을 가진다.
    V6(String),         // String 값을 가진다.
}
```

#### 2.2. 인스턴스 생성 및 접근하기

각 배리언트에 맞는 데이터를 제공하여 인스턴스를 생성한다. 연결된 데이터에 직접 접근하려면 **패턴 매칭**을 사용한다.

```rust
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1); // V4 인스턴스
    let loopback = IpAddr::V6(String::from("::1")); // V6 인스턴스
}
```

---

### 3\. `Option` 열거형 (표준 라이브러리)

Rust는 `null` 개념을 직접 사용하지 않고, 부재(absence)를 나타내기 위해 표준 라이브러리의 `Option<T>` 열거형을 사용한다. 이는 런타임의 \*\*널 포인터 역참조(null pointer dereference)\*\*와 같은 오류를 컴파일 타임에 방지한다.

```rust
enum Option<T> { // <T>는 제네릭 타입 파라미터이다.
    None,    // 값이 없음을 나타낸다.
    Some(T), // T 타입의 값이 존재함을 나타낸다.
}
```

#### 3.1. `Option` 사용하기

`Option<T>`는 값이 있을 수도 있고 없을 수도 있는 상황에서 사용된다.

```rust
fn main() {
    let some_number = Some(5);    // Some(5)는 Option<i32> 타입이다.
    let some_char = Some('e');    // Some('e')는 Option<char> 타입이다.

    let absent_number: Option<i32> = None; // None은 Option<i32> 타입이다.
}
```

`Some` 값은 `T` 타입의 실제 값을 가지고 있지만, `Option<T>`와 `T`는 다른 타입이므로 `Option<T>`를 직접 `T`처럼 사용할 수 없다. `match` 문 등을 통해 `Option` 내부의 값에 접근해야 한다.

---

## 패턴 매칭(Pattern Matching)과 `match` 문

`match` 문은 열거형 값에 따라 다른 코드를 실행하는 강력한 제어 흐름 구조다. 각 `arm` (패턴)은 특정 열거형 배리언트와 일치하는지 확인하고, 일치하면 해당 코드를 실행한다.

### 1\. `match` 문 사용하기

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter는 UsState를 가진다.
}

#[derive(Debug)] // 디버깅 출력을 위해 필요하다.
enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // coin 값과 패턴을 매칭한다.
        Coin::Penny => { // Penny 배리언트와 일치하면 이 코드를 실행한다.
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // Quarter 배리언트와 일치하면, state 변수에 연결된 데이터를 바인딩한다.
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska)); // 출력: State quarter from Alaska!
    value_in_cents(Coin::Penny); // 출력: Lucky penny!
}
```

### 2\. `match`의 필수 조건: 모든 가능한 패턴 커버하기

`match` 문은 주어진 타입의 \*\*모든 가능한 값(all possible patterns)\*\*을 처리해야 한다. 만약 모든 경우를 다루지 않으면 컴파일 오류가 발생한다. 이는 Rust가 예상치 못한 상황으로 인한 런타임 오류를 방지하는 또 다른 방법이다.

- 모든 배리언트를 명시적으로 나열하거나, `_` (와일드카드 패턴)을 사용하여 나머지 모든 경우를 처리할 수 있다.

<!-- end list -->

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // None인 경우를 처리한다.
        Some(i) => Some(i + 1), // Some(i)인 경우를 처리하고 i를 사용한다.
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five); // six는 Some(6)이 된다.
    let none = plus_one(None); // none은 None이 된다.
}
```

---

## Rust Enum의 언어 철학

Rust의 열거형은 다음 언어 철학을 강력하게 반영한다:

1.  **안전성과 견고성(Safety and Robustness):**

    - **부재(absence)의 명시적 처리:** `Option<T>`를 통해 `null` 참조와 같은 문제를 컴파일 타임에 해결한다. 개발자는 값이 없을 가능성을 명시적으로 처리하도록 강제되어, 런타임에 널 포인터 역참조로 인한 패닉이나 충돌을 예방한다.
    - **완전한 패턴 매칭:** `match` 문이 모든 가능한 케이스를 처리하도록 강제함으로써, 개발자가 특정 상황을 놓치지 않도록 돕는다. 이는 프로그램의 논리적 오류를 줄이고 예측 가능성을 높인다.

2.  **명시성과 표현력(Explicitness and Expressiveness):**

    - 열거형은 특정 값이 가질 수 있는 모든 가능한 형태를 명확하게 정의한다. 각 배리언트에 데이터를 연결함으로써, 타입 자체가 더 많은 의미를 전달하고 코드의 의도를 명확하게 표현할 수 있다.
    - 예를 들어, 단순히 `bool`이 아니라 `enum DoorState { Open, Closed }`를 사용하면 코드의 가독성이 훨씬 높아진다.

3.  **"제로 코스트 추상화"**:

    - 열거형과 패턴 매칭은 강력한 추상화를 제공하지만, 대부분의 경우 런타임 오버헤드가 거의 없다. 컴파일러는 이들을 효율적인 기계어 코드로 변환하여, 개발자가 수동으로 `if/else` 체인을 작성하는 것과 유사한 성능을 낸다.
    - 연결된 데이터가 있는 열거형은 내부적으로 가장 큰 배리언트에 맞춰 메모리 공간을 할당하고, 어떤 배리언트인지 식별하는 작은 태그(tag)를 추가하는 방식으로 최적화된다.

4.  **강력한 타입 시스템:**

    - 열거형은 Rust의 강력한 타입 시스템의 중요한 구성 요소다. 이를 통해 컴파일 타임에 더 많은 오류를 잡아내고, 런타임 오류의 위험을 줄일 수 있다.

결론적으로 Rust의 열거형은 안전성, 표현력, 그리고 성능이라는 Rust의 핵심 가치를 모두 구현하는 다재다능하고 강력한 기능이다.

---
