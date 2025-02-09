
### Building OS in Rust

 1. Create OS kernel as rust executable without linking the standard library to it. This makes it possible for Rust to run on bare metal without any underlying OS.

- To write OS kernel, we want the code to be independent of the OS features. 

- By default, Rust crates link the rust standard library to the rust executable providing various OS features like thread, files or networking etc. Also, it uses another C library known as `libc`. 
- We can disable the usage of standard library using `no_std` attribute.

2. Rust standard library provides it's own `panic_handler` implementation, which is the function that gets invoked whenever the panic occurs. After disabling the standard library, we have to write our own implementation.

3. We cannot have main function, after the standard library is disabled, so we need to disable the main function as well using `#[no_main]`. 

### Stack Unwinding: 
whenever panic occurs, compiler by default performs unwinding of all variables, calling their destructors, cleaning up the memory. This is provided as default implementation of `panic_handler` by rust std. library. But, as we have disabled std. library for compiling directly to bare metal target without any OS features used for creating OS kernel, the unwinding need to be removed as well. Rust provides custom panic implementation, where we can have 2 options for panic, either `abort` or `unwind`. `unwind` is the default option, but we can configure it in the `Cargo.toml` file.

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

### Rust program initialization

When you run a typical program (whether it's in Rust, C, Java, or Go), there's quite a bit happening before your main function runs. Think of it like starting up a factory - before the workers (your program's main logic) can begin, someone needs to turn on the lights, start the generators, and ensure all safety systems are operational.

--- 
In a standard Rust program, the startup sequence looks like this:
Operating System → crt0 → Rust Runtime → main function

When you run a rust binary, OS does not directly jump to your rust code. Instead, it first loads and execute `crt0` C runtime library. 

```rust
// Conceptual representation of what crt0 does
fn crt0_start() {
    // Create the initial stack
    let stack = create_stack();
    
    // Set up program arguments and loads them to CPU registers
    let args = prepare_program_arguments();
    
    // Call into the Rust runtime
    rust_start(stack, args); 
}
```

This C runtime, starts the entrypoint of the rust program which is marked by `start` language item.
Language items are special syntax used by the rust compiler for it's execution. 
```rust
#[lang = "start"] // start language item
fn lang_start(main: fn(), argc: isize, argv: *const *const u8) -> isize {
    use panic;
    use sys;
    use sys_common;
    use sys_common::thread_info;
    use thread::Thread;
    #[cfg(not(feature = "backtrace"))]
    use mem;

    sys::init();

    let failed = unsafe {
        let main_guard = sys::thread::guard::init();
        sys::stack_overflow::init();

        // Next, set up the current Thread with the guard information we just
        // created. Note that this isn't necessary in general for new threads,
        // but we just do this to name the main thread and to give it correct
        // info about the stack bounds.
        let thread = Thread::new(Some("main".to_owned()));
        thread_info::set(main_guard, thread);

        // Store our args if necessary in a squirreled away location
        sys::args::init(argc, argv);

        // Let's run some code!
        #[cfg(feature = "backtrace")]
        let res = panic::catch_unwind(|| {
            ::sys_common::backtrace::__rust_begin_short_backtrace(main)
        });
        #[cfg(not(feature = "backtrace"))]
        let res = panic::catch_unwind(mem::transmute::<_, fn()>(main));
        sys_common::cleanup();
        res.is_err()
    };

    if failed {
        101
    } else {
        0
    }
}
```

Rust has a very minimal runtime, which sets up printing a backtrace on panic and stackoverflow guards. 
Runtime finally calls the main function.

For our own OS kernel implemenation, it will not have `crt0` and rust runtime, that's why we need our own entry point. We need to overwrite the `crt0` endpoint directly. Now that we don't have rust runtime, there is also no need of main function, which was supposed to be invoked when the rust runtime sets itself up. So, that's why it was removed before.