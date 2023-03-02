# Guessing Game

## Importing libraries
Here we are importing the default input/output library.
``` Rust
use std::io;
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