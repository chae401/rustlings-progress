# Tests

Going out of order from the book to cover tests -- many of the following exercises will ask you to make tests pass!

## Further information

- [Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

---

# Tests

Rust에서 \*\*테스트(Tests)\*\*는 코드가 예상대로 정확하게 작동하는지 검증하는 데 필수적인 부분이다. 많은 다음 연습 문제들이 테스트를 통과하도록 요구할 것이다\! Rust는 테스트를 작성하고 실행하기 위한 강력한 내장 기능을 제공한다.

---

## Rust 테스트의 기본 문법과 동작

Rust의 테스트는 실제 코드를 실행하여 그 결과가 예상과 일치하는지 확인하는 작은 코드 조각이다. `cargo test` 명령어를 사용하여 모든 테스트를 쉽게 실행할 수 있다.

### 1\. 테스트 함수 정의하기

테스트 함수는 일반 함수와 유사하게 정의하지만, 몇 가지 특징을 가진다.

- **`#[test]` 속성:** 함수 위에 `#[test]` 속성을 추가하여 해당 함수가 테스트임을 Rust에게 알려준다.
- **테스트 모듈:** 테스트 함수는 보통 **테스트 모듈** 안에 정의된다. 이 모듈은 일반적으로 `src/lib.rs`나 `src/main.rs` 파일의 끝에 `#[cfg(test)]` 속성을 가진 `mod tests` 블록으로 작성된다.
  - `#[cfg(test)]`는 이 모듈이 `cargo test` 명령으로만 컴파일되고 실행되며, 일반적인 `cargo build` 시에는 무시되도록 지시한다.
- **가시성:** 테스트 모듈 내의 테스트 함수는 일반적으로 비공개(private) 함수를 테스트하기 위해 필요하다면 해당 함수에 접근할 수 있도록 바깥 스코프의 항목을 `use super::*`로 가져온다.

<!-- end list -->

```rust
// src/lib.rs 또는 src/main.rs

// add 함수는 테스트 대상이 된다.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 이 모듈은 오직 `cargo test` 명령으로 컴파일된다.
#[cfg(test)] // 조건부 컴파일 속성
mod tests {
    use super::*; // 부모 모듈(현재 파일)의 모든 항목을 가져온다.
                 // 이를 통해 `add` 함수와 같은 외부 항목에 접근할 수 있다.

    #[test] // 이 함수는 테스트임을 나타낸다.
    fn it_works() { // 테스트 함수의 이름은 무엇이든 될 수 있다.
        let result = add(2, 2);
        assert_eq!(result, 4); // `assert_eq!` 매크로를 사용하여 예상 결과를 검증한다.
    }

    #[test]
    fn another_test() {
        assert!(true); // `assert!` 매크로를 사용하여 조건이 참인지 검증한다.
    }

    #[test]
    #[should_panic] // 이 테스트는 패닉이 발생해야 성공한다.
    fn panics_test() {
        panic!("This test should panic!"); // 의도적으로 패닉을 발생시킨다.
    }

    #[test]
    #[ignore = "reason for ignoring"] // 이 테스트는 기본적으로 실행되지 않는다.
    fn expensive_test() {
        // 이 테스트는 시간이 오래 걸리거나 특정 환경에서만 실행되어야 할 때 사용한다.
        // `cargo test -- --ignored`로 실행할 수 있다.
    }
}
```

### 2\. 테스트 매크로 (`assert!`, `assert_eq!`, `assert_ne!`)

Rust는 테스트에서 결과를 검증하기 위한 편리한 매크로들을 제공한다.

- **`assert!(condition)`:** `condition`이 `true`로 평가되는지 확인한다. `false`일 경우 테스트 실패.
- **`assert_eq!(left, right)`:** `left`와 `right`가 동일한지 확인한다. 동일하지 않을 경우 테스트 실패. (디버그 정보 출력)
- **`assert_ne!(left, right)`:** `left`와 `right`가 동일하지 않은지 확인한다. 동일할 경우 테스트 실패. (디버그 정보 출력)

이 매크로들은 실패 시 패닉을 발생시킨다.

### 3\. 테스트 실행하기

터미널에서 `cargo test` 명령어를 사용하여 프로젝트의 모든 테스트를 실행할 수 있다.

```bash
cargo test
```

- **특정 테스트 실행:** `cargo test [테스트_함수_이름]`
  ```bash
  cargo test it_works
  ```
- **이름 일부로 테스트 실행:** `cargo test [포함될_문자열]`
  ```bash
  cargo test work # 'it_works' 테스트가 포함된다.
  ```
- **무시된 테스트만 실행:** `cargo test -- --ignored`
- **모든 테스트 결과 출력(성공/실패 여부와 관계없이):** `cargo test -- --show-output`

---

## Rust 테스트의 언어 철학

Rust의 테스트 시스템은 다음과 같은 핵심 언어 철학을 반영한다:

1.  **안전성(Safety)과 견고성(Robustness) 보장:**

    - 테스트는 Rust의 컴파일 타임 안전성 외에 **런타임 동작의 정확성**을 검증하는 중요한 수단이다. 개발자가 코드가 예상대로 작동하는지 지속적으로 확인할 수 있도록 하여 버그를 줄이고 프로그램의 신뢰성을 높인다.
    - 특히 `#[should_panic]`과 같은 속성은 의도적인 실패 시나리오도 테스트하여 코드의 견고성을 확인한다.

2.  **쉬운 테스트 작성 및 통합:**

    - 테스트 프레임워크가 언어와 빌드 시스템(`cargo`)에 내장되어 있어 별도의 외부 도구나 복잡한 설정 없이도 테스트를 쉽게 작성하고 실행할 수 있다. 이는 테스트 주도 개발(TDD)과 지속적인 통합(CI)을 장려한다.
    - `#[cfg(test)]`를 통해 테스트 코드가 제품 빌드에 포함되지 않도록 하여, 최종 바이너리의 크기와 성능에 영향을 주지 않는다.

3.  **명확한 피드백과 디버깅:**

    - `assert_eq!`와 `assert_ne!` 매크로는 실패 시 비교 대상 값들을 자세히 출력해주므로, 무엇이 잘못되었는지 빠르게 파악하고 디버깅하는 데 도움이 된다.
    - 테스트 결과는 명확하게 성공/실패 여부를 보여주어 개발자가 문제점을 즉시 인지하고 수정할 수 있게 한다.

4.  **문서화 및 예제로서의 역할:**

    - 잘 작성된 테스트 코드는 해당 기능이 어떻게 사용되는지에 대한 훌륭한 예제이자 살아있는 문서 역할을 한다. 다른 개발자가 코드를 이해하고 사용하는 데 도움이 된다.

결론적으로 Rust의 테스트 시스템은 개발자가 고품질의 안정적인 소프트웨어를 효율적으로 구축할 수 있도록 강력하고 통합된 지원을 제공하며, 이는 Rust의 실용주의적이고 안전 지향적인 철학의 중요한 부분이다.

---
