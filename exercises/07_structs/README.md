# Structs

Rust has three struct types: a classic C struct, a tuple struct, and a unit struct.

## Further information

- [Structures](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

# Structs

Rust의 \*\*구조체(Structs)\*\*는 여러 관련 데이터를 하나의 복합적인 데이터 타입으로 묶는 방법이다. 이는 데이터를 의미 있는 그룹으로 조직화하여 코드를 더 명확하고 관리하기 쉽게 만든다. Rust에는 세 가지 주요 구조체 타입이 존재한다: 클래식 C 스타일의 명명된 필드를 가진 구조체, 튜플 구조체, 그리고 유닛 구조체이다.

---

## Struct 타입의 종류와 기본 문법

### 1\. 클래식 C 스타일 구조체 (Classic C-style Structs)

가장 일반적인 형태의 구조체로, 각 필드에 이름을 붙여 데이터를 명확하게 식별할 수 있다. 이는 객체 지향 언어의 클래스나 다른 언어의 구조체와 유사하게 데이터를 캡슐화하는 데 사용된다.

#### 1.1. 정의하기

`struct` 키워드 뒤에 구조체 이름과 중괄호 안에 `필드_이름: 타입,` 형식으로 필드를 정의한다.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

#### 1.2. 인스턴스 생성하기

구조체를 사용하려면 각 필드에 값을 할당하여 해당 구조체의 인스턴스를 생성한다. 필드 순서는 중요하지 않다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 가변 인스턴스 생성 (필드 값 변경 가능)
    let mut user2 = User {
        active: false,
        username: String::from("anotheruser456"),
        email: String::from("another@example.com"),
        sign_in_count: 5,
    };
    user2.email = String::from("newemail@example.com"); // 가변 인스턴스의 필드 값 변경
    println!("User2 new email: {}", user2.email);
}
```

#### 1.3. 필드 접근하기

점 표기법(`instance.field_name`)을 사용하여 인스턴스의 특정 필드 값에 접근한다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User1 username: {}", user1.username); // 출력: User1 username: someuser123
}
```

#### 1.4. 구조체 업데이트 문법

기존 인스턴스의 일부 필드만 변경하여 새로운 인스턴스를 생성할 때 유용하다. `..` 연산자를 사용하여 나머지 필드를 복사한다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("another_new@example.com"),
        ..user1 // user1의 나머지 필드(active, username, sign_in_count)를 사용한다.
    };
    println!("User3 username: {}", user3.username); // 출력: User3 username: someuser123
    println!("User3 email: {}", user3.email);       // 출력: User3 email: another_new@example.com
}
```

**주의:** `..` 뒤의 인스턴스(`user1`)는 힙 데이터를 포함하는 필드(`String`)가 있다면, 그 필드들의 소유권이 새 인스턴스(`user3`)로 \*\*이동(move)\*\*하므로, 원본 인스턴스(`user1`)는 더 이상 유효하지 않게 된다. `Copy` 트레이트가 구현된 타입(예: `bool`, `u64`)은 복사된다.

### 2\. 튜플 구조체 (Tuple Structs)

명명된 필드 없이 튜플과 유사하게 순서에 따라 값을 가지는 구조체이다. 필드에 이름이 필요 없을 때나, 특정 타입의 튜플에 의미 있는 이름을 부여하고 싶을 때 사용한다.

#### 2.1. 정의하기

`struct` 키워드 뒤에 구조체 이름과 괄호 안에 필드의 타입을 나열한다.

```rust
struct Color(i32, i32, i32); // RGB 색상
struct Point(i32, i32, i32); // 3D 점
```

#### 2.2. 인스턴스 생성 및 접근하기

일반 튜플처럼 인스턴스를 생성하고, 인덱스를 사용하여 필드에 접근한다.

```rust
fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black R: {}", black.0);   // 출력: Black R: 0
    println!("Origin X: {}", origin.0); // 출력: Origin X: 0
}
```

### 3\. 유닛 구조체 (Unit-Like Structs)

필드가 전혀 없는 구조체이다. 특정 트레이트를 구현해야 하지만 저장할 데이터는 없는 경우에 유용하다. 이름은 있지만 튜플이나 클래식 구조체처럼 동작하지 않으므로 "유닛(Unit)"이라고 불린다.

#### 3.1. 정의하기

`struct` 키워드 뒤에 구조체 이름만 명시한다.

```rust
struct AlwaysEqual; // 항상 같다고 가정하는 타입
```

#### 3.2. 인스턴스 생성 및 사용

인스턴스 생성 시 필드를 명시하지 않는다. 주로 트레이트 구현 시 사용된다.

```rust
fn main() {
    let subject = AlwaysEqual; // 인스턴스 생성
    // 데이터가 없으므로 필드 접근은 불가하다.
}
```

---

## 메소드 문법 (Method Syntax)

구조체는 데이터를 캡슐화할 뿐만 아니라, 그 데이터에 대한 동작을 정의하는 \*\*메소드(methods)\*\*를 가질 수 있다. 메소드는 `impl` 블록 내에서 정의되며, 첫 번째 파라미터는 항상 구조체 인스턴스에 대한 참조(`&self`, `&mut self`, 또는 `self`)여야 한다.

### 1\. 메소드 정의하기

`impl` (implementation) 블록은 특정 구조체에 대한 함수를 정의하는 곳이다.

- **`&self`:** 메소드가 구조체의 불변 참조를 빌린다. 인스턴스의 데이터를 읽기만 할 때 사용하며, 원본 인스턴스는 변경되지 않고 여전히 사용 가능하다.
- **`&mut self`:** 메소드가 구조체의 가변 참조를 빌린다. 인스턴스의 데이터를 변경할 때 사용하며, 다른 참조가 동시에 존재할 수 없다.
- **`self`:** 메소드가 구조체 인스턴스의 소유권을 가져간다. 인스턴스를 소비하거나(consume) 변환할 때 사용한다. 메소드 호출 후 원본 인스턴스는 더 이상 유효하지 않다.

<!-- end list -->

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 불변 참조를 빌리는 메소드 (읽기 전용)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 가변 참조를 빌리는 메소드 (데이터 변경)
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 소유권을 가져가는 메소드 (인스턴스를 소비)
    // fn consume_and_describe(self) -> String {
    //     format!("Consumed rectangle: {}x{}", self.width, self.height)
    // }

    // 연관 함수 (Associated Functions) - 첫 파라미터가 self가 아님.
    // 주로 생성자처럼 사용되며, 구조체 이름::함수_이름() 형식으로 호출된다.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

### 2\. 메소드 호출하기

메소드는 점 표기법(`instance.method_name()`)을 사용하여 호출한다. 연관 함수는 `StructName::function_name()` 형식으로 호출한다.

```rust
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // rect1의 불변 참조를 전달한다.
    ); // 출력: The area of the rectangle is 1500 square pixels.

    let mut rect2 = Rectangle { width: 10, height: 20 };
    rect2.scale(2); // rect2의 가변 참조를 전달한다.
    println!("Scaled rect2: {}x{}", rect2.width, rect2.height); // 출력: Scaled rect2: 20x40

    let square = Rectangle::square(25); // 연관 함수 호출
    println!("Square area: {}", square.area()); // 출력: Square area: 625
}
```

---

## Rust의 Structs 구현 철학

Rust의 구조체 디자인은 언어의 핵심 철학을 강력하게 반영한다:

1.  **데이터와 동작의 응집(Cohesion):** 구조체와 `impl` 블록을 통해 관련 데이터를 함께 묶고, 그 데이터에 대해 작동하는 메소드를 정의하여 코드를 논리적으로 조직화한다. 이는 코드의 가독성과 유지보수성을 높인다.

2.  **명시적인 소유권 및 빌림:** 메소드 정의 시 `&self`, `&mut self`, `self`를 사용하여 데이터에 대한 접근 권한(읽기 전용, 쓰기 가능, 소유권 이전)을 명시적으로 선언하도록 강제한다. 이는 컴파일 타임에 \*\*메모리 안전성(Memory Safety)\*\*과 **데이터 경쟁 방지**를 보장하는 핵심 메커니즘이다. 개발자는 메소드가 원본 인스턴스를 변경할지, 소유권을 가져갈지 등을 명확하게 알 수 있다.

3.  **"제로 코스트 추상화"**: 구조체는 C++의 클래스와 유사하게, 데이터에 대한 추상화를 제공하지만, 런타임 오버헤드가 거의 없다. 즉, 개발자가 직접 메모리 레이아웃을 최적화한 것과 거의 동일한 성능을 달성할 수 있도록 설계되었다. 이는 저수준 제어와 고수준 추상화 사이의 균형을 제공한다.

4.  **유연성과 다용도성:**

    - **명명된 필드:** 데이터의 의미를 명확하게 전달하며, 규모가 큰 데이터 묶음에 적합하다.
    - **튜플 구조체:** 단순한 데이터 묶음(예: 좌표, 색상 값)에 간결함을 제공하며, 필드 이름 없이 순서로 접근하는 것이 더 자연스러울 때 사용된다.
    - **유닛 구조체:** 데이터 없이 논리적인 개념을 나타내거나 트레이트 구현에 필요한 마커로 사용되어, 다양한 설계 패턴에 유연성을 더한다.

Rust의 구조체는 데이터의 구조와 그 데이터에 대한 연산을 명확하고 안전하며 효율적인 방식으로 정의하도록 돕는 강력한 도구이다.
