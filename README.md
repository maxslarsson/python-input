# python-input

A simple pure-rust implementation of python's input function.

## Functions

There is currently only one function, the `input` function.

### Input Function

The input function takes one parameter, a string which is printed to the console as a prompt to the user.

## Example Code

### Add the following to your `Cargo.toml` file

```toml
[dependencies]
...
python-input = "0.8.0"
...
```

### Add the following to your rust file(s)

```rust
extern crate python_input;

use python_input::input;

fn main() {
  let name = input("What is your name? ");
  let age = input("How old are you? ");

  println!("Hello {}, you are {} years old.", name, age);
}
```
