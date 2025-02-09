
### Building OS in Rust

 1. Create OS kernel as rust executable without linking the standard library to it. This makes it possible for Rust to run on bare metal without any underlying OS.

- To write OS kernel, we want the code to be independent of the OS features. 

- By default, Rust crates link the rust standard library to the rust executable providing various OS features like thread, files or networking etc. Also, it uses another C library known as `libc`. 
- We can disable the usage of standard library using `no_std` attribute.

2. Rust standard library provides it's own `panic_handler` implementation, which is the function that gets invoked whenever the panic occurs. After disabling the standard library, we have to write our own implementation.