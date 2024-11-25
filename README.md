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

## Struct & Enumeration
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

## Vector(Vec\<T>\)
- allow you to store more than one value in a single data structure that puts all the values next to each other in memory
```
    let v = vec![1, 2, 3];
```

**Updating a vector**
```
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```
**Removing elements in an array**
```
    let v = vec![1, 2, 3];
    v.remove(0);
```
## String
**Slicing**

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection.
<br>
[image](images/r7bwet59.png)
<br>
```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

**Concatenation**

```
let hello = "Hello ".to_string();
let world = "world.".to_string();
let hello_world = hello + &world;
println!("{}", hello_world);
```

## Tuples 
 - general way of grouping together a number of values with a variety of types into one compound type.
 - Have a fixed length, they cannot grow or shrink.
 ```
 fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

## Generics(Polymorphism/Templates)
- Generic programming allows programmers to write general algorithms that work with arbitrary types.
```
struct Point {
    x:f32,
    y:f32,
}

let point: Point = Point { x: 0.3, y: 0.4 };
println!("point coordinates: ({}, {})", point.x, point.y);
```
## Ownership
- It enables Rust to make memory safety guarantees without needing a garbage collector.

**Ownership Rules**
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

**No Ownership Change**
```rust
pub fn main() {
    println!("-- No Ownership change --");

    let data = 5;
    let data2 = data;
    println!("Data: {}", data);
    println!("Data2: {}", data2);
    no_ownership_change(data);
}
```

**Ownership Change**
```rust
pub fn main() {
    println!("-- Ownership Change --");

    let v = vec![1,2,3];
    println!("Data: {}", v[0]);
    let v2 = v; /* Ownership movement*/
    println!("Data:{}", v2[0]);
    //println!("Data:{}", v[0]);
}
```