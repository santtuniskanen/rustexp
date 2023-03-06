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
