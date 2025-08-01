# Primitive Types

Rust has a couple of basic types that are directly implemented into the
compiler. In this section, we'll go through the most important ones.

## Further information

- [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

---

## Scalar Types

### Integer

| 구분           | 타입    | 설명                              | 크기        |
| -------------- | ------- | --------------------------------- | ----------- |
| 부호 있는 정수 | `i8`    | 8비트 부호 있는 정수              | -128 \~ 127 |
|                | `i16`   | 16비트 부호 있는 정수             |             |
|                | `i32`   | 32비트 부호 있는 정수 (기본 타입) |             |
|                | `i64`   | 64비트 부호 있는 정수             |             |
|                | `i128`  | 128비트 부호 있는 정수            |             |
|                | `isize` | 포인터 크기 정수 (64비트/32비트)  |             |
| 부호 없는 정수 | `u8`    | 8비트 부호 없는 정수              | 0 \~ 255    |
|                | `u16`   | 16비트 부호 없는 정수             |             |
|                | `u32`   | 32비트 부호 없는 정수             |             |
|                | `u64`   | 64비트 부호 없는 정수             |             |
|                | `u128`  | 128비트 부호 없는 정수            |             |
|                | `usize` | 포인터 크기 부호 없는 정수        |             |

### Floating-Point

| 실수형 | `f32` | 32비트 부동소수점 실수 | 단정도 |
| | `f64` | 64비트 부동소수점 실수 (기본 타입) | 배정도 |

### Boolean

- true
- false

### char

- 유니코드 scalar 값 하나를 저장('a', '한')
- 4 byte 크기

## Compound Types

### Tuple

- 고정 크기, 다양한 타입 허용
- 선언:

  ```
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  ```

- 접근:
  ```
  let = x = tup.0;
  ```

### Array

- 같은 타입의 값들을 고정된 길이로 저장
- 선언:
  ```
  let arr = [1, 2, 3];
  ```
- 타입 명시:
  ```
  let arr: [i32; 5] = [1, 2, 3, 4, 5];
  ```
- 초기화:
  ```
  let zeros = [0; 5]; // 0이 5개
  ```

## Slice Type

- 배열이나 문자열의 일부분을 참조하는 타입

### 특징

- 슬라이스는 소유권을 갖지 않으며 참조 타입이다.
- 배열 일부를 다루고 싶을 때 사용된다.

```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // [2, 3]
```

### 참조 &

- Rust에서 `&`은 참조를 의미한다.
- C의 포인터 개념과 비슷하지만 훨씬 더 안전하고 엄격한 소유권 시스템 위에서 작동한다.
- `&`의 기본 의미 = 값을 빌려 온다.

```
let s = String::from("hello");
let r = &s;  // s를 참조(빌려옴)

```

- `r`은 `s`를 소유하지 않고, 읽기만 가능한 접근권을 갖는다.
- 이를 immutable reference라 한다.

#### `&T`: 불변 참조

```
fn greet(name: &String) {
    println!("Hello, {name}!");
}

```

- `&String`은 소유권을 넘기지 않고, `String`의 값을 읽기 전용으로 빌려옴
- 원본은 그대로 남아 있다.

#### `&mut T`: 가변 참조

```
fn change(name: &mut String) {
    name.push_str("!");
}

```
- `&mut`은 값을 수정 가능하게 빌려준다.
- 단, 가변 참조는 동시에 하나만 허용된다.(컴파일러가 검사)

## 참고

- 타입이 명확하지 않은 경우, Rust는 [타입 추론](https://rustc-dev-guide.rust-lang.org/type-inference.html)을 통해 자동으로 결정한다.
- 정수 오버플로우는 디버그 모드에서 panic을 발생시키며, 릴리즈 모드에서는 wrap-around 된다.
- 배열과 튜플은 고정된 크기를 가지며, 크기는 타입의 일부이다.
