pub trait TestRpc {
    fn print_hello_world();

    fn hello_world() -> String;

    fn say_hello(
        first_name: String,
        last_name: String,
    ) -> String;

    fn sum(
        a: Int,
        b: Float,
        c: Double,
    ) -> Double;
}

pub trait TestSignalRpc {
    fn counter() -> SignalRpcResult<Int>;

    fn theme() -> SignalRpcResult<String>;

    fn trigger() -> SignalRpcResult<()>;
}
