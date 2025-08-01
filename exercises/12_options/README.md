# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)

---

# Options

**Option** 타입은 선택적 값(optional value)을 나타내는 열거형(enum)이다. 모든 `Option`은 값이 존재하는 경우를 나타내는 **`Some`** 배리언트를 가지거나, 값이 존재하지 않는 경우를 나타내는 **`None`** 배리언트를 가진다. `Option` 타입은 Rust 코드에서 매우 흔하게 사용되며, 다양한 용도로 활용된다.

---

## `Option` 열거형의 기본 개념과 사용

`Option<T>` 열거형은 Rust 표준 라이브러리에 정의되어 있으며, 어떤 값이 존재할 수도 있고 존재하지 않을 수도 있는 상황을 안전하고 명시적으로 다루기 위해 사용된다. Rust는 다른 언어의 `null` 개념을 직접 사용하지 않고 `Option`을 통해 이를 처리한다.

### 1\. `Option` 열거형의 정의

`Option`은 제네릭(Generic) 타입 `T`를 사용하여, 어떤 타입의 값이라도 담을 수 있도록 설계되었다.

```rust
enum Option<T> {
    None,    // 값이 없음을 나타낸다.
    Some(T), // T 타입의 값이 존재함을 나타내며, 그 값을 T에 담는다.
}
```

### 2\. `Option` 인스턴스 생성하기

- **`Some(value)`:** 값이 존재하는 경우에 사용한다.
  ```rust
  let some_number = Some(5);          // Option<i32> 타입이다.
  let some_string = Some("a string"); // Option<&str> 타입이다.
  ```
- **`None`:** 값이 존재하지 않는 경우에 사용한다.
  ```rust
  let absent_number: Option<i32> = None; // None은 타입 유추가 안되므로 명시해야 한다.
  ```

### 3\. `Option` 값을 다루는 주요 방법

`Option`은 그 자체로 값을 직접 사용할 수 없다. `Some` 안의 값에 접근하려면 \*\*패턴 매칭(Pattern Matching)\*\*을 사용해야 한다.

#### 3.1. `match` 문 사용하기

`match`는 `Option` 값을 안전하게 언패킹(unwrapping)하고 처리하는 가장 강력하고 권장되는 방법이다. `None` 케이스를 반드시 처리하도록 강제하여 빠뜨리는 경우를 방지한다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,       // x가 None이면 None을 반환한다.
        Some(i) => Some(i + 1), // x가 Some(i)이면 i를 1 증가시켜 Some(새 값)을 반환한다.
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);    // six는 Some(6)이 된다.
    let none = plus_one(None);   // none은 None이 된다.

    println!("{:?}", six);   // 출력: Some(6)
    println!("{:?}", none);  // 출력: None
}
```

#### 3.2. `if let` 사용하기

`match` 문은 모든 가능한 경우를 처리해야 하지만, 특정 `Some` 값에만 관심이 있고 `None`일 때는 아무것도 하지 않아도 될 때 `if let`을 사용하면 더 간결하게 코드를 작성할 수 있다.

```rust
fn main() {
    let config_max = Some(3u8);

    // config_max가 Some(max) 패턴과 일치할 때만 코드를 실행한다.
    if let Some(max) = config_max {
        println!("The maximum is: {}", max); // 출력: The maximum is: 3
    }

    let no_value: Option<u8> = None;
    if let Some(val) = no_value {
        println!("This will not print: {}", val); // 실행되지 않는다.
    }
}
```

#### 3.3. `while let` 사용하기

`Option`을 반복적으로 처리하여 `None`이 될 때까지 `Some` 값을 언패킹하고 싶을 때 `while let`을 사용한다.

```rust
fn main() {
    let mut stack = vec![1, 2, 3];

    // stack.pop()이 Some(value)를 반환하는 동안 루프를 실행한다.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 출력:
    // 3
    // 2
    // 1
}
```

#### 3.4. `unwrap()` 및 `expect()` (경고: 주의해서 사용\!)

`unwrap()`과 `expect()`는 `Option` 값을 언패킹하는 가장 간단한 방법이다. 하지만 `Option`이 `None`일 때 호출하면 \*\*패닉(panic)\*\*을 일으켜 프로그램을 강제 종료시킨다.

- **`unwrap()`:** `Some(value)`일 경우 `value`를 반환하고, `None`일 경우 패닉을 일으킨다.
- **`expect("error message")`:** `unwrap()`과 동일하게 동작하지만, `None`일 때 패닉 메시지를 지정할 수 있어 디버깅에 유용하다.

이 메소드들은 값이 `None`이 될 가능성이 절대 없다고 확신할 때만 사용해야 한다.

```rust
fn main() {
    let some_number = Some(10);
    let value = some_number.unwrap(); // value는 10이 된다.

    // let no_value: Option<i32> = None;
    // let result = no_value.unwrap(); // 여기서 패닉 발생!
    // let result = no_value.expect("값이 있어야 하는데 없습니다!"); // 사용자 정의 메시지와 함께 패닉 발생!
}
```

---

## `Option` 타입의 활용 사례 (주요 용도)

`Option` 타입은 다양한 상황에서 유용하게 사용된다.

- **초기 값:** 아직 값이 할당되지 않았지만 나중에 할당될 수 있는 필드나 변수를 나타낼 때 `None`으로 초기화한다.
- **부분 함수(Partial Functions)의 반환 값:** 모든 입력에 대해 정의되지 않는 함수(예: 문자열을 숫자로 변환하는 함수는 유효하지 않은 문자열에 대해 값을 반환하지 못할 수 있다)의 반환 타입으로 사용된다.
- **간단한 오류 보고:** 성공 시 `Some(T)`를 반환하고 오류 시 `None`을 반환하여 간단한 오류 상황을 나타낸다. (복잡한 오류에는 `Result<T, E>`를 사용한다.)
- **선택적 구조체 필드:** 구조체의 특정 필드가 항상 존재할 필요는 없을 때 해당 필드를 `Option<T>` 타입으로 선언한다.
- **`null` 가능한 포인터 대체:** 다른 언어의 `null` 포인터가 야기하는 문제를 해결한다. Rust에는 `null`이 없으므로 `Option<&T>` (선택적 참조)로 대체하여 사용한다.
- **값 스왑(swapping) 또는 "가져오기":** 구조체 내부의 값을 변경하면서 기존 값을 안전하게 가져와야 할 때 `Option`과 `take()` 또는 `replace()` 메소드를 활용할 수 있다.

---

## Rust `Option`의 언어 철학

Rust의 `Option` 타입은 다음 언어 철학을 강력하게 반영한다:

1.  **널 포인터 역참조(Null Pointer Dereference) 오류 방지:** `null` 개념이 없으므로, 값이 없을 가능성이 있는 모든 상황을 `Option` 타입으로 명시하도록 강제한다. 이는 "십억 달러짜리 실수(The Billion-Dollar Mistake)"로 불리는 널 참조 관련 런타임 오류를 컴파일 타임에 근본적으로 차단한다.
2.  **안전성(Safety) 및 견고성(Robustness):** 개발자가 `None`일 가능성을 **반드시** 명시적으로 처리하도록 요구한다(`match`의 완전성). 이를 통해 프로그램이 예상치 못한 `None` 값에 의해 비정상 종료되거나 불안정한 상태에 빠지는 것을 방지하여 코드의 견고성을 높인다.
3.  **명시성(Explicitness)과 가독성(Readability):** 어떤 함수나 변수가 값이 없을 수도 있다는 사실을 타입 시스템 자체에서 명확하게 드러낸다. 이는 코드의 의도를 명확하게 하고, 개발자가 잠재적인 문제를 미리 인지하고 처리하도록 돕는다.
4.  **"제로 코스트 추상화"**: `Option`은 강력한 안전성 추상화를 제공하지만, 런타임 오버헤드가 거의 없다. 컴파일러는 `Option`을 효율적인 기계어 코드로 변환하여, 개발자가 수동으로 검사를 수행하는 것과 유사한 성능을 낸다. `Some`은 그저 값을 그대로 저장하고, `None`은 빈 값이나 작은 태그로 표현된다.

`Option`은 Rust가 안전하고 신뢰할 수 있는 소프트웨어를 구축하는 데 핵심적인 역할을 하는 기능 중 하나이다.

---
