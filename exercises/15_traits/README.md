# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)

---

# Traits

\*\*트레이트(Trait)\*\*는 Rust에서 특정 타입이 가질 수 있는 \*\*공유된 동작(shared behavior)\*\*을 정의하는 방법이다. 트레이트는 메소드(methods)들의 집합으로 구성된다.

데이터 타입은 트레이트를 \*\*구현(implement)\*\*할 수 있다. 트레이트를 구현한다는 것은 해당 데이터 타입에 대해 트레이트가 정의하는 메소드들을 구현하는 것을 의미한다. 예를 들어, `String` 데이터 타입은 `From<&str>` 트레이트를 구현한다. 이는 사용자가 `String::from("hello")`와 같이 작성할 수 있도록 한다.

이러한 방식으로 트레이트는 Java의 인터페이스(interfaces)나 C++의 추상 클래스(abstract classes)와 다소 유사하다.

몇 가지 추가적인 흔한 Rust 트레이트는 다음과 같다:

- `Clone` (값을 복제하는 `clone` 메소드를 제공한다)
- `Display` ( `{}`를 통한 포맷팅된 출력을 허용한다)
- `Debug` ( `{:?}`를 통한 디버깅 목적의 포맷팅된 출력을 허용한다)

트레이트는 데이터 타입 간의 공유된 동작을 나타내기 때문에, \*\*제네릭(Generics)\*\*을 작성할 때 매우 유용하게 사용된다.

---

## Rust 트레이트의 기본 문법

### 1\. 트레이트 정의하기

`trait` 키워드를 사용하여 트레이트를 정의하고, 중괄호 `{}` 안에 트레이트를 구현하는 타입이 제공해야 할 메소드 시그니처(이름, 파라미터, 반환 타입)를 나열한다.

```rust
// Summary 트레이트를 정의한다.
// 이 트레이트를 구현하는 어떤 타입이든 summarize 메소드를 가져야 한다.
pub trait Summary {
    fn summarize(&self) -> String; // 메소드 시그니처를 정의한다.
                                  // 기본 구현을 제공할 수도 있다.
    fn author_name(&self) -> String; // 기본 구현이 없는 메소드

    // 기본 구현을 제공하는 메소드 (트레이트를 구현하는 타입이 오버라이드할 수 있다)
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### 2\. 트레이트 구현하기 (`impl` 키워드)

특정 타입에 대해 트레이트를 구현하려면 `impl TraitName for TypeName` 구문을 사용한다. 그리고 트레이트가 요구하는 모든 메소드들을 구현한다.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle 타입에 대해 Summary 트레이트를 구현한다.
impl Summary for NewsArticle {
    fn summarize(&self) -> String { // summarize 메소드를 구현한다.
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn author_name(&self) -> String { // author_name 메소드를 구현한다.
        format!("{}", self.author)
    }
    // default_summary는 오버라이드하지 않았으므로 기본 구현이 사용된다.
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Tweet 타입에 대해 Summary 트레이트를 구현한다.
impl Summary for Tweet {
    fn summarize(&self) -> String { // summarize 메소드를 구현한다.
        format!("{}: {}", self.username, self.content)
    }

    fn author_name(&self) -> String { // author_name 메소드를 구현한다.
        format!("{}", self.username)
    }
}
```

### 3\. 트레이트를 파라미터로 사용하기 (트레이트 바운드)

함수나 구조체에서 제네릭 타입이 특정 트레이트를 구현했음을 요구하기 위해 \*\*트레이트 바운드(Trait Bounds)\*\*를 사용한다. 이는 해당 제네릭 타입이 특정 동작을 가짐을 보장한다.

```rust
// Summary 트레이트를 구현하는 어떤 타입이든 이 함수의 인자로 올 수 있다.
pub fn notify(item: &impl Summary) { // impl Trait 구문 (Syntactic Sugar)
    println!("Breaking news! {}", item.summarize());
}

// 위와 동일한 표현 (Trait Bound Syntax)
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again took the Stanley Cup."),
    };

    notify(&tweet);   // 출력: Breaking news! horse_ebooks: of course, as you probably already know, people
    notify(&article); // 출력: Breaking news! Penguins win Stanley Cup!, by Iceburgh (Pittsburgh, PA)

    println!("Default summary: {}", tweet.default_summary()); // 기본 구현 사용: (Read more...)
}
```

### 4\. 트레이트를 반환 타입으로 사용하기

함수가 특정 트레이트를 구현하는 타입을 반환하도록 할 수 있다. 하지만 이때는 단일 구체적인 타입만 반환해야 한다 (예: `Tweet` 또는 `NewsArticle` 중 하나만).

```rust
// pub fn returns_summarizable(switch: bool) -> impl Summary { // 반환 타입으로 impl Trait 사용
//     if switch {
//         Tweet {
//             username: String::from("some_user"),
//             content: String::from("some content"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         // 이 경우, Tweet과 NewsArticle은 다른 구체적인 타입이므로 에러가 발생한다.
//         // 한 함수는 오직 하나의 구체적인 타입만 반환할 수 있다.
//         NewsArticle {
//             headline: String::from("another headline"),
//             location: String::from("some place"),
//             author: String::from("some author"),
//             content: String::from("some content"),
//         }
//     }
// }
```

### 5\. 파생 트레이트(Derivable Traits)

Rust는 자주 사용되는 몇몇 트레이트에 대해 개발자가 직접 구현 코드를 작성할 필요 없이 `#[derive]` 속성을 통해 자동으로 구현해주는 기능을 제공한다.

- **`Debug`:** `{:?}`를 사용하여 디버깅 형식으로 출력 가능하게 한다.
- **`Clone`:** `clone()` 메소드를 통해 깊은 복사(deep copy)를 가능하게 한다.
- **`Copy`:** 스택에 저장될 수 있는 타입에 대해 비트 단위 복사(bitwise copy)를 가능하게 하며, 소유권 이동 없이 복사되도록 한다.
- **`PartialEq`, `Eq`:** `==` 및 `!=` 연산자를 사용하여 동등성 비교를 가능하게 한다.
- **`PartialOrd`, `Ord`:** `<`, `<=`, `>`, `>=` 연산자를 사용하여 순서 비교를 가능하게 한다.
- **`Default`:** 기본값 인스턴스를 생성하는 `default()` 메소드를 제공한다.

<!-- end list -->

```rust
#[derive(Debug, Clone, PartialEq)] // 이 트레이트들을 자동으로 구현한다.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone(); // Clone 트레이트 덕분에 clone() 사용 가능
    println!("{:?}", p1); // Debug 트레이트 덕분에 {:?} 사용 가능

    assert_eq!(p1, p2); // PartialEq 트레이트 덕분에 == 연산자 사용 가능
}
```

---

## Rust 트레이트의 언어 철학

Rust의 트레이트 시스템은 다음과 같은 핵심 철학을 구현한다:

1.  **공유된 동작의 명시적 정의:**

    - 트레이트는 여러 타입이 공통적으로 수행할 수 있는 동작(메소드 집합)을 명확하게 정의하는 계약(contract) 역할을 한다. 이는 코드의 가독성과 이해도를 높인다.

2.  **다형성(Polymorphism) 구현:**

    - 트레이트 바운드를 통해 정적 다형성(컴파일 타임에 타입 결정)을 구현한다. 제네릭 함수가 특정 트레이트를 구현하는 모든 타입에 대해 작동하도록 하여 코드 재사용성을 극대화한다.
    - 동적 다형성(런타임에 타입 결정)은 트레이트 객체(trait objects, `dyn Trait`)를 통해 구현될 수 있으며, 이는 Rust가 객체 지향적 특성을 가지는 방식 중 하나다.

3.  **응집성과 확장성(Cohesion and Extensibility):**

    - 기존 타입에 새로운 동작을 추가하거나, 새로운 타입을 기존 트레이트에 맞춰 구현함으로써 시스템을 유연하게 확장할 수 있다. `impl Trait for Type` 구문을 통해 외부 크레이트의 타입에 대해서도 트레이트를 구현할 수 있다 (단, 해당 트레이트나 타입 중 하나는 로컬 크레이트에서 정의되어야 한다).

4.  **"제로 코스트 추상화"**:

    - 트레이트 바운드를 사용하는 제네릭은 **모노모피제이션(Monomorphization)** 방식으로 컴파일된다. 이는 런타임에 성능 오버헤드 없이 각 구체적인 타입에 최적화된 코드를 생성한다. 개발자는 추상화의 이점을 누리면서도 C/C++과 같은 저수준 언어의 성능을 달성할 수 있다.
    - 이는 컴파일러가 트레이트 제약을 통해 어떤 메소드를 호출해야 할지 컴파일 시점에 미리 알기 때문에 가능한 일이다.

5.  **컴파일 타임 안전성:**

    - 트레이트가 요구하는 모든 메소드가 구현되었는지 컴파일러가 엄격하게 확인한다. 이는 런타임에 "메소드가 없다"는 오류가 발생할 가능성을 제거하여 프로그램의 안정성을 높인다.

결론적으로 Rust의 트레이트는 코드 재사용, 유연한 확장성, 그리고 특히 메모리 안전성과 성능을 유지하면서 다형성을 구현하는 Rust의 근본적인 메커니즘이다.

---
