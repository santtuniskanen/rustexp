# Common programming concepts
This page goes over basic concepts and how they work in Rust. Learning about these concepts, like functions and control flow is going to be an important foundation not only for learning Rust but other languages too.
&nbsp;
## Variables and mutability
Earlier in the guessing game we went over what the `mut` keyword does in Rust. It allows us to mutate variables, since they are immutable by default in Rust. For example, if we ran this block of code, the program would throw an error and suggest we allow mutability.
``` Rust
    let x = 5;
    println!("x equals {x}");
    x = 6;
    print!("now x equals {x}");
```
We would get this error
``` Rust
 cargo run
   Compiling variables v0.1.0 (/[filepath])
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("x equals {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```
Luckily, allowing mutability is fairly easy. Instead of using `let`, we use `let mut`. Now the program would be able to compile after we try changing the value of the variable.
``` Rust
x equals 5
now x equals 6
```
&nbsp;
### Constants
Constants, `const`, are values that are bound to a name and aren't allowed to change. They are however slightly different than variables. While we are allowed to add mutability to variables when using let, constants are `always` immutable. The type of the value must be annotated. Here's an example of a constant declaration:
``` Rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
Notice how the name consists of uppercase letters with underscores? A constants type and value can not change during the programs runtime once it's defined. It's essentially hard coding a value.
``` Rust
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
    // Would print 10800
```
&nbsp;
### Shadowing
We can shadow a variable by using its name and repeating the use of `let` keyword.
``` Rust
let x = 5;
let x = x + 1;
```
Now the value of `x` would be `6`. But how, since we didn't allow mutability? The reason why is because we are essentially creating a new variable, shown by the use of `let`. If we didn't use that keyword, we would get compile errors. This way we can reuse the name of the variable. <br>
An example where this is useful is when checking the length of a string and returning an integer.
``` Rust
let spaces = "   ";
let spaces = spaces.len();
```
Initially we can see that the variable `spaces` is a type of `string`. After we use the `len()` function, it returns a number type and assigns it to the spaces variable. If we tried to allow mutability, we would get an error since you aren't allowed to mutate the type of a variable.
&nbsp;
## Data Types
Rust is a statically typed language. It must know the types of variables at compile time, but the compiler can infer what type we want based on the value and how it's used. Sometimes we need to manually define the type in order for our code to work. For example, in the guessing game project we had this line to convert a string to a number: 
``` Rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
If we don't add the `u32` type annotation, the compiler doesn't have enough information to know which type we want to use.
&nbsp;
### Scalar Types
A `scalar` type represents a single value. In Rust we have four `primary` scalar types; ints, floats, bools and chars. 
&nbsp;
#### Integer Types
Integers are numbers without fractional component. In Rust we have signed and unsigned integers and they have an explicit size. The difference between unsigned and signed is that the former variant can only have positive values while the latter includes negative values. Signed numbers are stored using `two's complement representation`.
&nbsp;
#### Floating-point Types
Rust's floating-point types are `f32` and `f64`, 32 and 64 bits in size. By default the type is f64, since it's as fast as f32 on modern chips while offering more precision. All floats are signed, which meant values can be both negative and positive.
&nbsp;
#### Numeric Operations
Rust supports all the basic mathematical operations you'd expect like addition, subtraction, multiplication, division and remainder. 
&nbsp;
#### Booleans
Booleans are one byte in size. The Boolean type is specified using `bool`.
``` Rust
let t = true;
let f: bool = false // Explicit type annotation
```
&nbsp;
#### Characters
``` Rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // explicit type annotation
    let heart_eyed_cat = '😻';
}
```
char is specified with single quotes whereas strings are marked with double quotes. Rust's char is four bytes in size.
&nbsp;
### Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types which are `tuples` and `arrays`.
&nbsp;
#### Tuples
A tuple is a general way of grouping together a number of values with a variet of types into one compound type. Tuples have fixed length.
``` Rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
The variable `tup` binds to the entire tuple.
#### Arrays
Another way to have a collection of multiple values is with an array. Every element in an array must have the same type.
``` Rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
