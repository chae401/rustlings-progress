# Strings

Rust has two string types, a string slice (`&str`) and an owned string (`String`).
We're not going to dictate when you should use which one, but we'll show you how
to identify and create them, as well as use them.

## Further information

- [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

# Strings

Rust는 텍스트 데이터를 다루는 데 두 가지 주요 문자열 타입을 제공한다: \*\*문자열 슬라이스(`&str`)\*\*와 \*\*소유된 문자열(`String`)\*\*이다. 이 두 타입은 Rust의 소유권(ownership) 시스템과 깊이 연관되어 있으며, 메모리 관리 방식과 사용 목적에서 차이가 있다. 어떤 타입을 언제 사용해야 할지 강제하지는 않지만, 이들을 식별하고 생성하며 사용하는 방법을 보여줄 것이다.

---

## Rust의 두 가지 문자열 타입

Rust는 UTF-8 인코딩을 지원하며, 텍스트 데이터를 안전하고 효율적으로 다룰 수 있도록 두 가지 주요 문자열 타입을 설계했다.

### 1\. 문자열 슬라이스 (`&str`)

\*\*문자열 슬라이스(`&str`)\*\*는 불변(immutable)이며, UTF-8로 인코딩된 문자열 데이터에 대한 \*\*참조(reference)\*\*이다. 문자열 리터럴(string literal)은 모두 `&str` 타입이다. 이들은 컴파일 시점에 프로그램의 바이너리에 직접 포함된다.

#### 1.1. 특징

- **불변(Immutable):** 생성된 후에는 내용을 변경할 수 없다.
- **고정 크기:** 컴파일 시점에 크기가 결정된다.
- **참조(Reference):** 특정 위치의 문자열 데이터를 "빌려온" 것이다. 소유권이 없다.
- **빠른 생성:** 별도의 런타임 메모리 할당이 필요 없다.
- **주로 함수 인자로 사용:** 문자열 데이터를 읽기 전용으로 전달할 때 유용하다.

#### 1.2. 생성 및 사용

```rust
fn main() {
    let s: &str = "Hello, world!"; // 문자열 리터럴은 &str 타입이다.

    let slice_from_string: &str;
    {
        let my_string = String::from("Rust programming");
        slice_from_string = &my_string[0..4]; // String에서 &str 슬라이스를 생성한다.
                                            // 주의: my_string의 생명 주기보다 slice_from_string의 생명 주기가 짧아야 한다.
    } // my_string이 여기서 드롭되므로, slice_from_string은 더 이상 유효하지 않다.
      // 실제로는 컴파일러가 'dangling reference'를 방지한다.
      // println!("{}", slice_from_string); // 이 시점에서 사용하면 컴파일 에러가 발생한다.
}
```

---

### 2\. 소유된 문자열 (`String`)

\*\*소유된 문자열(`String`)\*\*은 성장 가능(growable, 크기를 바꿀 수 있다)하고 가변(mutable, 내용을 바꿀 수 있다)인 UTF-8로 인코딩된 문자열 데이터다. 이 데이터는 \*\*힙(heap)\*\*에 할당되며, 런타임에 크기가 커지거나 줄어들 수 있다. `String` 타입은 Rust의 Vector와 유사하게 동작하며, 데이터를 소유한다.

#### 2.1. 특징

- **가변(Mutable):** 생성된 후에도 내용을 변경할 수 있다.
- **동적 크기:** 런타임에 크기를 조절할 수 있다.
- **소유권(Ownership):** 데이터를 직접 소유하며, 스코프를 벗어나면 메모리가 자동으로 해제된다.
- **더 많은 런타임 오버헤드:** 힙 할당이 필요하므로 `&str`보다 생성 및 변경에 비용이 더 든다.
- **주로 데이터 소유 및 변경이 필요할 때 사용:** 사용자 입력, 파일 내용 읽기 등에 적합하다.

#### 2.2. 생성 및 사용

```rust
fn main() {
    // 1. String::new()로 빈 String 생성
    let mut s1 = String::new();
    s1.push_str("hello"); // 문자열 추가
    println!("s1: {}", s1); // 출력: s1: hello

    // 2. to_string() 메소드 사용 (모든 Display 트레이트 구현 타입)
    let s2 = "world".to_string(); // &str 리터럴을 String으로 변환한다.
    println!("s2: {}", s2); // 출력: s2: world

    // 3. String::from() 함수 사용 (더 명시적)
    let s3 = String::from("Rust is fun");
    println!("s3: {}", s3); // 출력: s3: Rust is fun

    // 4. String 연결
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let s4 = hello + &world; // + 연산자를 사용하여 String과 &str을 연결한다.
                             // 주의: hello의 소유권은 s4로 이동하고, world는 &str로 빌려져 사용된다.
                             // println!("{}", hello); // 에러 발생! hello는 이미 이동했다.
    println!("s4: {}", s4); // 출력: Hello, world!

    // 5. format! 매크로 (가장 유연)
    let s5 = format!("{}-{}", s3, s2); // 여러 String 및 &str을 조합하여 새로운 String 생성
    println!("s5: {}", s5); // 출력: Rust is fun-world
}
```

---

## `&str`과 `String` 간의 변환

`String`은 `&str`을 빌려올 수 있고(`&my_string[..]`), `&str`은 `String`으로 소유권을 가질 수 있다(`"literal".to_string()` 또는 `String::from("literal")`). 이 유연성은 Rust의 소유권 및 빌림 시스템에 기반을 둔다.

- `String`에서 `&str` 얻기: `&my_string` 또는 `&my_string[..]`
- `&str`에서 `String` 얻기: `my_str.to_string()` 또는 `String::from(my_str)`

---

## Rust의 String 구현 철학

Rust가 두 가지 문자열 타입을 사용하는 것은 언어의 핵심 철학인 **안전성(Safety)**, **성능(Performance)**, 그리고 \*\*명시성(Explicitness)\*\*을 반영한다.

1.  **메모리 관리의 명확성:**

    - **`&str` (스택/정적 메모리):** 불변 데이터를 빠르게 접근하고 메모리 오버헤드를 줄일 때 사용한다. 이들은 프로그램 바이너리나 다른 `String`의 일부로 존재하며, 자체적으로 메모리를 할당하거나 해제하지 않는다. 이는 예측 가능한 성능을 제공한다.
    - **`String` (힙 메모리):** 런타임에 크기가 변동될 수 있는 동적인 데이터를 다룰 때 사용한다. 힙 할당은 더 많은 유연성을 제공하지만, 스택 할당보다 미세한 오버헤드가 발생한다. Rust는 `String`이 스코프를 벗어나면 자동으로 힙 메모리를 해제하여 \*\*메모리 누수(memory leak)\*\*를 방지한다.

2.  **안전한 문자열 처리:**

    - **UTF-8 기본 지원:** Rust의 모든 문자열 타입은 UTF-8을 기본으로 사용한다. 이는 전 세계 다양한 언어의 문자를 안전하게 처리할 수 있도록 돕는다.
    - **경계 검사:** 문자열 슬라이스를 만들거나 인덱스로 접근할 때, Rust는 항상 유효한 UTF-8 경계를 따르는지, 그리고 인덱스가 범위를 벗어나지 않는지 확인한다. 이는 런타임 패닉을 유발할 수 있지만, 메모리 손상과 같은 심각한 오류를 방지한다.
    - **소유권 시스템과의 통합:** `String`은 소유권 규칙을 따르므로, 이중 해제나 댕글링 포인터와 같은 메모리 안전 문제를 컴파일 타임에 방지한다.

3.  **성능과 유연성의 균형:**

    - Rust는 개발자가 데이터의 수명과 가변성에 따라 가장 적절한 문자열 타입을 선택하도록 유도한다. 이는 불필요한 메모리 할당(복사)을 피하고 성능을 최적화하는 데 도움이 된다.
    - `&str`은 `String`의 일부를 "빌려서" 읽기 전용으로 사용할 때 매우 효율적이다. `String`은 필요한 경우에만 동적인 할당을 수행한다.

결론적으로, Rust의 두 가지 문자열 타입은 개발자가 메모리 안전성과 성능이라는 두 마리 토끼를 모두 잡을 수 있도록 설계된 강력한 도구다. 이들을 이해하고 적절히 활용하는 것이 Rust 프로그래밍의 핵심이다.

---
