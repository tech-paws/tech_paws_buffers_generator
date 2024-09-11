struct ExamplesStreamRpc {
    private static let scopeId = "723ca727-6a66-43a7-bfcc-b8ad94eac9be"

    private(set) static var counterCurrent: Int32 = 0
    private static let counterSubject = PassthroughSubject<Int32, Never>()
    private(set) static var themeCurrent: String = ""
    private static let themeSubject = PassthroughSubject<String, Never>()
    private static let triggerSubject = PassthroughSubject<Void, Never>()

    static var counter: AnyPublisher<Int32, Never> {
        return counterSubject
            .receive(on: DispatchQueue.main)
            .eraseToAnyPublisher()
    }

    static var theme: AnyPublisher<String, Never> {
        return themeSubject
            .receive(on: DispatchQueue.main)
            .eraseToAnyPublisher()
    }

    static var trigger: AnyPublisher<Void, Never> {
        return triggerSubject
            .receive(on: DispatchQueue.main)
            .eraseToAnyPublisher()
    }

    static func consumeStreams(_ runtime: TechPawsBuffersStream) {
        runtime.consumeResult(
            scopeId: scopeId,
            methodId: 0
        ) { bytesReader in
            let value = bytesReader.readInt32()
            counterCurrent = value
            counterSubject.send(value)
        }
        runtime.consumeResult(
            scopeId: scopeId,
            methodId: 1
        ) { bytesReader in
            let value = String.readFromBuffers(bytesReader)
            themeCurrent = value
            themeSubject.send(value)
        }
        runtime.consumeResult(
            scopeId: scopeId,
            methodId: 2
        ) { bytesReader in
            triggerCurrent = value
            triggerSubject.send(())
        }
    }
}
