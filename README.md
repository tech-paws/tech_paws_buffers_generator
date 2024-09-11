# Tech Paws Buffer Generator

## Overview

Tech Paws Buffer Generator is a code generator for Tech Paws Buffers, a protocol similar to gRPC but designed to replace FFI (Foreign Function Interface). It establishes lightweight yet fast communication between different programming languages, offering a richer type system and signal RPC methods.

## Models Example

Tech Paws Buffer Generator supports various data structures and types. Here's an example of the models you can define:

```rust
struct BasicTypesModel {
    byte: u8,
    some_integer: i32,
    some_long: i64,
    some_unsiged_integer: u32,
    some_unsiged_long: u32,
    some_float_number: f32,
    some_double_number: f64,
    some_bool: bool,
    some_string: String,
    vector: Vec<String>,
    generic: LinearTable<f32, Test>,
    custom: MyModel,
    optional_string: Option<String>,
    optional_f32: Option<f32>,
}

enum MyEnum {
    #[1] Idle,
    #[2] Move {
        #[1] x: f64,
        #[2] y: f64,
    },
    #[3] Update(#[1] f64, #[2] f64, #[4] String),
}

enum MyEnumWithoutPositions {
    Option1(u64),
    Option2 { name: String },
    Option3,
    Option4,
}

enum MyEnumWithManyArguments {
    Option1(u64, u64, u64),
    Option2 { name: String },
    Option3,
    Option4,
}

enum MyEnumWithNamedArguments {
    Option1 {
        name: String,
        value: f32,
        bytes: Vec<u8>
    },
    Option2(u64, u64, u64),
    Option3,
    Option4,
}
```

## RPC Example

Tech Paws Buffer Generator supports defining RPC methods, providing an efficient alternative to FFI function calls. Here's an example:

```rust
#[id = "ee61593c-4ac8-43db-ac01-d5170eb5d1ab"]
#[namespace = "rpc_examples"]

#[rust(use = "crate::rpc_examples::*")]

fn print_hello_world();

fn hello_world() -> String;

fn say_hello(first_name: String, last_name: String) -> String;

fn sum(a: i32, b: f32, c: f64) -> f64;

signal counter_stream -> i32;
```

## Emplace Example

The emplace functionality is designed to significantly reduce GC memory allocations by using a pool of objects. This is particularly useful in real-time contexts. Here's an example of how to use it:

```rust
#[id = "7b6d3210-e963-47bd-8c3f-77d6156f49f9"]
#[namespace = "rpc_emplace_examples"]

#[rust(use = "crate::rpc_examples::*")]

#[memory(emplace)]
enum RenderCommand {
    Svg {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    DrawLines {
        line_width: f32,
        points: Vec<Position2D>,
    },
}

#[memory(emplace, copy)]
struct Position2D {
    x: f32,
    y: f32,
}

#[memory(emplace)]
signal render_commands -> Vec<RenderCommand>;
```
