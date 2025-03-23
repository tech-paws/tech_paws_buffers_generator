#[kotlin(package = "com.tech_paws.buffers_generated")]
#[kotlin(import = "kotlinx.coroutines.channels.Channel")]

trait TestRpc {
    signal test -> String;
}
