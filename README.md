# Rust Experimentation
A repository containing code written in Rust.
I am documenting my learning process here.<br>
Almost all of the information here comes from the book, [The Rust Programming Language](https://doc.rust-lang.org/book/) 

Despite most of the information coming from that book, I am trying to explain these concepts to myself instead of just copying them. For experimentation and fun!

# Setting up a project
When starting a new project, we need to type the following piece of code into our terminal <br>
`$ cargo new {project_name}` ->
`cd {project_name}`

The project will contain our `src` directory, which contains our main `main.rs` file and other source files. <br>
The new directory made with the 'cargo new' command will also contain `cargo.toml` file, where we can add dependencies. We can find libraries, called 'crates' at [crates.io](https://crates.io/).
&nbsp;
# Cargo.lock and its importance
Cargo uses a lockfile to make sure that the same code works for everyone regardless when the build is ran. We can imagine a scenario where a crate (library) gets an update that fixes some bugs, but it might also contain a regression that breaks **our** code. `Cargo.lock` essentially describes the state of the 'world' at the time of a successful build. This means that our crates stay the same version as they were during time of build unless we tell cargo to go and update.

## Updating Crates
You can update crates by running `cargo update`. Note: this will not necessarily update the crate to the specific version you want to. For example, in the guessing-game folder we have installed `version 0.8.5` of `rand`. Running `cargo update` would only update the crate to any version between `0.8.5 - 0.9.0`. If we wanted to update to version 0.9.0, we would have to specify that in the `Cargo.toml` file and build the carago again.