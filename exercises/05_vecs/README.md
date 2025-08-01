# Vectors

Vectors are one of the most-used Rust data structures. In other programming
languages, they'd simply be called Arrays, but since Rust operates on a
bit of a lower level, an array in Rust is stored on the stack (meaning it
can't grow or shrink, and the size needs to be known at compile time),
and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are a bit of a later chapter in the book, but we think that they're
useful enough to talk about them a bit earlier. We shall be talking about
the other useful data structure, hash maps, later.

## Further information

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

---

## Vector

**Vector**는 Rust에서 동적으로 크기를 조절할 수 있는 컬렉션 타입으로, 힙(heap)에 데이터를 저장한다. 다른 언어의 `ArrayList`나 `Vec`과 유사하다고 생각하면 된다. 동일한 타입의 값들을 나열하여 저장하며, 런타임에 크기가 커지거나 줄어들 수 있다.

---

### 1\. Vector 생성하기

새로운 빈 Vector를 생성하려면 `Vec::new()` 함수를 사용한다. 이때 Vector에 어떤 타입의 요소가 들어갈지 명시해야 한다.

```rust
let v: Vec<i32> = Vec::new(); // i32 타입의 빈 Vector를 생성한다.
```

초기 값과 함께 Vector를 생성하려면 `vec!` 매크로를 사용하는 것이 더 편리하다. 이때는 타입 추론이 가능해 타입을 명시할 필요가 없다.

```rust
let v = vec![1, 2, 3]; // [1, 2, 3]을 포함하는 Vector를 생성한다.
```

---

### 2\. 값 추가하기 (`push`)

Vector의 끝에 새로운 값을 추가하려면 `push()` 메소드를 사용한다. Vector는 가변(mutable)이어야 한다.

```rust
let mut v = Vec::new(); // 가변 Vector를 생성한다.
v.push(5);
v.push(6);
v.push(7);
println!("{:?}", v); // 출력: [5, 6, 7]
```

---

### 3\. 값 읽기

Vector의 특정 인덱스에 있는 값을 읽는 방법은 두 가지가 있다.

#### 3.1. 인덱싱 (`[]`)

가장 일반적인 방법으로, 배열처럼 `[]`를 사용하여 인덱스로 접근할 수 있다. 하지만 이 방법은 \*\*패닉(panic)\*\*을 일으킬 수 있는 위험이 있다. 유효하지 않은 인덱스에 접근하려 하면 프로그램이 강제 종료된다.

```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2]; // 세 번째 요소(인덱스 2)에 접근한다.
println!("The third element is {}", third); // 출력: The third element is 3

// v[10]처럼 존재하지 않는 인덱스에 접근하면 런타임 패닉이 발생한다!
// let does_not_exist = &v[10];
```

#### 3.2. `get` 메소드

`get()` 메소드는 `Option<&T>`를 반환한다. 값이 존재하면 `Some(value)`를, 존재하지 않으면 `None`을 반환하므로, 안전하게 값을 읽을 수 있다. `match` 문과 함께 사용하여 에러 처리를 할 수 있다.

```rust
let v = vec![1, 2, 3, 4, 5];

match v.get(2) {
    Some(third) => println!("The third element is {}", third), // 출력: The third element is 3
    None => println!("There is no third element."),
}

match v.get(10) {
    Some(tenth) => println!("The tenth element is {}", tenth),
    None => println!("There is no tenth element."), // 출력: There is no tenth element.
}
```

---

### 4\. 값 수정하기

Vector의 특정 인덱스에 있는 값을 변경하려면 해당 인덱스에 접근하여 새로운 값을 할당한다. Vector는 가변이어야 한다.

```rust
let mut v = vec![10, 20, 30];
v[1] = 25; // 두 번째 요소(인덱스 1)를 25로 변경한다.
println!("{:?}", v); // 출력: [10, 25, 30]
```

---

### 5\. 값 삭제하기 (`pop`, `remove`)

- **`pop()`:** Vector의 마지막 요소를 제거하고 `Option<T>`로 반환한다. 비어있으면 `None`을 반환한다.
  ```rust
  let mut v = vec![1, 2, 3];
  let last = v.pop(); // last는 Some(3)이다.
  println!("{:?}", last); // 출력: Some(3)
  println!("{:?}", v);    // 출력: [1, 2]
  ```
- **`remove(index)`:** 특정 인덱스의 요소를 제거하고 그 값을 반환한다. 나머지 요소들은 앞으로 당겨진다.
  ```rust
  let mut v = vec![1, 2, 3];
  let second = v.remove(1); // second는 2이다.
  println!("{:?}", second); // 출력: 2
  println!("{:?}", v);     // 출력: [1, 3]
  ```

---

### 6\. Vector 순회하기

`for` 루프를 사용하여 Vector의 모든 요소를 순회할 수 있다.

#### 6.1. 불변 참조로 순회 (`&v`)

가장 일반적인 방법으로, 요소를 읽기만 할 때 사용한다.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
// 출력:
// 100
// 32
// 57
```

#### 6.2. 가변 참조로 순회 (`&mut v`)

요소를 순회하면서 값을 변경해야 할 때 사용한다. `iter_mut()` 메소드를 통해 가변 참조를 얻을 수 있다.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // 역참조 연산자(*)를 사용하여 값을 변경한다.
}
println!("{:?}", v); // 출력: [150, 82, 107]
```

#### 6.3. 소유권을 넘겨주면서 순회 (`v`)

Vector의 소유권을 `for` 루프에 넘겨주면서 순회한다. 이 방법은 Vector를 더 이상 사용하지 않고 각 요소의 소유권이 필요할 때 사용한다. 루프가 끝나면 Vector는 드롭된다.

```rust
let v = vec![100, 32, 57];
for i in v { // v의 소유권이 for 루프로 이동한다.
    println!("{}", i);
}
// println!("{:?}", v); // 에러 발생! v는 이미 드롭되었다.
```

---

### 7\. Vector에 다른 타입 저장하기 (Enums 활용)

Vector는 한 가지 타입의 값만 저장할 수 있다. 하지만 **Enum**을 활용하면 여러 다른 타입의 값을 Vector에 저장하는 것처럼 보이게 할 수 있다.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("Integer: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
```

---

## Rust가 Vector와 Array를 이렇게 구현한 이유 (언어 철학)

Rust의 핵심 철학은 **안전성(Safety)**, **성능(Performance)**, 그리고 \*\*동시성(Concurrency)\*\*이다. Vector와 Array의 디자인은 이러한 철학을 깊이 반영하고 있다.

---

### 1\. **메모리 관리의 명시성 및 예측 가능성**

- **Array (스택 할당):**
  - **컴파일 타임에 크기 고정:** Rust의 배열은 크기가 고정되어 있으며, 스택에 할당된다. 스택 할당은 매우 빠르고 효율적이다. 메모리 할당 및 해제 오버헤드가 없으며, 캐시 효율도 좋다.
  - **패닉 발생 가능성:** 배열은 크기가 고정되어 있으므로, 존재하지 않는 인덱스에 접근하려 하면 (예: `arr[10]`에 접근하려는데 배열 크기가 5일 때) 컴파일러가 이를 막거나 런타임에 패닉을 일으켜 프로그램이 비정상 종료되도록 한다. 이는 "안전하지 않은" 메모리 접근(예: 다른 프로그램의 메모리 영역 침범)을 방지하고, 프로그래머에게 명확하게 오류를 알리는 방식이다. 즉, \*\*"모든 오류는 명시적으로 처리되거나 패닉으로 실패해야 한다"\*\*는 Rust의 철학을 따른다.
- **Vector (힙 할당):**
  - **런타임에 크기 조절:** Vector는 힙에 할당된다. 힙은 스택보다 유연하여 런타임에 크기를 조절할 수 있다. 이는 동적으로 데이터가 추가되거나 제거될 때 효율적이다.
  - **성능 고려:** 힙 할당은 스택보다 느리지만, `Vec`은 내부적으로 용량(capacity)을 미리 할당하고 필요할 때만 재할당하는 전략(doubling capacity)을 사용하여 재할당에 따른 성능 저하를 최소화한다.
  - **안전한 접근(`get()`):** `Vec`의 `get()` 메소드가 `Option`을 반환하는 것은 인덱스 접근 시 \*\*널 포인터 역참조(null pointer dereference)\*\*와 같은 심각한 런타임 오류를 방지하기 위함이다. 존재하지 않는 인덱스에 접근할 가능성을 컴파일 타임에 `Option` 타입으로 경고하고, 개발자가 이를 명시적으로 처리하도록 강제하여 프로그램의 안정성을 높인다.

---

### 2\. **소유권(Ownership) 시스템과의 조화**

Rust의 Vector와 Array는 소유권, 빌림(borrowing), 생명 주기(lifetimes) 시스템과 완벽하게 통합된다.

- `Vec<T>`는 그 안에 있는 `T` 타입 요소들의 소유권을 가진다. Vector가 스코프 밖으로 나가면, 그 안에 있는 모든 요소들도 함께 해제된다. 이는 **메모리 누수(memory leak)를 방지**한다.
- `&Vec<T>` (불변 참조)와 `&mut Vec<T>` (가변 참조)를 통해 데이터를 안전하게 공유하고 변경할 수 있다. `iter()`와 `iter_mut()` 메소드가 참조를 반환하는 이유도 이 때문이다. 이는 \*\*데이터 레이스(data race)\*\*와 같은 동시성 문제를 컴파일 타임에 방지하는 데 기여한다.

---

### 3\. **성능과 제어의 균형**

Rust는 "제로 코스트 추상화(Zero-Cost Abstraction)"를 지향한다. 즉, 추상화를 사용하더라도 런타임 오버헤드가 거의 없거나, 직접 구현했을 때와 동일한 성능을 내도록 설계되었다.

- Vector는 편리함을 제공하지만, 개발자는 여전히 메모리 할당 방식(힙 vs 스택)과 그에 따른 성능 특성을 이해하고 선택할 수 있다.
- `Vec`은 내부적으로 일반적인 동적 배열처럼 구현되어 있으며, 다른 언어의 고성능 라이브러리에서 사용하는 최적화 기법(예: `realloc`을 통한 효율적인 메모리 재할당)이 적용되어 있다.

---

결론적으로 Rust의 Vector와 Array는 **메모리 안전성, 성능 최적화, 그리고 개발자에게 명시적인 제어권을 부여**하려는 Rust의 깊은 언어 철학을 반영한 결과물이다. 이를 통해 Rust는 메모리 관리에 대한 부담을 줄이면서도 C/C++과 같은 저수준 언어에 버금가는 성능을 달성할 수 있다.

---

## 기타: 패닉(Panic) 상태

"패닉(Panic)" 상태는 Rust에서 중요한 개념이며, 다른 프로그래밍 언어에도 유사한 개념이 존재하지만 Rust에서는 특히 \*\*안전성(Safety)\*\*과 **오류 처리(Error Handling)** 철학의 핵심적인 부분으로 다루어진다.

---

### Rust에서 "패닉"이란?

Rust에서 패닉은 **회복 불가능한(unrecoverable) 오류**를 나타낸다. 즉, 프로그램이 더 이상 진행할 수 없는 심각한 상태에 도달했음을 의미한다. 이러한 상황이 발생하면 Rust 프로그램은 기본적으로 현재 스레드(thread)의 실행을 중단하고, 콜 스택(call stack)을 풀어서(unwinding) 정리한 다음, **프로그램 전체를 종료**한다.

패닉이 발생하는 일반적인 경우:

- **배열/벡터 인덱스 범위를 벗어난 접근:** `let arr = [1, 2, 3]; arr[10];` (런타임 패닉)
- **나눗셈 0:** `1 / 0;` (런타임 패닉)
- **옵션/결과 타입의 `unwrap()` 또는 `expect()` 호출 시 `None` 또는 `Err` 값에 대해:** `None.unwrap();`
- **불변(immutable) 조건이 깨진 경우:** `assert!` 매크로가 실패했을 때.
- **프로그래머가 의도적으로 `panic!` 매크로를 호출했을 때:** 예를 들어, 아직 구현되지 않은 기능에 대한 플레이스홀더로 `panic!("Not yet implemented");`를 사용할 수 있다.

**패닉의 목적:**
Rust는 패닉을 통해 "이 프로그램은 현재 상태에서 더 이상 올바르게 동작할 수 없으므로, 즉시 종료하여 더 큰 문제를 방지해야 한다"는 신호를 보낸다. 이는 메모리 손상이나 보안 취약점과 같은 심각한 문제를 예방하려는 Rust의 안전성 지향 철학의 일환이다.

---

### 다른 언어의 유사 개념

"패닉"이라는 용어 자체는 Rust에서 특정 의미를 가지지만, 다른 프로그래밍 언어에도 이와 유사하게 **회복 불가능한 심각한 오류 상태를 나타내는 개념**들이 존재한다.

1.  **예외(Exception) / Throw & Catch (Java, C++, Python, JavaScript 등):**

    - 대부분의 주류 언어에서 사용되는 가장 일반적인 오류 처리 메커니즘이다.
    - **차이점:** 예외는 기본적으로 **회복 가능성**을 염두에 둔다. `try-catch` 또는 `try-except` 블록을 사용하여 예외를 잡고(catch) 프로그램 흐름을 복구할 수 있다.
    - Rust의 패닉은 예외처럼 `catch`하여 복구하는 것이 일반적이지 않다. Rust는 회복 가능한 오류에는 `Result` 타입을, 회복 불가능한 오류에는 패닉을 사용하도록 설계되었다.
    - 다만, 일부 언어에서는 "처리되지 않은 예외(unhandled exception)"가 발생하면 프로그램이 종료될 수 있는데, 이 경우 Rust의 패닉과 유사한 결과(프로그램 종료)를 가져온다.

2.  **어설션 실패(Assertion Failure) (C, C++):**

    - `assert()`와 같은 매크로/함수는 특정 조건이 참(true)이라고 가정한 후, 거짓(false)일 경우 프로그램을 비정상 종료시킨다.
    - **유사점:** 어설션 실패는 일반적으로 개발 중에 논리적 오류를 잡아내기 위해 사용되며, 런타임에 발생하면 프로그램이 즉시 종료된다는 점에서 Rust의 패닉과 매우 유사하다. 이는 "이런 상황은 발생해서는 안 된다"는 의미로 사용된다.

3.  **Fatal Error / Abort (다양한 언어):**

    - 일부 언어나 시스템에서는 "치명적인 오류(Fatal Error)" 또는 "중단(Abort)"이라는 용어를 사용하여 프로그램이 더 이상 진행할 수 없어서 강제로 종료되는 상황을 나타낸다.
    - **유사점:** Rust의 패닉과 가장 유사한 개념으로 볼 수 있다. 회복할 수 없는 오류가 발생했을 때 시스템 자원을 정리하고 종료하는 방식이다.

---

### Rust의 오류 처리 철학 요약

Rust는 오류 처리에 있어 명확한 구분을 가진다:

- **회복 가능한 오류 (Recoverable Errors):** 사용자 입력 오류, 파일 없음, 네트워크 연결 실패 등과 같이 프로그램을 계속 실행할 수 있지만, 오류를 처리해야 하는 상황이다.
  - **`Result<T, E>` 열거형**을 사용하여 명시적으로 오류를 반환하고 호출자가 이를 처리하도록 강제한다.
- **회복 불가능한 오류 (Unrecoverable Errors):** 프로그래밍 버그, 메모리 손상, 논리적 모순 등과 같이 프로그램이 더 이상 올바르게 작동할 수 없는 심각한 상황이다.
  - **`panic!` 매크로**를 호출하여 프로그램을 즉시 종료한다.

이러한 명확한 구분 덕분에 Rust 개발자는 어떤 종류의 오류를 어떻게 처리해야 할지 미리 계획하고 안전한 코드를 작성하는 데 도움을 받는다.

따라서 "패닉"은 Rust의 특정 용어지만, 그 개념(회복 불가능한 심각한 오류로 인한 프로그램 종료)은 다른 프로그래밍 언어에서도 다양한 이름으로 존재하고 있다.
