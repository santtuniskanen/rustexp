# Guessing Game
Guessing games are usually a pretty good way to start exploring new languages. They teach you about variables, functions, libraries and control flow.

## Importing libraries
Here we are importing the default input/output library.
``` Rust
use std::io;
```

## Printing in Rust
Printing in Rust seems very straightforward.
``` Rust
println!("Hello World!");
```
You can also print variables very easily by including them between curly brackets.
``` Rust
let x = 5;
println!("x equals {x}"); 
// This would print "x equals 5"
``` 
You can also set empty brackets inside the print statement and outside the quotation marks
add the expressions.
``` Rust
let x = 5;
let y = 5;
println!("x multiplied by y equals {}", x * y);
// This would print "x multiplied by y equals 25"
```
&nbsp;
## Creating variables
The `let` keyword in Rust is used to declare a variable. Adding the keyword `mut` will make that variable mutable, since all variables in Rust are immutable by default. <br>
`String::new();` creates an empty string, indicated by the `new()` function. It does not allocate any initial buffer, so it can cause excessive allocation later when adding data to it.
``` Rust
let mut guess = String::new();
```

## Receiving User Input 
Next we use the `stdin()` function from the standard input/output library which allows us to handle user input. `.read_line()` method is called from, the library and `&mut guess` is passed as the argument so we can pass the user input into the `guess` variable we declared earlier on. Notice we also need to pass `mut` as an argument so we can change the strings content.
`&` is means the argument is a reference. A reference is a way to let parts of our code access a piece of data without needing to copy that data into memory multiple times.
``` Rust
io::stdin()
        .read_line(&mut guess)
```

### Handling potential failure
The `.read_line` method doesn't just save user input into a string, it also returns a `Result` value, where the type is an enumeration. The value can be one of two things: `Ok` or `Err`. The Result has an `expect` method we can call.
``` Rust
.expect("Failed to read line");
```
In the case that the method brings a Result of 'Err', maybe due to underlying error coming from your Operating System, the '.expect' method crashes the program and displays a message we passed as an argument to the method.
&nbsp;
## Generating a random number
We can use the [rand](https://crates.io/crates/rand) library for random number generation.
To add the libary to our project we could run 'cargo add rand' or go to the `Cargo.toml` file and manually add `rand = "0.8.5"` in the dependencies section. Then we only need to run `cargo build` and the library is installed to our project.
&nbsp;
``` Rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```
Here you can see I have defined a variable called `secret_number`. In order to create the random number, we use the `thread_rng()` function, which gives the seed for our rng by the operating system. After that we call `gen_range(1..=100)` which sets the range of the randomization between 1 and 100.
&nbsp;
## Comparing player choice to the randomly generated value
Next we need to import a crate called `cmp` which stands for comparison. This library will let us compare our values with a function called `Ordering`. Just like the `.read_line()` function, Ordering is a type of `Enum` and has variants `Less, Greater and Equal`.
&nbsp;
This function is called on the `guess` variable we defined earlier. We are also using the `match` expression decide what to do next.
``` Rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your number is too small"),
        Ordering::Greater => println!("Your number is too big"),
        Ordering::Equal => println!("You win!"),
    }
```
&nbsp;
### Potential issues we need to fix
This match block wouldn't work on its own right now. When we defined `guess` variable, we called the `String::new` method to create an empty `string`. The randomly generated `secret_number` however is the type of int, so there is a mismatch.
&nbsp;
In order to fix this error, we add this line of code to the body...
``` Rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```
Here we are `shadowing` the original `guess` value we defined earlier and are saving user input into. It's useful when we need to convert values to other types without having to create new variables and creating more clutter. In order to do this we tell Rust that guess should be type of `unsigned` 32-bit integer. Unsigned means that the integer can only hold non-negative values. 
&nbsp;
In the following line, `trim()` function removes whitespace and `parse()` function converts strings to other types. The function also returns another Result type, 'Ok' and 'Err'. 
``` Rust
guess.trim().parse()
```
Parse can only work on characters that can logically be converted into numbers. This means that erros can happen very easily. If parse returns 'Err' Result, he expect call will crash the application and print the error message. We'll get to this point later on since even this result isn't perfect as inputting characters will cause the except method to panic.
&nbsp;
## Loooooops
In order to guess multiple times, we can throw our program inside a loop:
``` Rust 
loop {
        // -- snip --
}
```
We need to also remember to exit the loop eventually, which we can do with the `break` statement.
``` Rust
loop {
        condition;
        condition;
        condition met {
                break;
        }
}
```