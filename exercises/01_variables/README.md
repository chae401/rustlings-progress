# Variables

In Rust, variables are immutable by default.
When a variable is immutable, once a value is bound to a name, you can’t change that value.
You can make them mutable by adding `mut` in front of the variable name.

## Further information

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## Rust에서 '변수'의 특징

### 1. Immutable by Default

Rust에선 변수를 선언하면 기본적으로 불변이다.

```
let x = 5;
x = 6; // ❌ 컴파일 에러!
```

#### 장점

- 동시성 프로그래밍에서 데이터 변경은 리스크가 크다.
- 따라서 변경을 명시적으로 하지 않는 한, 값은 졀대 안 바뀐다는 보장을 주는게 안전하다.
- 이로 인해 Rust의 버그율은 훨씬 낮아진다.

#### 왜 기본이 immutable일까?

C, Python, Java에서 변수는 기본적으로 mutable이다보니,
-> "이 값이 왜 바뀌었지?"
-> "멀티스레드에서 왜 값이 꼬였지?" 같은 디버깅 악몽이 생김.

Rust의 Soltuion

- "우린 개발자에게 안전을 강요하겠다"
- 그래서 불변이 기본값
- 값이 바뀌면 추론이 어려워지고, 추론이 어려워지면 버그가 줄어든다.

### 2. 가변이어야만 한다면 `mut`를 사용하라

- mut: 나는 이 값을 변경할 의도가 있다

```rust
let mut x = 5;
x = 6; // ✅ OK!
```

### 3. `const` vs `let`

- `let`은 런타임에 결정됨
- `const`는 **컴파일 타임에 결정**되어야 하며, 타입 명시가 필수

```
const MAX_POINTS: u32 = 100_000;
```

### 4. Shadowing

- 기존 메모리 주소에 덮어쓰기 X
- 새로운 변수를 선언하는 것, 즉 새로운 메모리 할당
- 변수가 shadowing 되면 이전 변수는 더 이상 사용되지 않게 되고, 해당 스코프에서 "drop" 처리되어 메모리가 해제된다.
  - Rust는 RAII(Resource Acquisition Is Initialization) 원칙을 따르기 때문에, **변수의 생명주기(scope-based lifetime)**에 따라 자동으로 drop이 호출됨

```
let x = 5;
let x = x + 1;
let x = x.to_string();

println!("{}", x); // 출력: "6"
```

| 특징             | `mut`                         | Shadowing                                    |
| ---------------- | ----------------------------- | -------------------------------------------- |
| 메모리 재사용    | ✅ 같은 변수 메모리 위치 사용 | ❌ 새로운 변수 생성                          |
| 타입 변경 가능   | ❌ (컴파일 에러 발생)         | ✅ 타입 변경 가능                            |
| 상태 추적 용이성 | 더 어려움 (값이 계속 바뀜)    | 쉬움 (새 변수니까 불변처럼 추적 가능)        |
| 쓰임새           | 진짜 “값이 바뀌는 상태” 표현  | 값이 조금씩 가공되지만 원본 유지하고 싶을 때 |

### 5. Scope와 Drop

- Rust는 변수의 생명 주기를 스코프 단위로 관리한다.
- `let`로 선언된 변수는 해당 블록을 벗어나면 자동으로 메모리에서 해제된다.

```
{
    let name = "Elly"; // 여기서 생성됨
} // 여기서 drop됨
```

## Rust의 특징 정리

| 특징              | 의미                                                  |
| ----------------- | ----------------------------------------------------- |
| 기본 불변 (`let`) | 안전이 기본값. 실수로 값 변경하는 일 방지             |
| `mut`             | 의도된 변경을 명확하게 드러냄                         |
| `const`           | 컴파일 타임 상수. 타입 명시 필수                      |
| Shadowing         | 새로운 값을 기존 변수에 덮어쓰기, 타입 바꿀 수도 있음 |
| Scope-based drop  | 자동 메모리 관리 (RAII 철학 기반)                     |

## “C는 자유, Rust🦀는 자유와 책임.”

- 제어는 주되 안전이 기본값

- 명시적 의도 없인 위험한 일은 못함

- 성능과 안전을 동시에 추구

- 컴파일 타임에 가능한 한 모든 오류를 잡는다
