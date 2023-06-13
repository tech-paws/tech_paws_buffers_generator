const commands {
    DRAW_LINES: u64 = 0x0002_0001;
    DRAW_PATH: u64 = 0x0002_0002;
    DRAW_QUADS: u64 = 0x0002_0003;
    DRAW_CENTERED_QUADS: u64 = 0x0002_0004;
    DRAW_TEXTS: u64 = 0x0002_0005;
    SET_COLOR_PIPELINE: u64 = 0x0002_0006;
    SET_TEXTURE_PIPELINE: u64 = 0x0002_0007;
    DRAW_CIRCLES: u64 = 0x0002_0008;
    DRAW_HOLLOW_CIRCLES: u64 = 0x0002_0009;
}

const addr {
    SOME_VALUE: String = "Hello World!";

    const groups {
        MAIN: GroupAddress = 0;
        MAIN_RENDER: GroupAddress = 1;
        RPC: GroupAddress = 2;
        RPC_SYNC: GroupAddress = 3;
        RPC_READ: GroupAddress = 4;
    }

    DELTA_TIME: i64 = 16.6;

    const commands_buffers {
        WIN1_MAIN_RENDER: CommandsBufferAddress = 0;
    }
}
