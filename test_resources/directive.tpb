#[namespace = "test"]
#[dart(file_name = "test.dart", rpc_prefix = "Prefix")]

#[memory(emplace, copy)]
struct Point {
    x: i32,
    y: i32,
}

#[memory(emplace)]
enum Variant {
    None,
    Something(Point),
}

struct Regular {
    value: i32,
}

#[memory(emplace, copy)]
fn hello_world();

#[memory(emplace)]
signal test -> Vec<Variant>;
