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
        MAIN = GroupAddress { address: 0 };
        MAIN_RENDER = GroupAddress { address: 1 };
        RPC = GroupAddress { address: 2 };
        RPC_SYNC = GroupAddress { address: 3 };
        RPC_READ = GroupAddress { address: 4 };
    }

    DELTA_TIME: f64 = 16.6;
    FLAG: bool = true;

    const render_commands {
        RENDER_PLANES = RenderCommand::Planes;
        RENDER_LINES = RenderCommand::Lines(32.)
        RENDER_CIRCLES = RenderCommand::Circles {
            radius: 32.,
            color: ColorRGBA { r: 1., g: 0., b: 0., a: 1. },
        };
    }
}
