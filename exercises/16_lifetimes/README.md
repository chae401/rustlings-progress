# Lifetimes

Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid".

They are only necessary on borrows, i.e. references,
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are
restrictive of their callers.

If you'd like to learn more about lifetime annotations, the
[lifetimekata](https://tfpk.github.io/lifetimekata/) project
has a similar style of exercises to Rustlings, but is all about
learning to write lifetime annotations.

## Further information

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

---

# Lifetimes

\*\*생명 주기(Lifetimes)\*\*는 Rust 컴파일러에게 참조(reference)가 주어진 상황에서 유효하게 존재할 만큼 충분히 오래 살아 있는지 확인하는 방법을 알려준다. 예를 들어, 생명 주기는 "매개변수 'a'가 매개변수 'b'만큼 오래 살아 있는지 확인하여 반환 값이 유효하도록 하세요"와 같이 지시한다.

생명 주기는 **빌림(borrow)**, 즉 참조에만 필요하다. 복사된 매개변수나 이동된 값은 각자의 스코프(scope) 내에서 소유되기 때문에 외부에서 참조될 수 없다. 생명 주기는 함수와 같은 호출 코드에서 인자들이 유효한지 확인할 수 있도록 한다. 따라서 생명 주기는 호출하는 코드에 제약을 가한다.

생명 주기 어노테이션에 대해 더 자세히 알고 싶다면, Rustlings와 유사한 스타일의 연습 문제를 제공하는 [lifetimekata](https://tfpk.github.io/lifetimekata/) 프로젝트를 참고하면 된다. 이 프로젝트는 생명 주기 어노테이션 작성법을 배우는 데 특화되어 있다.

---

## Rust 생명 주기의 기본 개념과 문법

Rust의 생명 주기는 `&` 기호 뒤에 작은따옴표(`'`)로 시작하는 이름을 붙여 명시한다. `'a`, `'b` 등이 일반적인 생명 주기 이름이다. 이들은 참조가 유효할 것으로 예상되는 기간을 나타낸다.

### 1\. 생명 주기 문제의 발생 원인

함수가 참조를 인자로 받고 참조를 반환할 때, Rust 컴파일러는 반환되는 참조가 인자로 받은 참조들 중 어느 것에서 왔는지 알 수 없을 수 있다. 이때 반환된 참조가 유효하지 않은(dangling) 데이터를 가리킬 위험이 생긴다. 생명 주기는 이러한 '댕글링 참조'를 방지한다.

예시: 이 코드는 컴파일되지 않는다.

```rust
// fn longest(x: &str, y: &str) -> &str { // 에러: 반환 참조의 생명 주기를 알 수 없다.
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
```

위 함수는 `x`와 `y` 중 더 긴 문자열 슬라이스를 반환하는데, Rust는 반환되는 `&str`의 생명 주기가 `x`의 생명 주기와 같을지 `y`의 생명 주기와 같을지 알지 못한다. 즉, 함수를 호출한 쪽에서 `x`나 `y`가 먼저 사라져 버리면 반환된 참조가 유효하지 않은 데이터를 가리키게 될 수 있다.

### 2\. 생명 주기 어노테이션 문법

생명 주기 어노테이션은 제네릭 문법과 유사하게 꺾쇠 괄호(`<`) 안에 지정한다. 함수 시그니처에서 각 참조 파라미터와 반환 참조에 생명 주기 어노테이션을 붙여 관계를 명시한다.

```rust
// 반환되는 참조가 인자 x와 y 중 더 짧은 생명 주기('a)만큼 유효함을 나타낸다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2); // string1.as_str()은 &str을 반환한다.
    println!("The longest string is {}", result); // 출력: The longest string is abcd

    // 다른 예시: 생명 주기가 겹치지 않아 컴파일 에러 발생 (주석 처리하면 컴파일됨)
    // let string1 = String::from("long string is long");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str()); // string2의 생명 주기가 이 블록 내에서 끝난다.
    //     // println!("The longest string is {}", result); // 이 시점에서 result는 유효하지 않은 string2를 가리킬 수 있어 에러
    // }
}
```

위 `longest` 함수에서 `<'a>`는 모든 참조에 동일한 생명 주기 `'a`를 부여한다. 이는 반환되는 참조가 입력 참조 `x`와 `y` 중 **가장 짧은 생명 주기**만큼만 유효함을 의미한다. 이렇게 명시함으로써 Rust 컴파일러는 참조의 유효성을 검사할 수 있게 된다.

### 3\. 구조체에서 생명 주기 사용하기

구조체가 참조를 포함할 때, 해당 참조가 구조체 인스턴스보다 오래 살아 있음을 보장하기 위해 생명 주기 어노테이션을 사용한다.

```rust
struct ImportantExcerpt<'a> { // ImportantExcerpt 인스턴스가 존재하는 동안 참조 'a'가 유효해야 한다.
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence, // first_sentence는 novel에서 빌려온 것이므로, novel이 살아 있는 한 i도 유효하다.
    };

    println!("Important part: {}", i.part); // 출력: Important part: Call me Ishmael
}
```

### 4\. 생명 주기 생략 규칙 (Lifetime Elision Rules)

Rust 컴파일러는 특정 상황에서 생명 주기 어노테이션을 자동으로 추론한다. 이를 **생명 주기 생략 규칙**이라고 부른다. 모든 참조에 생명 주기를 명시할 필요는 없으며, 대부분의 간단한 경우에는 Rust가 자동으로 처리한다.

- 입력 생명 주기 규칙:
  1.  각 입력 참조 매개변수는 고유한 생명 주기 파라미터를 얻는다.
  2.  입력 생명 주기 파라미터가 하나만 있을 경우, 그 생명 주기는 모든 출력 생명 주기 파라미터에 할당된다.
  3.  여러 입력 생명 주기 파라미터가 있지만, 하나가 `&self` 또는 `&mut self`인 메소드의 경우, `self`의 생명 주기는 모든 출력 생명 주기 파라미터에 할당된다.

이러한 규칙은 개발자가 코드를 더 간결하게 작성할 수 있도록 돕는다.

---

## Rust 생명 주기의 언어 철학

Rust의 생명 주기 시스템은 언어의 핵심 철학인 \*\*메모리 안전성(Memory Safety)\*\*과 \*\*성능(Performance)\*\*을 동시에 달성하기 위한 근본적인 기반이다.

1.  **메모리 안전성 보장 (댕글링 참조 방지):**

    - 생명 주기는 컴파일 타임에 참조가 가리키는 데이터가 여전히 유효한지 확인한다. 이는 C/C++에서 흔히 발생하는 **댕글링 포인터(dangling pointer)** 문제(해제된 메모리를 가리키는 참조)를 Rust에서는 원천적으로 불가능하게 만든다.
    - 런타임에 메모리 오류로 인해 프로그램이 충돌하거나 예측할 수 없는 동작을 하는 것을 방지한다.

2.  **가비지 컬렉터(Garbage Collector) 없는 성능:**

    - 다른 많은 메모리 안전 언어가 가비지 컬렉터를 사용하여 런타임에 메모리를 관리하는 반면, Rust는 생명 주기 시스템과 소유권(Ownership)을 통해 컴파일 타임에 메모리 할당 및 해제 시점을 결정한다.
    - 이는 런타임 오버헤드가 없는 "제로 코스트 추상화"를 가능하게 하여, 가비지 컬렉터가 없는 C/C++과 같은 언어에 필적하는 고성능을 달성할 수 있도록 한다.

3.  **명시적인 계약(Contract)과 제어:**

    - 생명 주기 어노테이션은 개발자에게 참조의 유효 기간에 대한 명시적인 계약을 요구한다. 이는 코드를 읽는 다른 개발자가 참조의 관계와 데이터의 수명을 명확하게 이해하는 데 도움을 준다.
    - 컴파일러가 생명 주기 제약 조건을 엄격하게 시행하므로, 개발자는 메모리 안전에 대한 확신을 가지고 저수준 코드를 작성할 수 있다.

4.  **"Borrow Checker"의 역할:**

    - 생명 주기는 Rust의 \*\*빌림 검사기(Borrow Checker)\*\*가 동작하는 핵심적인 정보이다. 빌림 검사기는 모든 참조의 생명 주기를 추적하여 빌림 규칙(예: 하나의 가변 참조 또는 여러 개의 불변 참조)이 위반되지 않고, 모든 참조가 유효한 데이터를 가리키는지 확인한다.

결론적으로 Rust의 생명 주기 시스템은 학습 곡선이 가파를 수 있지만, 메모리 관련 버그를 컴파일 타임에 제거하고 런타임 성능 저하 없이 안전한 코드를 작성할 수 있도록 하는 Rust의 가장 강력하고 근본적인 기능 중 하나다.

---
