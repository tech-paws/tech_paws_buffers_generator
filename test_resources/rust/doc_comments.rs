// GENERATED, DO NOT EDIT

//! Top level doc comment
//! Some description

#![allow(warnings)]
#![allow(clippy)]
#![allow(unknown_lints)]

use tech_paws_buffers::memory::{BytesReader, BytesWriter, BuffersModel};

/// Some doc comment
/// Another doc comment
#[derive(Debug, Clone, PartialEq)]
pub enum MyEnum {
    Idle,
    Move {
        /// This is x field
        x: f64,

        /// This is y field
        y: f64,
    },
    Update(
        /// This is first option
        f64,

        /// This is second option
        f64,

        /// This is third option
        String,
    ),
}

impl Default for MyEnum {
    fn default() -> Self {
        Self::Idle
    }
}

/// Hello World!
/// This is View Data, Important Structure!
#[derive(Debug, Clone, PartialEq)]
pub struct ViewData {
    /// Delta time is delta time
    pub delta_time: f32,

    /// View Width
    pub view_width: f32,

    /// View Height!
    pub view_height: f32,

    /// Touch Start X
    /// It is starting position
    pub touch_start_x: f32,

    /// Touch Start Y
    /// It is starting position
    pub touch_start_y: f32,
}

impl Default for ViewData {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            view_width: 0.0,
            view_height: 0.0,
            touch_start_x: 0.0,
            touch_start_y: 0.0,
        }
    }
}

impl BuffersModel for MyEnum {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        let value = bytes_reader.read_u32();

        match value {
            1 => return MyEnum::Idle,
            2 => return MyEnum::Move {
                x: bytes_reader.read_f64(),
                y: bytes_reader.read_f64(),
            },
            3 => return MyEnum::Update(
                bytes_reader.read_f64(),
                bytes_reader.read_f64(),
                String::read_from_buffers(bytes_reader),
            ),
            _ => panic!("Unsupported enum value: {}", value),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        match self {
            MyEnum::Idle => {
                bytes_writer.write_u32(1);
            },
            MyEnum::Move {
                x,
                y,
            } => {
                bytes_writer.write_u32(2);
                bytes_writer.write_f64(*x);
                bytes_writer.write_f64(*y);
            },
            MyEnum::Update(
                v0,
                v1,
                v2,
            ) => {
                bytes_writer.write_u32(3);
                bytes_writer.write_f64(*v0);
                bytes_writer.write_f64(*v1);
                v2.write_to_buffers(bytes_writer);
            },
        }
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            let value = bytes_reader.read_u32();

            match value {
                1 => (),
                2 => {
                    bytes_reader.read_f64();
                    bytes_reader.read_f64();
                },
                3 => {
                    bytes_reader.read_f64();
                    bytes_reader.read_f64();
                    String::read_from_buffers(bytes_reader);
                },
                _ => panic!("Unsupported enum value: {}", value),
            }
        }
    }
}

impl BuffersModel for ViewData {
    fn read_from_buffers(bytes_reader: &mut BytesReader) -> Self {
        Self {
            delta_time: bytes_reader.read_f32(),
            view_width: bytes_reader.read_f32(),
            view_height: bytes_reader.read_f32(),
            touch_start_x: bytes_reader.read_f32(),
            touch_start_y: bytes_reader.read_f32(),
        }
    }

    fn write_to_buffers(&self, bytes_writer: &mut BytesWriter) {
        bytes_writer.write_f32(self.delta_time);
        bytes_writer.write_f32(self.view_width);
        bytes_writer.write_f32(self.view_height);
        bytes_writer.write_f32(self.touch_start_x);
        bytes_writer.write_f32(self.touch_start_y);
    }

    fn skip_in_buffers(bytes_reader: &mut BytesReader, count: u64) {
        for _ in 0..count {
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
            bytes_reader.read_f32();
        }
    }
}
