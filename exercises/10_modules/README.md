# Modules

In this section we'll give you an introduction to Rust's module system.

## Further information

- [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

# Modules

Rust의 **모듈(Modules)** 시스템은 코드를 조직화하고, 가시성을 제어하며, 대규모 프로젝트를 관리하는 데 필수적인 기능이다. 파일을 계층적으로 구조화하고 코드 항목(함수, 구조체, 열거형 등)의 접근성을 지정하여, 복잡한 프로젝트를 더 쉽게 개발하고 유지보수할 수 있도록 돕는다.

---

## Rust 모듈 시스템의 기본 개념

Rust의 모듈 시스템은 **패키지(packages)**, **크레이트(crates)**, 그리고 \*\*모듈(modules)\*\*이라는 세 가지 주요 구성 요소로 이루어져 있다.

### 1\. 패키지(Packages)

패키지는 하나 이상의 **크레이트**와 `Cargo.toml` 파일을 묶는 단위다. `Cargo.toml` 파일은 프로젝트의 메타데이터와 의존성을 정의한다. `cargo new` 명령으로 프로젝트를 생성하면 기본적으로 패키지가 만들어진다.

- **역할:** 프로젝트를 구성하는 최상위 단위이자, `Cargo`를 통해 빌드, 테스트, 공유되는 단위이다.

### 2\. 크레이트(Crates)

크레이트는 Rust 컴파일러가 인식하는 가장 작은 컴파일 단위이다. 모든 Rust 프로젝트는 하나 또는 두 종류의 크레이트로 컴파일된다:

- **바이너리 크레이트(Binary Crate):** 실행 가능한 프로그램(바이너리)을 생성한다. `src/main.rs` 또는 `src/bin/*.rs` 파일이 바이너리 크레이트의 루트가 된다.

- **라이브러리 크레이트(Library Crate):** 다른 프로젝트에서 재사용할 수 있는 라이브러리를 생성한다. `src/lib.rs` 파일이 라이브러리 크레이트의 루트가 된다.

- **역할:** 코드를 묶어 하나의 컴파일 단위로 만든다. 모든 Rust 코드는 어떤 크레이트에 속하게 된다.

### 3\. 모듈(Modules)

모듈은 크레이트 내에서 코드를 조직화하는 단위다. 함수, 구조체, 열거형, 상수 등 모든 Rust 코드는 모듈 내에 정의된다. 모듈을 사용하여 코드를 계층적으로 그룹화하고 가시성(visibility)을 제어할 수 있다.

- **역할:** 크레이트 내의 코드를 논리적으로 분리하고, 접근 권한을 설정한다.

#### 3.1. 모듈 정의하기 (`mod` 키워드)

`mod` 키워드를 사용하여 모듈을 정의하고, 중괄호 `{}` 안에 모듈의 내용을 작성한다.

```rust
// src/main.rs (또는 src/lib.rs)

mod front_of_house { // front_of_house라는 모듈을 정의한다.
    mod hosting { // hosting이라는 중첩 모듈을 정의한다.
        fn add_to_waitlist() {} // 함수는 기본적으로 private이다.
    }

    mod serving { // serving이라는 중첩 모듈을 정의한다.
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```

#### 3.2. 파일로 분리하기

큰 모듈은 별도의 파일로 분리하여 관리할 수 있다. `mod` 선언 뒤에 내용을 직접 작성하는 대신 세미콜론(;)을 붙이면, Rust는 같은 이름의 `.rs` 파일이나 디렉토리를 찾아 그 내용을 모듈로 포함시킨다.

- `src/main.rs` 또는 `src/lib.rs`에 `mod some_module;`이라고 선언하면:
  - Rust는 `src/some_module.rs` 파일을 찾거나,
  - `src/some_module/mod.rs` 파일을 찾아 모듈 내용으로 사용한다.

<!-- end list -->

```rust
// src/main.rs
mod front_of_house; // front_of_house 모듈을 src/front_of_house.rs에서 찾는다.

fn main() {
    // ...
}

// src/front_of_house.rs
pub mod hosting { // pub을 사용하여 외부에서 접근 가능하게 한다.
    pub fn add_to_waitlist() {} // pub을 사용하여 함수를 외부에서 호출 가능하게 한다.
}
```

#### 3.3. 가시성(Visibility) 제어 (`pub` 키워드)

Rust의 모든 항목(함수, 구조체, 열거형, 모듈 등)은 기본적으로 \*\*비공개(private)\*\*이다. 즉, 정의된 모듈 내부에서만 접근할 수 있다. 다른 모듈에서 접근할 수 있도록 하려면 `pub` 키워드를 사용하여 \*\*공개(public)\*\*로 선언해야 한다.

- **`pub fn`:** 공개 함수
- **`pub struct`:** 공개 구조체 (기본적으로 필드는 비공개)
  - 구조체의 필드를 개별적으로 공개하려면 `pub field_name: Type,`처럼 필드에도 `pub`을 붙여야 한다.
- **`pub enum`:** 공개 열거형 (모든 배리언트는 자동으로 공개)

<!-- end list -->

```rust
mod my_module {
    pub fn public_function() { // 이 함수는 외부에서 접근 가능하다.
        println!("This is a public function.");
    }

    fn private_function() { // 이 함수는 my_module 내부에서만 접근 가능하다.
        println!("This is a private function.");
    }

    pub struct PublicStruct {
        pub field1: i32, // 이 필드는 외부에서 접근 가능하다.
        field2: String, // 이 필드는 my_module 내부에서만 접근 가능하다.
    }

    pub enum PublicEnum { // 이 열거형과 그 배리언트들은 모두 공개이다.
        VariantA,
        VariantB,
    }
}

fn main() {
    my_module::public_function(); // 공개 함수 호출 가능
    // my_module::private_function(); // 에러: private 함수는 접근 불가

    let public_instance = my_module::PublicStruct {
        field1: 10,
        field2: String::from("private data"), // private 필드이므로, PublicStruct를 생성하는 main 함수에서는 접근 가능하지만,
                                            // 다른 외부 모듈에서는 접근 불가능하다.
    };
    println!("{}", public_instance.field1); // public 필드 접근 가능
    // println!("{}", public_instance.field2); // 에러: private 필드는 직접 접근 불가

    let enum_variant = my_module::PublicEnum::VariantA; // 공개 열거형 배리언트 접근 가능
}
```

#### 3.4. `use` 키워드를 사용하여 경로 가져오기

`use` 키워드는 긴 경로를 반복해서 작성하는 대신, 특정 항목을 현재 스코프로 가져와 짧은 이름으로 사용할 수 있게 한다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting; // hosting 모듈을 현재 스코프로 가져온다.

fn main() {
    hosting::add_to_waitlist(); // 이제 front_of_house::hosting::add_to_waitlist() 대신 hosting::add_to_waitlist()를 사용할 수 있다.
}
```

- `use self::*` (glob import): 특정 경로의 모든 공개 항목을 가져온다.
- `use self::Path as NewName`: 항목의 이름을 변경하여 가져온다.

---

## Rust 모듈 시스템의 언어 철학

Rust의 모듈 시스템은 다음과 같은 핵심 철학을 반영한다:

1.  **조직화와 확장성(Organization and Scalability):**

    - 복잡한 프로젝트를 작은 단위로 분할하여 관리할 수 있게 한다. 이는 코드의 가독성을 높이고, 여러 개발자가 동시에 작업할 때 충돌을 최소화하며, 재사용 가능한 라이브러리 개발을 용이하게 한다.
    - 이는 대규모 소프트웨어 개발의 핵심 요소이다.

2.  **캡슐화와 정보 은닉(Encapsulation and Information Hiding):**

    - 기본적으로 모든 항목이 비공개(private)인 것은 Rust의 **캡슐화** 원칙을 강력하게 반영한다. 모듈 외부에서는 공개(public)로 명시된 항목에만 접근할 수 있다.
    - 이를 통해 모듈의 내부 구현 세부 사항을 숨기고, 모듈의 공용 API만 노출하여 예측 가능한 동작과 쉬운 유지보수를 가능하게 한다. 내부 구현이 변경되어도 외부 코드에 영향을 주지 않도록 한다.

3.  **명시적인 의존성 관리:**

    - `use` 키워드를 통해 어떤 항목을 가져오는지 명시적으로 선언한다. 이는 코드의 의존성을 명확하게 보여주고, 예상치 못한 이름 충돌을 방지하는 데 도움을 준다.
    - `Cargo.toml`을 통한 패키지 및 크레이트 의존성 관리는 프로젝트 전체의 빌드와 배포를 체계적으로 만든다.

4.  **안전성(Safety) 및 컴파일 타임 검증:**

    - 모듈 시스템은 컴파일러의 강력한 정적 분석과 결합하여, 유효하지 않은 접근이나 모듈 경계를 벗어난 사용을 컴파일 타임에 잡아낸다. 이는 런타임 오류를 줄이고 안정적인 소프트웨어를 만드는 데 기여한다.

결론적으로 Rust의 모듈 시스템은 단순히 코드를 정리하는 도구를 넘어, Rust가 대규모 프로젝트에서 안전하고 효율적이며 관리하기 쉬운 코드를 작성하도록 돕는 핵심적인 구조적 기반을 제공한다.
