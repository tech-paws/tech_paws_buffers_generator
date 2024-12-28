object ExamplesSyncRpc {
    private const val SCOPE_ID = "4de616f8-12c5-4d2c-8d48-9c5fb038991f"

    fun printHelloWorld() {
        TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { runtime ->
            runtime.callRpc()
        }
    }

    fun helloWorld(): String {
        return TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 1U,
        ) { runtime ->
            runtime.callRpc()

            runtime.readResult { reader ->
                String.readFromBuffers(reader)
            }
        }
    }

    fun sayHello(
        firstName: String,
        lastName: String,
    ): String {
        return TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 2U,
        ) { runtime ->
            runtime.writeArgs { writer ->
                firstName.writeToBuffers(writer)
                lastName.writeToBuffers(writer)
            }
            runtime.callRpc()

            runtime.readResult { reader ->
                String.readFromBuffers(reader)
            }
        }
    }

    fun sum(
        a: Int,
        b: Float,
        c: Double,
    ): Double {
        return TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 3U,
        ) { runtime ->
            runtime.writeArgs { writer ->
                a.writeToBuffers(writer)
                b.writeToBuffers(writer)
                c.writeToBuffers(writer)
            }
            runtime.callRpc()

            runtime.readResult { reader ->
                Double.readFromBuffers(reader)
            }
        }
    }
}
