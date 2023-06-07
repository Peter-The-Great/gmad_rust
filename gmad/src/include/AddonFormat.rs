use std::collections::LinkedList;
use std::ffi::CString;

pub const IDENT: &str = "GMAD";
pub const VERSION: char = '3';
pub const APP_ID: u32 = 4000;
pub const COMPRESSION_SIGNATURE: u32 = 0xBEEFCACE;

#[repr(C)]
pub struct Header {
    ident: [u8; 4],
    version: char,
}

pub struct FileEntry {
    str_name: CString,
    i_size: i64,
    i_crc: u32,
    i_file_number: u32,
    i_offset: i64,
}

pub type FileEntryList = LinkedList<FileEntry>;

mod tags {
    pub const TYPE: [&str; 10] = [
        "gamemode",
        "map",
        "weapon",
        "vehicle",
        "npc",
        "entity",
        "tool",
        "effects",
        "model",
        "servercontent",
    ];

    pub fn type_exists(name: &str) -> bool {
        TYPE.iter().any(|&t| t == name)
    }

    pub const MISC: [&str; 9] = [
        "fun", "roleplay", "scenic", "movie", "realism", "cartoon", "water", "comic", "build",
    ];

    pub fn tag_exists(name: &str) -> bool {
        MISC.iter().any(|&t| t == name)
    }
}

pub const TIMESTAMP_OFFSET: u32 = std::mem::size_of::<Header>() as u32 + std::mem::size_of::<u64>() as u32;
