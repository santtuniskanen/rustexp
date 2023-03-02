# Guessing Game


Here we are importing the default input/output library.
``` Rust
use std::io;
```

The `let` keyword in Rust is used to declare a variable. Adding the keyword `mut` will make that variable mutable, since all variables in Rust are immutable by default. <br>
`String::new();` creates an empty string, indicated by the `new()` function. It does not allocate any initial buffer, so it can cause excessive allocation later when adding data to it.
``` Rust
let mut guess = String::new();
```

