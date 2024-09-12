#[id = "4de616f8-12c5-4d2c-8d48-9c5fb038991f"]
#[namespace = "test"]

fn print_hello_world();

fn hello_world() -> String;

fn say_hello(name: String) -> String;

fn sum(a: i32, b: f32, c: f64);

async fn print_hello_world_async();

async fn hello_world_async() -> String;

async fn say_hello_async(name: String) -> String;

async fn sum_async(a: i32, b: f32, c: f64) -> f64;

signal trigger;

signal theme -> Theme;

async signal trigger_async;

async signal theme_async -> Theme;
