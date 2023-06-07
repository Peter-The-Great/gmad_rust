use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, Read};

use bootil::{AutoBuffer, BString, Buffer, Data, File as BootilFile, Json, Tree};

pub mod AddonReader;

pub struct Reader {
    buffer: AutoBuffer,
    fmt_version: char,
    name: BString,
    author: BString,
    desc: BString,
    file_block: u32,
    tags: Vec<String>,
    index: FileEntryList,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            buffer: AutoBuffer::new(),
            fmt_version: '\0',
            name: BString::new(),
            author: BString::new(),
            desc: BString::new(),
            file_block: 0,
            tags: Vec::new(),
            index: FileEntryList::new(),
        }
    }

    pub fn read_from_file(&mut self, file_name: BString) -> io::Result<()> {
        self.buffer.clear();
        let mut file = File::open(file_name.to_string())?;
        file.read_to_end(&mut self.buffer)?;
        Ok(())
    }

    pub fn parse(&mut self) -> bool {
        self.buffer.set_pos(0);

        // Ident
        if self.buffer.read_type::<char>() != 'G'
            || self.buffer.read_type::<char>() != 'M'
            || self.buffer.read_type::<char>() != 'A'
            || self.buffer.read_type::<char>() != 'D'
        {
            return false;
        }

        // Format Version
        self.fmt_version = self.buffer.read_type::<char>();

        if self.fmt_version > VERSION {
            return false;
        }

        self.buffer.read_type::<u64>(); // steamid
        self.buffer.read_type::<u64>(); // timestamp

        // Required content (not used at the moment, just read out)
        if self.fmt_version > 1 {
            let mut str_content = self.buffer.read_string();

            while !str_content.is_empty() {
                str_content = self.buffer.read_string();
            }
        }

        self.name = self.buffer.read_string();
        self.desc = self.buffer.read_string();
        self.author = self.buffer.read_string();

        self.buffer.read_type::<i32>(); // Addon version - unused

        let mut i_file_number = 1;
        let mut i_offset = 0;

        while self.buffer.read_type::<u32>() != 0 {
            let mut entry = FileEntry {
                str_name: BString::new(),
                i_size: 0,
                i_crc: 0,
                i_file_number,
                i_offset,
            };

            entry.str_name = self.buffer.read_string();
            entry.i_size = self.buffer.read_type::<i64>();
            entry.i_crc = self.buffer.read_type::<u32>();
            entry.i_offset = i_offset;
            self.index.push_back(entry);
            i_offset += entry.i_size;
            i_file_number += 1;
        }

        self.file_block = self.buffer.get_pos();

        // Try to parse the description
        let mut json = Tree::new();

        if let Ok(json_data) = self.desc.to_str() {
            if Json::import(&mut json, json_data) {
                self.desc = json.child_value("description").unwrap_or_default();
                self.tags = json
                    .get_child("tags")
                    .map(|tags| tags.children().map(|tag| tag.value().to_string()).collect())
                    .unwrap_or_default();
            }
        }

        true
    }

    pub fn get_file(&self, file_id: u32) -> Option<FileEntry> {
        self.index.iter().find(|file| file.i_file_number == file_id).cloned()
    }

    pub fn read_file(&self, file_id: u32, buffer: &mut Buffer) -> bool {
        if let Some(file) = self.get_file(file_id) {
            buffer.write(
                self.buffer
                    .get_base(self.file_block + file.i_offset as usize),
                file.i_size as usize,
            );
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.fmt_version = '\0';
        self.name.clear();
        self.author.clear();
        self.desc.clear();
        self.index.clear();
        self.file_block = 0;
        self.tags.clear();
    }

    pub fn get_list(&self) -> &FileEntryList {
        &self.index
    }

    pub fn get_format_version(&self) -> char {
        self.fmt_version
    }

    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn title(&self) -> &str {
        self.name.to_str()
    }

    pub fn description(&self) -> &str {
        self.desc.to_str()
    }

    pub fn author(&self) -> &str {
        self.author.to_str()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}

pub type FileEntryList = LinkedList<FileEntry>;

pub struct FileEntry {
    str_name: BString,
    i_size: i64,
    i_crc: u32,
    i_file_number: u32,
    i_offset: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() {
        let mut reader = Reader::new();
        assert!(reader.read_from_file(BString::from("example.gma")).is_ok());
        assert!(reader.parse());
        assert_eq!(reader.title(), "Example Addon");
        assert_eq!(reader.description(), "This is an example addon for testing.");
        assert_eq!(reader.author(), "John Doe");
        assert_eq!(reader.get_format_version(), '3');
        assert_eq!(reader.get_list().len(), 1);

        let mut buffer = Buffer::new();
        assert!(reader.read_file(1, &mut buffer));
        assert_eq!(buffer.len(), 6);
    }
}