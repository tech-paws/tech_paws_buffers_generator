pub mod commands {
    pub const DRAW_LINES: u64 = 131073;
    pub const DRAW_PATH: u64 = 131074;
    pub const DRAW_QUADS: u64 = 131075;
    pub const DRAW_CENTERED_QUADS: u64 = 131076;
    pub const DRAW_TEXTS: u64 = 131077;
    pub const SET_COLOR_PIPELINE: u64 = 131078;
    pub const SET_TEXTURE_PIPELINE: u64 = 131079;
    pub const DRAW_CIRCLES: u64 = 131080;
    pub const DRAW_HOLLOW_CIRCLES: u64 = 131081;
}

pub mod addr {
    pub const SOME_VALUE: String = "Hello World!";

    pub mod groups {
        pub const MAIN: GroupAddress = GroupAddress { address: 0 };
        pub const MAIN_RENDER: GroupAddress = GroupAddress { address: 1 };
        pub const RPC: GroupAddress = GroupAddress { address: 2 };
        pub const RPC_SYNC: GroupAddress = GroupAddress { address: 3 };
        pub const RPC_READ: GroupAddress = GroupAddress { address: 4 };
    }

    pub const DELTA_TIME: f64 = 16.6;
    pub const FLAG: bool = true;

    pub mod commands_buffers {
        pub const RENDER_PLANES: RenderCommand = RenderCommand::Planes;
        pub const RENDER_LINES: RenderCommand = RenderCommand::Lines(32.);
        pub const RENDER_CIRCLES: RenderCommand = RenderCommand::Circles {
            radius: 32.,
            color: ColorRGBA {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            }
        };
    }
}
