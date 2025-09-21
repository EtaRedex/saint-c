# saint-c
tiny library to generate C code from Rust.
Intended for personal projects mainly as a C backend for compilers.

Target interface
```rust
use saint_c;

let point = saint_c::cstruct("Point")
    .field(saint_c::int, "x")
    .field(saint_c::int, "y")
    .build();

let block = saint_c::block()
    .add_statment(
        saint_c::statement(
            saint_c::function_call("printf", vec![saint_c::string_literal("Hello, World!")])
        )
    ).build();

let mainfn = saint_c::function("main", saint_c::int)
    .set_block(block)
    .build();

let file = saint_c::file()
    .include_system("stdio.h")
    .add_struct(point)
    .add_function(main)
    .build();

println!("{}", file.emit());
``` 
