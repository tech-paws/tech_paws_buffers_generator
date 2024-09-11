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
    pub const SOME_VALUE: &'static str = "Hello World!";

    pub mod groups {
        pub const MAIN: tech_paws_runtime::GroupAddress = tech_paws_runtime::GroupAddress(0);
        pub const MAIN_RENDER: tech_paws_runtime::GroupAddress = tech_paws_runtime::GroupAddress(1);
        pub const RPC: tech_paws_runtime::GroupAddress = tech_paws_runtime::GroupAddress(2);
        pub const RPC_SYNC: tech_paws_runtime::GroupAddress = tech_paws_runtime::GroupAddress(3);
        pub const RPC_READ: tech_paws_runtime::GroupAddress = tech_paws_runtime::GroupAddress(4);
    }

    pub const DELTA_TIME: f64 = 16.6;
    pub const FLAG: bool = true;

    pub mod commands_buffers {
        pub const WIN1_MAIN_RENDER: tech_paws_runtime::CommandsBufferAddress = tech_paws_runtime::CommandsBufferAddress(0);
    }
}
