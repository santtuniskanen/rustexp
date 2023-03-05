# Guessing Game

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
