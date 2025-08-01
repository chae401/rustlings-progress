# Intro

Rust uses the `print!` and `println!` macros to print text to the console.

## Further information

- [Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)

---

## Rust의 `print!` / `println!`과 매크로 개념 정리

### 1. `print!`, `println!`이란?

- **`print!`**: 콘솔에 문자열 출력 (줄바꿈 없음)
- **`println!`**: 콘솔에 문자열 출력 + 줄바꿈(`\n`)

```rust
print!("Hello, ");
println!("world!");
// 출력: Hello, world!
```

---

### 2. 이게 왜 **매크로**일까? 함수가 아니고?

#### ✅ 매크로는 컴파일 타임에 \*\*코드를 생성(펼침)\*\*하는 도구

Rust의 `println!`은 매크로(`macro_rules!`)로 정의되어 있음. 이유는 다음과 같다:

| 이유                      | 설명                                                                 |
| ------------------------- | -------------------------------------------------------------------- |
| **가변 인자 지원**        | `println!("x = {}", x)`처럼 인자 수가 고정되지 않음                  |
| **타입 제한 없음**        | 어떤 타입이든 `{}`에 넣을 수 있음 (`Display`, `Debug` 트레이트 사용) |
| **컴파일 타임 오류 검출** | 포맷 문자열과 인자 수/type이 맞지 않으면 컴파일 오류 발생            |
| **고성능**                | 필요한 코드만 컴파일 타임에 생성 → 불필요한 런타임 비용 없음         |

---

### 3. 내부 동작 방식

- `println!`은 내부적으로 `format_args!` 매크로를 통해 포맷된 문자열을 생성하고,
- `std::io::stdout()`에 출력해 줌.

```rust
println!("x = {}", x);
// 실제로는 ↓ 이런 식으로 동작
std::io::stdout().write_fmt(format_args!("x = {}", x)).unwrap();
```

---

### 4. 포맷 지정 예시

```rust
println!("{} is {}", "Rust", 18);                       // 일반 출력
println!("{0}, {1}, {0}", "Elly", "Kim");               // 위치 지정
println!("{name} is {age}", name="Elly", age=25);       // 이름 지정
println!("{:?}", vec![1, 2, 3]);                         // 디버그 출력
println!("{:^6}", "Hi");                                // 가운데 정렬
println!("{:.2}", 3.141592);                            // 소수점 제한
```

---

### 5. 매크로의 단점

| 단점          | 이유                                        |
| ------------- | ------------------------------------------- |
| 디버깅 어려움 | 펼쳐진 코드가 많아 에러 위치 파악이 어려움  |
| IDE 지원 부족 | 일부 IDE가 매크로 내부 코드를 분석하지 못함 |

---

### 📘 추가 참고

- [Rust by Example: Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)
- [The Rust Book - Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [`println!` 공식 문서](https://doc.rust-lang.org/std/macro.println.html)
