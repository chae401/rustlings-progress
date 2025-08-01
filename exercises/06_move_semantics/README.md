# Move Semantics

These exercises are adapted from [pnkfelix](https://github.com/pnkfelix)'s [Rust Tutorial](https://pnkfelix.github.io/rust-examples-icfp2014/) -- Thank you Felix!!!

## Further information

For this section, the book links are especially important.

- [Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Reference and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

# Move Semantics

Rust의 **Move Semantics**는 언어의 핵심인 **소유권(Ownership)** 시스템과 밀접하게 관련되어 있다. 다른 프로그래밍 언어에서는 변수에 값을 할당하거나 함수에 인자를 전달할 때 대부분 값이 복사(copy)되거나 참조(reference)된다. 하지만 Rust에서는 이러한 상황에서 기본적으로 \*\*이동(move)\*\*이 발생한다. 이동은 값의 소유권이 한 변수에서 다른 변수로 넘어가는 것을 의미하며, 이는 메모리 안전성(Memory Safety)을 보장하는 Rust의 주요 메커니즘 중 하나다.

---

## Move Semantics의 기본 개념과 동작

### 1\. 소유권(Ownership)

Rust의 모든 값은 \*\*소유자(owner)\*\*라고 불리는 변수를 가진다. 어떤 시점에서든 값의 소유자는 하나뿐이다. 소유자가 스코프(scope)를 벗어나면, 값은 드롭(drop)되어 메모리에서 해제된다.

```rust
fn main() {
    let s1 = String::from("hello"); // s1이 "hello"의 소유자가 된다.
    // s1이 스코프 끝에 도달하면 드롭되어 메모리가 해제된다.
}
```

### 2\. 이동(Move)

Rust에서 `String`과 같이 힙(heap)에 할당되는 데이터 타입(즉, `Copy` 트레이트가 구현되지 않은 타입)의 경우, 변수 간의 할당이나 함수로의 전달은 **소유권의 이동**을 발생시킨다.

```rust
fn main() {
    let s1 = String::from("hello"); // s1이 "hello"의 소유자가 된다.
    let s2 = s1;                     // 소유권이 s1에서 s2로 이동한다.
                                     // 이제 s1은 더 이상 유효하지 않다. (s1은 "moved")

    // println!("{}", s1); // 에러 발생! s1의 소유권이 이동했기 때문에 사용할 수 없다.
    println!("{}", s2);   // s2는 유효하다. 출력: hello
} // s2가 스코프를 벗어나면서 "hello"의 메모리가 해제된다.
```

함수에 값을 전달할 때도 동일한 원리가 적용된다.

```rust
fn takes_ownership(some_string: String) { // some_string이 소유권을 받는다.
    println!("{}", some_string);
} // some_string이 스코프를 벗어나면서 드롭된다.

fn main() {
    let s = String::from("world"); // s가 "world"의 소유자다.
    takes_ownership(s);             // s의 소유권이 takes_ownership 함수로 이동한다.

    // println!("{}", s); // 에러 발생! s는 더 이상 유효하지 않다.
}
```

### 3\. 복사(Copy)

정수, 부동소수점, 불리언, 고정 크기 배열 등과 같이 **스택(stack)에 저장되고 크기가 알려진 간단한 데이터 타입**들은 `Copy` 트레이트가 구현되어 있어 이동 대신 **복사**가 발생한다. 즉, 소유권이 이동하지 않고 값이 단순히 복제된다.

```rust
fn main() {
    let x = 5; // x가 5의 소유자다.
    let y = x; // x는 Copy 트레이트가 구현되어 있으므로, y는 x의 복사본을 받는다.
               // x는 여전히 유효하다.

    println!("x: {}, y: {}", x, y); // 출력: x: 5, y: 5
}
```

함수에 `Copy` 타입의 값을 전달할 때도 복사가 발생한다.

```rust
fn makes_copy(some_integer: i32) { // some_integer가 5의 복사본을 받는다.
    println!("{}", some_integer);
} // some_integer는 스코프를 벗어나 드롭된다.

fn main() {
    let x = 5; // x가 5의 소유자다.
    makes_copy(x); // x는 Copy 트레이트가 구현되어 있으므로, x의 복사본이 함수로 전달된다.
                   // x는 여전히 유효하다.

    println!("{}", x); // 출력: 5
}
```

### 4\. 참조와 빌림(References and Borrowing)

값의 소유권을 이동시키지 않으면서 값을 사용하고 싶을 때 \*\*참조(reference)\*\*를 사용한다. 참조를 사용하는 것을 \*\*빌림(borrowing)\*\*이라고 부른다.

- **불변 참조 (`&T`):** 값을 읽기만 할 수 있으며, 여러 개의 불변 참조를 동시에 가질 수 있다.
- **가변 참조 (`&mut T`):** 값을 변경할 수 있으며, 오직 하나의 가변 참조만 특정 시점에 존재할 수 있다.

<!-- end list -->

```rust
fn calculate_length(s: &String) -> usize { // s는 String의 불변 참조를 빌린다.
    s.len()
} // s는 스코프를 벗어나지만, 소유권을 가지고 있지 않으므로 아무것도 드롭하지 않는다.

fn change_string(s: &mut String) { // s는 String의 가변 참조를 빌린다.
    s.push_str(", world");
} // s는 스코프를 벗어나지만, 소유권을 가지고 있지 않으므로 아무것도 드롭하지 않는다.

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1의 불변 참조를 전달한다. s1은 여전히 유효하다.
    println!("The length of '{}' is {}.", s1, len); // 출력: The length of 'hello' is 5.

    let mut s2 = String::from("hello");
    change_string(&mut s2); // s2의 가변 참조를 전달한다. s2는 여전히 유효하다.
    println!("{}", s2); // 출력: hello, world
}
```

참조 규칙은 컴파일 타임에 Rust 컴파일러에 의해 엄격하게 검사되어 \*\*데이터 경쟁(data race)\*\*을 방지한다.

---

## Move Semantics를 통한 Rust의 언어 철학

Rust의 Move Semantics는 다음 세 가지 핵심 철학을 구현하는 데 중요한 역할을 한다.

### 1\. **메모리 안전성(Memory Safety) 보장**

- **이중 해제(Double Free) 방지:** 소유권이 이동하면 이전 변수는 더 이상 유효하지 않으므로, 하나의 메모리 위치가 두 번 해제되는 것을 컴파일 타임에 원천적으로 방지한다. 이는 C/C++에서 흔히 발생하는 심각한 보안 취약점이다.
- **댕글링 포인터(Dangling Pointer) 방지:** 소유자가 스코프를 벗어나 메모리를 해제하면, 해당 메모리를 가리키던 다른 참조가 존재하지 않도록 컴파일러가 강제한다. 즉, 유효하지 않은 메모리 주소를 가리키는 포인터가 생기는 것을 막는다.
- **런타임 오류 최소화:** `get()` 메서드가 `Option`을 반환하는 것과 같이, Rust는 가능한 많은 오류를 컴파일 타임에 잡아내려 한다. Move Semantics는 이 노력의 핵심으로, 메모리 관련 런타임 오류를 줄여 프로그램의 안정성을 극대화한다.

### 2\. **데이터 경쟁(Data Race) 방지 및 안전한 동시성(Concurrency)**

- Rust의 빌림 규칙(참조 규칙)은 특정 시점에 가변 참조는 하나만 존재할 수 있고, 불변 참조는 여러 개가 존재할 수 있지만 가변 참조와 동시에 존재할 수는 없다고 명시한다.
- 이러한 규칙은 컴파일 타임에 **데이터 경쟁**을 효과적으로 방지한다. 데이터 경쟁은 두 개 이상의 포인터가 동시에 같은 데이터에 접근하고, 그 중 하나라도 쓰기 작업을 하며, 데이터에 접근하는 작업의 순서가 중요할 때 발생한다. 이는 동시성 프로그래밍에서 가장 흔하고 디버깅하기 어려운 버그 중 하나다. Move Semantics와 빌림 규칙은 이러한 위험을 제거하여 **안전한 동시성 프로그래밍**을 가능하게 한다.

### 3\. **명시적인 자원 관리 및 제어**

- Move Semantics는 데이터의 소유권과 생명 주기를 매우 명시적으로 만든다. 개발자는 어떤 변수가 언제 자원의 소유권을 가지는지, 그리고 언제 자원이 해제될지 정확히 알 수 있다.
- 이는 **"제로 코스트 추상화(Zero-Cost Abstraction)"** 철학의 일환이다. 개발자가 수동으로 메모리를 관리하는 C/C++과 비슷한 수준의 제어권을 가지면서도, 소유권 시스템 덕분에 자동적인 메모리 안전성 보장을 받을 수 있다. 런타임 가비지 컬렉터(Garbage Collector)의 오버헤드 없이 메모리를 효율적으로 관리할 수 있게 한다.

---

결론적으로 Rust의 Move Semantics는 소유권, 빌림, 생명 주기 규칙과 함께 Rust가 **안전하고 빠르며 동시성 프로그래밍이 용이한 언어**가 되도록 하는 근본적인 설계 철학을 반영하고 있다.
