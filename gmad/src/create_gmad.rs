use bootil::{BString, Output, CommandLine, Time, AutoBuffer, File, Hasher, String};

pub mod create_addon {
    pub fn verify_files(files: &mut Vec<BString>, warn_invalid: bool) -> bool {
        let mut b_ok = true;

        if files.is_empty() {
            Output::Warning("No files found, can't continue!\n");
            b_ok = false;
        }

        let mut old_files = std::mem::take(files);
        files.clear();

        for file in old_files.iter() {
            Output::Msg("\t{}\n", file.as_str());

            if Addon::WhiteList::Check(String::GetLower(file)) {
                files.push(file.clone());
            } else {
                Output::Warning("\t\t[Not allowed by whitelist]\n");
                if !warn_invalid {
                    b_ok = false;
                }
            }

            if String::GetLower(file) != *file {
                Output::Warning("\t\t[Filename contains capital letters]\n");
            }
        }

        b_ok
    }

    pub fn create(
        buffer: &mut AutoBuffer,
        str_folder: BString,
        files: &Vec<BString>,
        str_title: BString,
        str_description: BString,
    ) -> bool {
        let do_crs = if CommandLine::HasSwitch("-nocrc") {
            false
        } else {
            true
        };

        buffer.write(&Addon::Ident); // Ident (4)
        buffer.write_type(Addon::Version as char); // Version (1)
        buffer.write_type(0u64); // SteamID (8) [unused]
        buffer.write_type(Time::UnixTimestamp() as u64); // TimeStamp (8)
        buffer.write_type(0u8); // Required content (a list of strings)
        buffer.write_string(str_title); // Addon Name (n)
        buffer.write_string(str_description); // Addon Description (n)
        buffer.write_string("Author Name"); // Addon Author (n) [unused]
        buffer.write_type(1i32); // Addon Version (4) [unused]

        Output::Msg("Writing file list...\n");

        for (i, f) in files.iter().enumerate() {
            let file_path = str_folder.clone() + f;

            let i_size = File::Size(file_path.clone());
            if i_size <= 0 {
                Output::Warning(
                    "File '{}' seems to be empty, or we couldn't get its size! (errno={})\n",
                    file_path,
                    errno,
                );
                return false;
            }

            let i_file_num = i as u32 + 1;
            buffer.write_type(i_file_num); // File number (4)
            buffer.write_string(String::GetLower(f.clone())); // File name (all lower case!) (n)
            buffer.write_type(i_size); // File size (8)

            if do_crs {
                let i_crc = File::CRC(file_path);
                buffer.write_type(i_crc); // File CRC (4)
            } else {
                buffer.write_type(0u32);
            }
        }

        let i_file_num = 0u32;
        buffer.write_type(i_file_num);

        Output::Msg("Writing files...\n");

        for f in files.iter() {
            let mut file_buffer = AutoBuffer::new();
            let res = File::Read(str_folder.clone() + f, &mut file_buffer);

            if file_buffer.GetWritten() == 0 {
                Output::Warning(
                    "File '{}' seems to be empty (or we couldn't read it)\n",
                    str_folder.clone() + f,
                );
                return false;
            }

            let before = buffer.GetWritten();
            buffer.write_buffer(&file_buffer);
            let diff = buffer.GetWritten() - before;
            if diff < 1 {
                Output::Warning(
                    "Failed to write file '{}' - written {} bytes! (Can't grow buffer?)\n",
                    f,
                    diff,
                );
                return false;
            }
        }

        if do_crs {
            let addon_crc = Hasher::CRC32::Easy(buffer.GetBase(), buffer.GetWritten());
            buffer.write_type(addon_crc);
        } else {
            buffer.write_type(0u32);
        }

        true
    }
}

pub fn create_addon_file(str_folder: BString, str_outfile: BString, warn_invalid: bool) -> i32 {
    let mut b_errors = false;

    String::File::FixSlashes(&mut str_folder, "\\", "/");
    String::Util::TrimRight(&mut str_folder, "/");
    str_folder.push_str("/");

    if str_outfile.is_empty() {
        str_outfile = str_folder.clone();
        String::Util::TrimRight(&mut str_outfile, "/");
    }
    String::File::StripExtension(&mut str_outfile);
    str_outfile.push_str(".gma");

    Output::Msg("Looking in folder \"{}\"\n", str_folder);

    let addon_info = CAddonJson::new(str_folder.clone() + "addon.json");
    if !addon_info.GetError().is_empty() {
        Output::Warning(
            "{} error: {}\n",
            str_folder.clone() + "addon.json",
            addon_info.GetError(),
        );
        return 1;
    }

    let mut files = Vec::new();
    File::GetFilesInFolder(&str_folder, &mut files, true);

    addon_info.RemoveIgnoredFiles(&mut files);
    String::SortList(&mut files, false);

    if !create_addon::verify_files(&mut files, warn_invalid) {
        Output::Warning("File list verification failed\n");
        return 1;
    }

    let mut buffer = AutoBuffer::new();
    if !create_addon::create(
        &mut buffer,
        str_folder.clone(),
        &files,
        addon_info.GetTitle(),
        addon_info.BuildDescription(),
    ) {
        Output::Warning("Failed to create the addon\n");
        return 1;
    }

    Output::Msg(
        "Writing the .gma...\n",
        str_outfile,
        String::Format::Memory(buffer.GetWritten()),
    );

    if !File::Write(str_outfile.clone(), &buffer) {
        Output::Warning("Couldn't save to file \"{}\"\n", str_outfile);
        return 1;
    }

    Output::Msg(
        "Successfully saved to \"{}\" [{}]\n",
        str_outfile,
        String::Format::Memory(buffer.GetWritten()),
    );

    return 0;
}