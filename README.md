# Rust
## What is Rust
 is a general-purpose programming language emphasizing performance, type safety, and concurrency.

## Rust Installation 
Click [here](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://www.rust-lang.org/tools/install&ved=2ahUKEwji_5OW0eCIAxX2VvEDHX1-AM8QFnoECBUQAQ&usg=AOvVaw3Icgu945TtBSmUIPVgdOzY) for installation across different operating system

## How to create a Project
- Create a project `cargo new --bin 'project-name'`
- Version checking `rustc --version`
- Initializing a directory `cargo init`
### Compiling Rust file
- Compile rust file to be executable `rustc main.rs`

## Data Types
- Boolean `bool`
- Character `char`
- Unsigned Integer `u8`, `u16`, `u32`, `u64`
- Signed Integer `i8`, `i16`, `i32`,`i64`
- Float-points `f32`, `f64`
- String `&str`

## Stack and Heap
 - **Stack** - suitable for fixed-size data with predictable lifetimes, while **heap** is ideal for dynamic data with uncertain lifetimes.
#### Pointer type
- **usize**
- **isize**

## Operators & Conditions
- *Operator* -> `+`, `-`, `*`, `/`, `%`, `-=`, `+=`, `*=`
- *Condition* -> `<`, `>`, `==`, `<=`, `>=`

## Conditional statements
- `if else` statement
- `else if` statement

## Loops
- `while` loop
- `for` loop

## Match statements
- `match` statement

## Struct & Enumaration
- *Struct* 
```
struct Student {
    age:i32,
    mark:i32,
}
```

- *enum*
```
#[derive(Debug)]
enum Language {
    Java,
    Scala,
    Rust,
    C,
    Swift,
}
```