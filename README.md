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

## Variables
- **Assigned** using `let` keyword
- **Print** to standard output by `print()` or `println!()`
- **Scope** of a variable defined by the **block of code** in which it is declared.
- **Function** is a named block of code that is **reusable**
- **Shadowing** allows a variable to be **re-declared** in the same scope with the same name.

## Data Types
- Boolean `bool`
- Character `char`
- Number - Integer Types
  - Unsigned Integer -> Can represent both **positive** and **negative** integers
  - Signed Integer -> always positive

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-it | i28    | u128     |
| arch   | isize  | usize    |
      
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
  ```rust
  fn check_for_even_numbers(num:i32) {
    if (num % 2) == 0 {
      true
    } else { 
      false
    };
  }
  ```
- `else if` statement
  ```rust
  fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
  }
  ```

## Loops
- `Loop`
  ```rust
    fn main() {
      let mut count = 0u32;
  
      println!("Let's count until infinity!");
  
      // Infinite loop
      loop {
          count += 1;
  
          if count == 3 {
              println!("three");
  
              // Skip the rest of this iteration
              continue;
          }
  
          println!("{}", count);
  
          if count == 5 {
              println!("OK, that's enough");
  
              // Exit this loop
              break;
          }
      }
  }
  ```
  - `while` loop
    ```rust
    fn main() {
      // A counter variable
      let mut n = 1;

      // Loop while `n` is less than 101
      while n < 101 {
          if n % 15 == 0 {
              println!("fizzbuzz");
          } else if n % 3 == 0 {
              println!("fizz");
          } else if n % 5 == 0 {
              println!("buzz");
          } else {
              println!("{}", n);
          }

          // Increment counter
          n += 1;
      }
    } 
    ```
- `for` loop 

  - For and Range
  ```rust
  pub fn sum_of_even(start: i32, end: i32) -> i32 {
    if start >= end {
        0
    } else { 
        let sum = 0;
        for i in start..end {
            if i % 2 == 0 {
                sum += i;
            }
        } 
        sum
    }
  }
  ```
  - for and iterators

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
**Reference(`&`)** -> way to access without taking ownership of it.
- Immutable References
    - created using `&`.
    - Allow read-only access to a resource.
    - Can e created multiple times.
    ```rust
    fn main() {
        let vec = vec![10,11];
        for i in &vec {
            println!("{}", i)
        }
      }
  ```
- Mutable Reference
    - Created using `&mut` symbol.
    - Allow read & write access 
    - There can only be one mutable reference to a resource to any given time.
    ```rust
    fn main() {
      let mut vec = vec![10,11];
        let first = &mut vec[0];
        *first = 6; //dereferencing
        println!("{:?}", vec);
  }
  ```

**Rules of borrowing**
- Each reference can only have one mutable reference.
- Reference must always be valid.
- A mutable reference cannot exist at the same time as any other reference.

## Crates
- Crate is a compilation unit.
 - creating crate
```commandline
cargo new my-new-crate --lib
```
## HashMap
- Where vectors store values by an integer index, `HashMap` S store values by key.
- `HashMap` keys can be booleans, integers, strings, or any other type that implements the `Eq` and `Hash` traits.

**Requirements of HashMap key**
Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`. This includes:

- `bool` (though not very useful since there is only two possible keys)
- `int`, `uint`, and all variations thereof 
- `String` and `&str` (tips: you can have a HashMap keyed by String and call .get() with an &str)

Note that f32 and f64 do not implement Hash, likely because floating-point precision errors would make using them as hashmap keys horribly error-prone.

```rust
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

fn main() { 
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number."),
    }

    // `HashMap::insert()` returns `None`
    // if the inserted value is new, `Some(value)` otherwise
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley"); 

    // `HashMap::iter()` returns an iterator that yields 
    // (&'a key, &'a value) pairs in arbitrary order.
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number)); 
    }
}
```