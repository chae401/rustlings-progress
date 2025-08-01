# Iterators

This section will teach you about Iterators.

## Further information

- [Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator documentation](https://doc.rust-lang.org/stable/std/iter/)

---

# Iterators

\*\*이터레이터(Iterator)\*\*는 Rust에서 컬렉션의 요소를 순회하는 표준적이고 효율적인 방법이다. 이 섹션은 이터레이터에 대해 자세히 설명한다. 이터레이터는 컬렉션의 모든 요소를 하나씩 처리할 수 있는 일련의 값들을 추상화한 것으로, 지연(lazy) 방식으로 작동하여 필요한 시점에만 값을 생성한다.

---

## Rust 이터레이터의 기본 개념과 문법

이터레이터는 `Iterator` 트레이트가 정의하는 메소드를 통해 구현된다. 이 트레이트를 구현하는 어떤 타입이든 이터레이터가 될 수 있다.

### 1\. `Iterator` 트레이트

`Iterator` 트레이트는 다음과 같은 필수 메소드를 정의한다:

```rust
pub trait Iterator {
    type Item; // 이터레이터가 생산하는 요소의 타입을 정의하는 연관 타입이다.

    fn next(&mut self) -> Option<Self::Item>; // 이터레이터의 핵심 메소드다.
                                             // 다음 요소를 Some(value)로 반환하거나,
                                             // 더 이상 요소가 없으면 None을 반환한다.
    // 이 외에도 map, filter, fold 등 다양한 기본 메소드들이 구현되어 있다.
}
```

`next` 메소드는 호출될 때마다 이터레이터의 상태를 변경(증가)시키므로, `&mut self`를 매개변수로 받는다.

### 2\. 이터레이터 생성하기

컬렉션에 대해 이터레이터를 생성하는 가장 일반적인 방법은 `iter()`, `into_iter()`, `iter_mut()` 메소드를 사용하는 것이다.

- **`iter()`:** 컬렉션의 요소에 대한 \*\*불변 참조(`&T`)\*\*를 반환하는 이터레이터를 생성한다. 원본 컬렉션의 소유권을 이동시키지 않으므로, 이터레이션 후에도 컬렉션을 계속 사용할 수 있다.

  ```rust
  let v = vec![1, 2, 3];
  let mut v_iter = v.iter(); // v의 불변 참조를 순회하는 이터레이터 생성

  println!("{:?}", v_iter.next()); // 출력: Some(1)
  println!("{:?}", v_iter.next()); // 출력: Some(2)
  println!("{:?}", v_iter.next()); // 출력: Some(3)
  println!("{:?}", v_iter.next()); // 출력: None (더 이상 요소가 없다)
  ```

- **`into_iter()`:** 컬렉션의 요소에 대한 \*\*소유권(`T`)\*\*을 이동시키는 이터레이터를 생성한다. 이터레이션 후에는 원본 컬렉션을 더 이상 사용할 수 없다.

  ```rust
  let v = vec![1, 2, 3];
  let mut v_into_iter = v.into_iter(); // v의 소유권을 이동시키는 이터레이터 생성

  println!("{:?}", v_into_iter.next()); // 출력: Some(1)
  // println!("{:?}", v); // 에러! v의 소유권이 이동되었다.
  ```

- **`iter_mut()`:** 컬렉션의 요소에 대한 \*\*가변 참조(`&mut T`)\*\*를 반환하는 이터레이터를 생성한다. 이터레이션 중에 요소를 변경할 수 있다.

  ```rust
  let mut v = vec![1, 2, 3];
  let mut v_mut_iter = v.iter_mut(); // v의 가변 참조를 순회하는 이터레이터 생성

  if let Some(x) = v_mut_iter.next() {
      *x += 10; // 역참조하여 값 변경
  }
  println!("{:?}", v); // 출력: [11, 2, 3]
  ```

### 3\. 이터레이터 소비 메소드(Consuming Adaptors)

`next()` 메소드를 직접 호출하는 것 외에도, 이터레이터를 소비하고 단일 값이나 새로운 컬렉션을 반환하는 다양한 메소드들이 있다.

- **`sum()`:** 이터레이터의 모든 요소를 합산한다.

  ```rust
  let v = vec![1, 2, 3];
  let total: i32 = v.iter().sum(); // 불변 참조를 순회하며 합산
  println!("Total: {}", total); // 출력: Total: 6
  ```

- **`collect()`:** 이터레이터의 요소들을 새로운 컬렉션 타입으로 수집한다.

  ```rust
  let v: Vec<i32> = vec![1, 2, 3];
  let doubled_v: Vec<i32> = v.iter().map(|x| x * 2).collect(); // 각 요소를 2배 한 후 Vec으로 수집
  println!("{:?}", doubled_v); // 출력: [2, 4, 6]
  ```

### 4\. 이터레이터 어댑터(Iterator Adaptors)

이터레이터 어댑터는 이터레이터를 소비하지 않고, 다른 종류의 이터레이터를 반환하는 메소드다. 이는 이터레이터 체이닝(chaining)을 가능하게 하여 복잡한 데이터 변환을 간결하게 표현할 수 있다.

- **`map()`:** 이터레이터의 각 요소에 클로저를 적용하여 새로운 이터레이터를 생성한다.

  ```rust
  let v = vec![1, 2, 3];
  let mapped_iter = v.iter().map(|x| x + 1); // 새로운 이터레이터 (아직 값은 생성되지 않았다)

  // mapped_iter는 지연(lazy)적이므로, collect()와 같은 소비 메소드가 호출될 때까지 실제 계산은 일어나지 않는다.
  let collected: Vec<_> = mapped_iter.collect();
  println!("{:?}", collected); // 출력: [2, 3, 4]
  ```

- **`filter()`:** 클로저의 결과가 `true`인 요소만 포함하는 새로운 이터레이터를 생성한다.

  ```rust
  let v = vec![1, 2, 3, 4, 5];
  let filtered_iter = v.iter().filter(|&x| x % 2 == 0); // 짝수만 걸러내는 이터레이터

  let collected: Vec<_> = filtered_iter.collect();
  println!("{:?}", collected); // 출력: [2, 4]
  ```

---

## Rust 이터레이터의 언어 철학

Rust의 이터레이터 시스템은 다음 언어 철학을 강력하게 반영한다:

1.  **성능(Performance)과 효율성(Efficiency):**

    - **"제로 코스트 추상화"**: 이터레이터는 고수준의 추상화를 제공하지만, 런타임 오버헤드가 거의 없다. Rust 컴파일러는 이터레이터 체이닝을 `for` 루프와 같은 수동 최적화 코드와 거의 동일한 성능을 내도록 인라인(inlining)하고 최적화한다.
    - **지연(Lazy) 동작:** 이터레이터 어댑터는 소비 메소드가 호출될 때까지 실제 계산을 지연시킨다. 이는 불필요한 중간 컬렉션 생성을 피하여 메모리 할당과 CPU 주기를 절약한다.

2.  **안전성(Safety)과 불변성(Immutability) 우선:**

    - `iter()`, `iter_mut()`, `into_iter()`를 통해 컬렉션 요소를 순회하는 방식을 명확하게 구분한다. 이는 소유권(Ownership) 및 빌림(Borrowing) 규칙과 완벽하게 통합되어, 컬렉션을 안전하게 순회하고 변경할 수 있도록 보장한다.
    - `&T`, `&mut T`, `T` 중 어떤 것을 사용할지 명시적으로 선택함으로써, 개발자가 의도한 데이터 접근 방식을 컴파일 타임에 강제한다.

3.  **유연성과 표현력(Flexibility and Expressiveness):**

    - 이터레이터 어댑터와 클로저의 조합은 복잡한 데이터 변환 및 필터링 작업을 매우 간결하고 읽기 쉬운 방식으로 표현할 수 있게 한다. 이는 코드의 가독성과 유지보수성을 높인다.
    - 다양한 컬렉션 타입(`Vec`, `HashMap` 등)이 `Iterator` 트레이트를 구현하여 일관된 순회 인터페이스를 제공한다.

4.  **함수형 프로그래밍 스타일:**

    - `map`, `filter`, `fold`와 같은 메소드들은 함수형 프로그래밍 패러다임의 영향을 받았다. 이는 데이터를 불변적으로 처리하고, 변환 파이프라인을 구축하는 데 유용하다.

결론적으로 Rust의 이터레이터는 강력한 성능과 안전성을 제공하면서도, 데이터를 다루는 작업을 매우 유연하고 표현력 있게 만들 수 있도록 돕는 핵심적인 기능이다.

---
