struct ExamplesSyncRpc {
    private static let scopeId = "4de616f8-12c5-4d2c-8d48-9c5fb038991f"

    static func printHelloWorld() {
        TechPawsBuffersRpc.rpc(
            scopeId: scopeId,
            methodId: 0
        ) { runtime in
            runtime.callRpc()
        }
    }

    static func helloWorld() -> String {
        TechPawsBuffersRpc.rpc(
            scopeId: scopeId,
            methodId: 1
        ) { runtime in
            runtime.callRpc()

            return runtime.readResult() { bytesReader in
                return String.readFromBuffers(bytesReader)
            }
        }
    }

    static func sayHello(
        firstName: String,
        lastName: String
    ) -> String {
        TechPawsBuffersRpc.rpc(
            scopeId: scopeId,
            methodId: 2
        ) { runtime in
            runtime.writeArgs() { bytesWriter in
                firstName.writeToBuffers(bytesWriter)
                lastName.writeToBuffers(bytesWriter)
            }
            runtime.callRpc()

            return runtime.readResult() { bytesReader in
                return String.readFromBuffers(bytesReader)
            }
        }
    }

    static func sum(
        a: Int32,
        b: Float,
        c: Double
    ) -> Double {
        TechPawsBuffersRpc.rpc(
            scopeId: scopeId,
            methodId: 3
        ) { runtime in
            runtime.writeArgs() { bytesWriter in
                bytesWriter.writeInt32(a)
                bytesWriter.writeFloat(b)
                bytesWriter.writeDouble(c)
            }
            runtime.callRpc()

            return runtime.readResult() { bytesReader in
                return bytesReader.readDouble()
            }
        }
    }
}
