#[kotlin(package = "com.tech_paws.buffers_generated")]
#[kotlin(import = "kotlinx.coroutines.channels.Channel")]

const address {
    ADDRESS: u64 = 1;
}

struct Test {
    value: i32,
}

trait TestRpc {
    fn test_fn();

    signal test -> String;
}
