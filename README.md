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

## Option<T>
- It has two variance
1. **None** ==> Absence of a value or failure of processing
```
let letter: Option<i32> = None;
```
2. **Some(v)** ==> a value with type "T"
```
let number = Some(7);
```

## Arrays
- Fixed list
- Another way to have a collection of multiple values 
- Must have the same type
- Separated by a comma
- Have fixed length
```
fn main() {
    let a: [i32; 6] = [1,2,3,4,5,6];
}
```

**Accessing Array Elements**
```
fn main() {
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];
}
```
**Iterating through arrays**
```
for i in a.iter() {
    println!("i = {}", i);
}
```
**Creating fixed arrays**
```
let array_two = [4;5];
```