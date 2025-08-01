# Generics

Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for some rather involved syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.

## Further information

- [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

---

# Generics

\*\*제네릭(Generics)\*\*은 타입과 기능을 더 넓은 범위의 케이스로 일반화하는 개념이다. 이는 다양한 방법으로 코드 중복을 줄이는 데 매우 유용하지만, 다소 복잡한 문법을 필요로 할 수 있다. 특히, 제네릭을 사용할 때는 제네릭 타입이 실제로 어떤 타입에 대해 유효한지 매우 신중하게 지정해야 한다. 제네릭의 가장 간단하고 흔한 용도는 \*\*타입 파라미터(Type Parameters)\*\*를 사용하는 것이다.

---

## Rust 제네릭의 기본 문법

제네릭은 함수, 구조체, 열거형, 메소드 등 다양한 곳에서 사용할 수 있으며, `<T>`와 같은 타입 파라미터를 사용하여 정의한다. 여기서 `T`는 어떤 구체적인 타입이든 될 수 있는 **플레이스홀더(placeholder)** 역할을 한다.

### 1\. 함수에서 제네릭 사용하기

함수가 여러 다른 타입에 대해 동일한 로직으로 작동해야 할 때 제네릭을 사용한다.

```rust
// 두 숫자를 비교하여 더 큰 값을 반환하는 제네릭 함수
// PartialOrd는 부분 순서 비교를, Copy는 값 복사를 가능하게 하는 트레이트 바운드다.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result); // 출력: The largest number is 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result); // 출력: The largest char is q
}
```

### 2\. 구조체에서 제네릭 사용하기

구조체가 하나 이상의 제네릭 타입을 포함하는 필드를 가질 때 사용한다.

```rust
// 두 개의 동일한 제네릭 타입 T를 가지는 Point 구조체
struct Point<T> {
    x: T,
    y: T,
}

// 두 개의 다른 제네릭 타입 T와 U를 가지는 Point 구조체
struct PointDifferent<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 }; // x와 y 모두 i32 타입이다.
    let float_point = Point { x: 1.0, y: 4.0 }; // x와 y 모두 f64 타입이다.

    // let mixed_point = Point { x: 5, y: 4.0 }; // 에러 발생! x와 y는 같은 타입이어야 한다.

    let mixed_point_different = PointDifferent { x: 5, y: 4.0 }; // x는 i32, y는 f64 타입이다.
    println!("Mixed point: ({}, {})", mixed_point_different.x, mixed_point_different.y);
}
```

### 3\. 열거형에서 제네릭 사용하기

열거형의 배리언트가 제네릭 타입을 포함하는 데이터를 가질 때 사용한다. Rust의 `Option<T>`와 `Result<T, E>`가 대표적인 예시이다.

```rust
enum MyOption<T> {
    MyNone,
    MySome(T),
}

fn main() {
    let integer_option = MyOption::MySome(5);
    let string_option = MyOption::MySome(String::from("hello"));
    let no_value: MyOption<i32> = MyOption::MyNone; // 타입 추론을 위해 명시해야 한다.
}
```

### 4\. 메소드에서 제네릭 사용하기

`impl` 블록에서 제네릭 타입을 사용하는 구조체에 대한 메소드를 정의할 수 있다.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> { // impl 다음에 <T>를 선언하여 Point<T>의 메소드임을 나타낸다.
    fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { // 특정 타입(f32)에 대해서만 메소드를 구현할 수도 있다.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.get_x()); // 출력: p.x = 5

    let p_float = Point { x: 3.0f32, y: 4.0f32 };
    println!("Distance from origin: {}", p_float.distance_from_origin()); // 출력: Distance from origin: 5
}
```

---

## 제네릭의 제약(Bounds)과 트레이트(Traits)

제네릭 타입을 사용할 때 모든 타입에 대해 코드가 의미가 있거나 컴파일될 수 있는 것은 아니다. 예를 들어, 제네릭 타입 `T`에 대해 `<` 연산을 사용하려면 `T`가 반드시 비교 가능한 타입이어야 한다. 이때 \*\*트레이트 바운드(Trait Bounds)\*\*를 사용하여 제네릭 타입이 특정 트레이트를 구현했음을 명시하여 제약을 가한다.

- `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T` 예시에서 `T: PartialOrd + Copy`는 `T`가 부분 순서 비교(`PartialOrd`)가 가능하고 복사(`Copy`) 가능한 타입이어야 함을 의미한다.
- 이는 제네릭 함수의 내부에서 `T` 타입의 값에 대해 `>` 비교 연산이나 값 복사(`for &item`)를 사용할 수 있도록 보장한다.

---

## Rust 제네릭의 언어 철학

Rust의 제네릭은 다음 언어 철학을 깊이 반영한다:

1.  **코드 재사용 및 중복 감소:**

    - 동일한 알고리즘이나 데이터 구조를 여러 다른 타입에 대해 재작성할 필요 없이 한 번만 정의하여 사용할 수 있게 한다. 이는 개발 생산성을 높이고 코드의 일관성을 유지한다.

2.  **컴파일 타임 안전성:**

    - Rust의 제네릭은 C++의 템플릿과 유사하게 **모노모피제이션(Monomorphization)** 방식으로 컴파일된다. 즉, 제네릭 코드가 사용되는 각 구체적인 타입에 대해 별도의 고유한 버전의 코드를 생성한다. 이는 런타임 오버헤드를 발생시키지 않으면서도, 컴파일 타임에 모든 타입 체크를 수행하여 \*\*타입 안전성(Type Safety)\*\*을 강력하게 보장한다.
    - 런타임에 제네릭 타입에 대한 동적 디스패치나 타입 검사를 수행할 필요가 없어, C++ 템플릿과 유사한 뛰어난 런타임 성능을 제공한다.

3.  **명시적인 제약(Bounds)을 통한 유연성:**

    - 트레이트 바운드를 통해 제네릭 타입에 대한 명확한 제약을 설정한다. 이는 개발자가 어떤 타입이 제네릭 코드에 사용될 수 있는지 명확하게 이해하고, 필요한 기능을 트레이트 형태로 요구하여 유연성을 유지하도록 한다.
    - 예를 들어, "나는 `T`가 어떤 타입이든 상관없지만, 적어도 `Display` 트레이트가 구현되어 있어서 화면에 출력할 수 있어야 한다"와 같이 요구할 수 있다.

4.  **"제로 코스트 추상화"**:

    - 제네릭은 높은 수준의 추상화를 제공하지만, 런타임 오버헤드가 거의 없다. 컴파일러는 제네릭 코드를 각 특정 타입에 맞게 최적화된 코드로 변환한다. 이는 개발자가 추상화의 이점을 누리면서도 C/C++과 같은 저수준 언어의 성능을 달성할 수 있도록 돕는다.

결론적으로 Rust의 제네릭은 타입 안전성, 성능, 그리고 코드 재사용성이라는 Rust의 핵심 가치를 동시에 만족시키며, 유연하고 효율적인 프로그래밍을 가능하게 하는 강력한 기능이다.
