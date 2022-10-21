#![allow(unused)]

use std::{
    path::{
        Path,
    },
    fs::{
        File,
        remove_file,
        rename,
    },
    io::{
        Read,
        Write,
        BufReader,
        BufWriter,
        Error as IoError,
    },
    ops::{
        ControlFlow,
    },
};

use egui::*;

use rustnbt::tag::*;
use rustnbt::io::*;
use rustnbt::*;

/*
The plan is to have a multi-tab/split-view NBT-editor with a lot of built in functionality.
The first step to achieving that is to have some sort of persistent NBT root component that can be rendered with egui.
*/

// Struct for holding an Nbt
pub struct NbtFile {
    path: String,
    pub root: NamedTag,
}

impl NbtFile {

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, NbtError> {
        let mut file = File::open(path.as_ref())?;
        let size = file.metadata().expect("Failed to unwrap metadata.").len() as usize;
        let buffer_capacity = size.min(rustnbt::mebibytes(4));
        let mut reader = BufReader::with_capacity(buffer_capacity, file);
        let root = NamedTag::nbt_read(reader.get_mut())?;
        let path = String::from(path.as_ref().to_str().expect("Failed to create string from path."));
        Ok(Self {
            path,
            root,
        })
    }

    pub fn save(&self) -> Result<usize, NbtError> {
        let mut file = File::create(&self.path)?;
        let buffer_capacity = self.root.tag().nbt_size().min(rustnbt::mebibytes(4));
        let mut writer = BufWriter::with_capacity(buffer_capacity, file);
        self.root.nbt_write(writer.get_mut())
    }
}

// Now I'll need to write some code to render an NBT Tree in egui.
// I'll create a trait for that, I'll call it NbtWidget.
// Since this will be a tool for handling Minecraft NBT data, I will
// restrict the API to only tags that are supported by Minecraft.

trait NbtWidget {
    fn render(&mut self, ui: &mut Ui) -> Response;
}

fn foo() {
}

// I can use the tag_info_table from rustnbt. This can give me a shortcut
// that will allow me to quickly generate this code.
// I just have to make some design decisions.
// For one, how do I want to handle Array types? There may be many elements, so I may want some sort of way to navigate a large array.
macro_rules! primitive_table {
    ($($primitive:ident)+) => {
        $(
            impl NbtWidget for $primitive {
                fn render(&mut self, ui: &mut Ui) -> Response {
                    egui::widgets::Slider::new(self, $primitive::MIN..=$primitive::MAX).ui(ui)
                }
            }
        )+
    };
}

/// These are the primitive types that will be read and write in Big-Endian order.
primitive_table![
    i8 u8
    i16 u16
    i32 u32 f32
    i64 u64 f64
];