# Hashmaps

A _hash map_ allows you to associate a value with a particular key.
You may also know this by the names [_unordered map_ in C++](https://en.cppreference.com/w/cpp/container/unordered_map),
[_dictionary_ in Python](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) or an _associative array_ in other languages.

This is the other data structure that we've been talking about before, when
talking about Vecs.

## Further information

- [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

---

---

# Hash Maps

\*\*해시 맵(Hash Map)\*\*은 키(key)와 값(value)을 연결하여 데이터를 저장하는 컬렉션 타입이다. 다른 프로그래밍 언어에서는 C++의 **비정렬 맵(unordered map)**, Python의 **딕셔너리(dictionary)**, 또는 다른 언어의 **연관 배열(associative array)** 등으로 알려져 있다.

이전 섹션에서 Vector를 다룰 때 언급했던 또 다른 유용한 데이터 구조이다. 해시 맵은 특정 값을 빠르게 찾거나 변경해야 할 때 매우 효율적이다.

---

## Hash Map의 기본 문법

Rust의 표준 라이브러리에서 해시 맵은 `std::collections::HashMap`으로 제공된다. 사용하려면 먼저 `use` 선언을 통해 스코프로 가져와야 한다.

### 1\. Hash Map 생성하기

새로운 빈 해시 맵을 생성하려면 `HashMap::new()` 함수를 사용한다. 해시 맵은 키와 값 모두에 대해 타입을 명시해야 한다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new(); // String 키와 i32 값의 HashMap을 생성한다.
}
```

### 2\. 값 추가하기 (`insert`)

해시 맵에 키-값 쌍을 추가하려면 `insert()` 메소드를 사용한다. 해시 맵은 가변(mutable)이어야 한다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // 타입은 추론된다.

    scores.insert(String::from("Blue"), 10); // "Blue" 키에 10을 연결한다.
    scores.insert(String::from("Yellow"), 50); // "Yellow" 키에 50을 연결한다.

    println!("{:?}", scores); // 출력: {"Blue": 10, "Yellow": 50} (순서는 다를 수 있다)
}
```

### 3\. 값 읽기 (`get`)

해시 맵에서 키에 해당하는 값을 읽으려면 `get()` 메소드를 사용한다. `get()`은 `Option<&V>`를 반환하는데, 값이 존재하면 `Some(&value)`를, 존재하지 않으면 `None`을 반환하므로 안전하게 값을 다룰 수 있다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // team_name의 불변 참조를 전달한다.

    match score {
        Some(s) => println!("Blue team score: {}", s), // 출력: Blue team score: 10
        None => println!("Team not found."),
    }

    let team_name2 = String::from("Red");
    let score2 = scores.get(&team_name2);
    match score2 {
        Some(s) => println!("Red team score: {}", s),
        None => println!("Team not found."), // 출력: Team not found.
    }
}
```

### 4\. 값 업데이트하기

해시 맵에 이미 존재하는 키에 `insert()`를 호출하면, 이전 값이 새로운 값으로 덮어쓰기된다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // "Blue" 키의 값이 10에서 25로 덮어쓰기된다.

    println!("{:?}", scores); // 출력: {"Blue": 25}
}
```

#### 4.1. 키가 없을 때만 값 삽입하기 (`entry().or_insert()`)

`entry()` 메소드는 특정 키에 대한 **Entry** 열거형을 반환한다. `Entry`는 `Occupied` (키가 존재함) 또는 `Vacant` (키가 존재하지 않음) 배리언트 중 하나이다. 이와 함께 `or_insert()` 메소드를 사용하면 키가 존재하지 않을 때만 값을 삽입할 수 있다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // "Yellow" 키가 없으므로 50을 삽입한다.
    scores.entry(String::from("Blue")).or_insert(50);    // "Blue" 키가 이미 있으므로 덮어쓰지 않는다.

    println!("{:?}", scores); // 출력: {"Blue": 10, "Yellow": 50} (순서는 다를 수 있다)
}
```

#### 4.2. 기존 값을 기반으로 값 업데이트하기

`entry()`와 `or_insert()`는 특정 키에 대한 가변 참조(`&mut V`)를 반환하므로, 기존 값을 기반으로 값을 업데이트할 때도 사용할 수 있다.

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(String::from(word)).or_insert(0); // 키가 없으면 0으로 초기화하고, 키가 있으면 기존 값의 가변 참조를 얻는다.
        *count += 1; // 가변 참조를 역참조하여 값을 1 증가시킨다.
    }

    println!("{:?}", map);
    // 출력: {"hello": 1, "world": 2, "wonderful": 1} (순서는 다를 수 있다)
ㅈ}
```

### 5\. Hash Map 순회하기

`for` 루프를 사용하여 해시 맵의 모든 키-값 쌍을 순회할 수 있다.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores { // 불변 참조로 순회한다.
        println!("{}: {}", key, value);
    }
    // 출력 (순서는 다를 수 있다):
    // Blue: 10
    // Yellow: 50
}
```

### 6\. 키와 값의 소유권

- 해시 맵에 값을 삽입할 때, 해시 맵은 해당 값들의 소유권을 가져간다.
- `Copy` 트레이트가 구현된 타입(예: `i32`, `f64`)은 해시 맵에 복사되어 들어가므로, 삽입 후에도 원본 변수를 사용할 수 있다.
- `String`과 같이 `Copy` 트레이트가 구현되지 않은 타입은 소유권이 해시 맵으로 \*\*이동(move)\*\*하므로, 삽입 후에는 원본 변수를 사용할 수 없다.

<!-- end list -->

```rust
use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name과 field_value의 소유권이 map으로 이동한다.

    // println!("{}{}", field_name, field_value); // 에러 발생! 소유권이 이동했다.
}
```

---

## Rust Hash Map 구현 철학

Rust의 해시 맵(`HashMap`) 디자인은 언어의 핵심 철학인 **안전성(Safety)**, **성능(Performance)**, 그리고 \*\*명시성(Explicitness)\*\*을 반영한다.

1.  **메모리 안전성 보장:**

    - **소유권 시스템과의 통합:** 해시 맵은 삽입된 키와 값의 소유권을 가져간다. 이는 해시 맵이 스코프를 벗어날 때 내부에 저장된 모든 데이터를 안전하게 해제하여 \*\*메모리 누수(memory leak)\*\*를 방지한다는 것을 의미한다.
    - **참조 반환과 `Option`:** `get()` 메소드가 `Option<&V>`를 반환함으로써, 키가 존재하지 않을 때 \*\*널 포인터 역참조(null pointer dereference)\*\*와 같은 런타임 오류를 컴파일 타임에 방지하도록 개발자에게 명시적으로 오류 처리를 강제한다.

2.  **예측 가능한 성능:**

    - 해시 맵은 평균적으로 $O(1)$ 시간 복잡도(상수 시간)로 삽입, 검색, 삭제 작업을 수행한다. 이는 대규모 데이터 세트에서 매우 효율적이다.
    - Worst-case 시나리오(예: 해시 충돌이 매우 심한 경우)에서는 성능이 저하될 수 있지만, Rust는 기본적으로 암호학적으로 강력한 해시 함수를 사용하여 이러한 위험을 줄인다. 개발자는 필요에 따라 커스텀 해시 함수를 지정할 수 있다.

3.  **명시적인 키-값 타입:**

    - 해시 맵을 선언할 때 키와 값의 타입을 명시해야 한다. 이는 타입 시스템의 강점을 활용하여 컴파일 타임에 타입 불일치 오류를 잡아낸다.
    - 키로 사용되는 타입은 `Eq`와 `Hash` 트레이트를 구현해야 한다. 이는 Rust가 해시 맵을 올바르게 작동시키기 위해 필요한 조건을 명시적으로 요구하는 방식이다.

4.  **유연한 데이터 관리:**

    - `entry().or_insert()`와 같은 메소드는 키의 존재 여부에 따라 유연하게 값을 삽입하거나 업데이트할 수 있도록 돕는다. 이는 코드를 간결하게 만들고 일반적인 사용 패턴을 효율적으로 처리한다.

결론적으로 Rust의 해시 맵은 키-값 데이터를 효율적이고 안전하게 관리하기 위한 강력한 도구다. Rust의 핵심 원칙을 따르면서도 개발자에게 강력한 기능을 제공하여 다양한 애플리케이션에서 사용될 수 있다.

---
