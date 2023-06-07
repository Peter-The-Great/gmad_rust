use bootil::{BString, Output, File, AutoBuffer, String};

pub fn extract_addon_file(str_file: BString, mut str_out_path: BString) -> i32 {
    Output::Msg("Opening \"{}\"\n", str_file);

    // If an output path hasn't been provided, make our own
    if str_out_path.is_empty() {
        str_out_path = String::File::GetStripExtension(str_file.clone());
    }

    // Remove slash, add slash (enforces a slash)
    String::File::FixSlashes(&mut str_out_path);
    String::Util::TrimRight(&mut str_out_path, "/");
    str_out_path.push_str("/");

    let mut addon = Addon::Reader::new();
    if !addon.ReadFromFile(str_file.clone()) {
        Output::Warning("There was a problem opening the file\n");
        return 1;
    }

    if !addon.Parse() {
        Output::Warning("There was a problem parsing the file\n");
        return 1;
    }

    Output::Msg("Extracting Files:\n");
    let mut bad_file_count = 0;
    for entry in addon.GetList().iter() {
        Output::Msg(
            "\t{} [{}]\n",
            entry.strName,
            String::Format::Memory(entry.iSize),
        );

        // Make sure folders exist
        File::CreateFolder(
            &format!("{}{}", str_out_path, String::File::GetStripFilename(entry.strName)),
            true,
        );

        // Load the file into the buffer
        let mut file_contents = AutoBuffer::new();
        if addon.ReadFile(entry.iFileNumber, &mut file_contents) {
            // Write the file to disk
            if !File::Write(&format!("{}{}", str_out_path, entry.strName), &file_contents) {
                let gen_path = format!("badnames/{}.unk", bad_file_count);
                Output::Warning("\t\tCouldn't write, trying to write as '{}'..\n", gen_path);

                // Try to write the file but don't use any of its name, since we don't know which part of it may have caused the problem
                File::CreateFolder(&format!("{}badnames/", str_out_path), true);
                File::Write(&format!("{}{}", str_out_path, gen_path), &file_contents);
                bad_file_count += 1;
            }
        } else {
            Output::Warning("\t\tCouldn't extract!\n");
        }
    }
    Output::Msg("Done!\n");
    return 0;
}