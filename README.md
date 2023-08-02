# rust-go-example

An example of how to combine Rust and Go in the same program.

This function:

```go
func Add(x, y int) int {
    return x + y
}
```

Is compiled into a dynamic library (using `cargo`, `build.rs` and the `go` compiler).

The `Add` function is then called from Rust (using unsafe Rust and the `libc` crate):

```rust
result = Add(x, y);
```

And the answer is printed out:

```rust
println!("The answer is: {}", result);
```

Tested on Arch Linux.
