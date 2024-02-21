struct Commands {
    static let drawLines: UInt64 = 131073;
    static let drawPath: UInt64 = 131074;
    static let drawQuads: UInt64 = 131075;
    static let drawCenteredQuads: UInt64 = 131076;
    static let drawTexts: UInt64 = 131077;
    static let setColorPipeline: UInt64 = 131078;
    static let setTexturePipeline: UInt64 = 131079;
    static let drawCircles: UInt64 = 131080;
    static let drawHollowCircles: UInt64 = 131081;
}

struct Addr {
    static let someValue: String = "Hello World!";

    struct Groups {
        static let main: UInt64 = 0;
        static let mainRender: UInt64 = 1;
        static let rpc: UInt64 = 2;
        static let rpcSync: UInt64 = 3;
        static let rpcRead: UInt64 = 4;
    }

    static let deltaTime: Double = 16.6;
    static let flag: Bool = true;

    struct CommandsBuffers {
        static let win1MainRender: UInt64 = 0;
    }
}
