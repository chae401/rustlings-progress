# Error handling

Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

## Further information

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)

---

# Error handling

대부분의 오류는 프로그램이 완전히 멈춰야 할 만큼 심각하지 않다. 때로는 함수가 실패하더라도, 그 원인을 쉽게 해석하고 대응할 수 있는 경우가 있다. 예를 들어, 파일을 열려고 시도했는데 파일이 존재하지 않아 작업이 실패한다면, 프로세스를 종료하는 대신 파일을 생성하고 싶을 수 있다.

---

## Rust 오류 처리의 기본 개념과 문법

Rust는 오류를 크게 두 가지 범주로 나눈다: **회복 불가능한(unrecoverable) 오류**와 **회복 가능한(recoverable) 오류**다. 회복 불가능한 오류는 `panic!` 매크로를 통해 처리되어 프로그램이 종료된다. 반면, 회복 가능한 오류는 `Result<T, E>` 열거형을 사용하여 처리되며, 프로그램이 계속 실행될 수 있도록 한다.

### 1\. `Result<T, E>` 열거형

`Result<T, E>`는 Rust에서 회복 가능한 오류를 다루는 표준적인 방법이다. 이 열거형은 두 가지 배리언트를 가진다:

- **`Ok(T)`:** 작업이 성공했음을 나타내며, 성공 시의 값 `T`를 포함한다.
- **`Err(E)`:** 작업이 실패했음을 나타내며, 실패의 원인인 오류 값 `E`를 포함한다.

<!-- end list -->

```rust
enum Result<T, E> {
    Ok(T), // T는 성공 시 반환될 값의 타입이다.
    Err(E), // E는 실패 시 반환될 오류 값의 타입이다.
}
```

### 2\. `Result` 값 다루기 (`match` 문)

`Result` 값을 다루는 가장 일반적인 방법은 `match` 표현식을 사용하는 것이다. `match`를 사용하면 `Ok`와 `Err`의 두 가지 가능한 경우를 모두 명시적으로 처리하도록 강제한다.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt"); // 파일을 연다.

    let greeting_file = match greeting_file_result {
        Ok(file) => file, // 파일 열기 성공 시 File 인스턴스를 가져온다.
        Err(error) => match error.kind() { // 파일 열기 실패 시 오류 종류를 확인한다.
            ErrorKind::NotFound => { // 파일이 존재하지 않으면
                match File::create("hello.txt") { // 파일을 새로 생성하려고 시도한다.
                    Ok(fc) => fc, // 파일 생성 성공 시 File 인스턴스를 가져온다.
                    Err(e) => panic!("Problem creating the file: {:?}", e), // 파일 생성 실패 시 패닉한다.
                }
            }
            other_error => { // 그 외의 다른 오류인 경우
                panic!("Problem opening the file: {:?}", other_error); // 패닉한다.
            }
        },
    };
    // greeting_file을 사용하여 파일 작업을 계속한다.
}
```

### 3\. 실패 시 패닉하는 지름길 (`unwrap`, `expect`)

`Option` 타입과 유사하게, `Result` 타입도 `unwrap()`과 `expect()` 메소드를 제공한다. 이들은 `Ok` 값을 편리하게 가져오지만, `Err`일 경우 **패닉**을 일으켜 프로그램을 종료시킨다.

- **`unwrap()`:** `Ok(value)`일 경우 `value`를 반환하고, `Err`일 경우 패닉을 일으킨다.
- **`expect("error message")`:** `unwrap()`과 동일하게 동작하지만, `Err`일 때 사용자 정의 패닉 메시지를 지정할 수 있어 디버깅에 유용하다.

이 메소드들은 테스트 코드에서나, `Err` 케이스가 절대 발생하지 않을 것이라고 **강력하게 확신할 때**만 사용해야 한다.

```rust
use std::fs::File;

fn main() {
    // let greeting_file = File::open("hello.txt").unwrap(); // 파일이 없으면 패닉한다.
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt 파일을 열 수 없었습니다."); // 사용자 정의 메시지와 함께 패닉한다.
}
```

### 4\. 오류 전파하기 (`?` 연산자)

함수가 `Result`를 반환하고, 해당 함수 내부에서 또 다른 `Result`를 반환하는 함수를 호출할 때, `match` 문을 일일이 작성하는 대신 `?` (물음표) 연산자를 사용하여 오류를 더 간결하게 전파할 수 있다.

- `?` 연산자는 `Result` 값이 `Ok`이면 `Ok` 안의 값을 추출하여 반환한다.
- `Result` 값이 `Err`이면, `?` 연산자는 그 `Err` 값을 즉시 호출하는 함수로부터 **반환**한다.

`?` 연산자는 `Result`를 반환하는 함수 내에서만 사용할 수 있다.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // File::open이 Err를 반환하면 즉시 이 Err를 반환한다.
                                                      // Ok이면 File 인스턴스를 username_file에 할당한다.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // read_to_string이 Err를 반환하면 즉시 이 Err를 반환한다.
                                                 // Ok이면 값을 읽어온다.
    Ok(username) // 모든 작업이 성공하면 Ok(username)을 반환한다.
}

fn main() {
    match read_username_from_file() {
        Ok(s) => println!("Username: {}", s),
        Err(e) => println!("Error reading username: {:?}", e),
    }
}
```

위 예시에서 `?`는 `match` 문을 간결하게 줄여준다.

### 5\. 오류 박싱 (Boxing Errors)

함수가 여러 종류의 오류(다른 `E` 타입)를 반환할 수 있는 경우, `Box<dyn std::error::Error>`와 같은 \*\*트레이트 객체(trait object)\*\*를 사용하여 모든 오류를 하나의 타입으로 박싱(`Box::new`)하여 처리할 수 있다. 이는 특히 라이브러리를 작성할 때 유용하다.

```rust
use std::fmt;

// 사용자 정의 오류 타입 정의
#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "My custom error occurred!")
    }
}

impl std::error::Error for MyError {} // std::error::Error 트레이트 구현

fn do_something_risky(choice: i32) -> Result<(), Box<dyn std::error::Error>> {
    if choice == 1 {
        // String::from_utf8_lossy("bad_utf8_bytes").to_string(); // 실제 유효하지 않은 UTF-8 바이트 시도
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "IO problem!",
        ))) // io::Error를 Box로 감싸 반환
    } else if choice == 2 {
        Err(Box::new(MyError)) // 사용자 정의 오류를 Box로 감싸 반환
    } else {
        Ok(())
    }
}

fn main() {
    if let Err(e) = do_something_risky(1) {
        println!("Error: {}", e); // 출력: Error: IO problem!
    }
    if let Err(e) = do_something_risky(2) {
        println!("Error: {}", e); // 출력: Error: My custom error occurred!
    }
}
```

---

## Rust 오류 처리의 언어 철학

Rust의 오류 처리 방식은 다음 언어 철학을 강력하게 반영한다:

1.  **명시적인 오류 처리(Explicit Error Handling):**

    - Rust는 `Result` 타입을 통해 회복 가능한 오류를 명시적으로 반환하도록 강제한다. 이는 개발자가 오류가 발생할 가능성을 무시할 수 없도록 하여, **숨겨진 버그를 줄이고 프로그램의 안정성을 높인다.** 다른 언어의 예외(exceptions)처럼 오류를 "던져서(throw)" 처리하는 방식과 달리, Rust는 오류도 일반적인 데이터처럼 "반환"하여 타입 시스템의 검증 대상이 되도록 한다.

2.  **회복 가능성 강조:**

    - 대부분의 오류를 `panic!`으로 프로그램 종료를 강제하기보다, `Result`를 통해 프로그램이 오류를 감지하고 대응하여 계속 실행될 수 있도록 설계되었다. 이는 견고하고 오래 실행되는 애플리케이션에 필수적이다.

3.  **"제로 코스트 추상화"**:

    - `Result` 열거형은 런타임 오버헤드가 거의 없다. `Ok` 또는 `Err` 내부에 값과 오류 정보를 그대로 저장하며, 가비지 컬렉터와 같은 추가적인 런타임 비용이 발생하지 않는다. `match` 문이나 `?` 연산자도 컴파일 타임에 효율적인 기계어 코드로 변환된다.

4.  **강력한 타입 시스템과의 통합:**

    - `Result` 타입은 Rust의 강력한 타입 시스템과 완벽하게 통합되어, 컴파일 타임에 `Result` 값을 제대로 처리했는지 검증한다. 이는 런타임에 발생할 수 있는 오류를 미리 잡아내어 개발자가 더 안전한 코드를 작성할 수 있도록 돕는다.

5.  **편리한 전파 및 처리:**

    - `?` 연산자는 오류 전파를 매우 간결하게 만들어, 개발자가 오류 처리 로직에 매몰되지 않고 핵심 비즈니스 로직에 집중할 수 있도록 돕는다. `unwrap`과 `expect`는 특정 상황에서 개발 편의성을 제공하지만, 사용에 대한 책임은 개발자에게 있다.

결론적으로 Rust의 오류 처리 방식은 안전성, 명시성, 그리고 성능이라는 Rust의 핵심 가치를 모두 만족시키며, 개발자가 신뢰할 수 있는 소프트웨어를 구축하는 데 필수적인 기반을 제공한다.

---
