use bootil::{BString, Output, CommandLine, File, String};
use std::fmt::Debug;
fn main() {
    Debug::SuppressPopups(true);
    CommandLine::Set(std::env::args().collect::<Vec<String>>().len(), std::env::args().collect::<Vec<String>>().as_slice());
    Console::FGColorPush(Console::Green);
    Output::Msg("Garry's Mod Addon Creator 1.1\n");
    Console::FGColorPop();

    let str_command = String::GetLower(CommandLine::GetArg(0));

    if str_command == "create" || File::IsFolder(str_command.clone()) {
        let mut str_folder = CommandLine::GetSwitch("-folder", "");

        if str_folder.is_empty() && str_command != "create" {
            str_folder = str_command.clone();
        }

        if str_folder.is_empty() {
            Output::Msg("Missing -folder (the folder to turn into an addon)\n");
            std::process::exit(1);
        }

        let mut str_target = CommandLine::GetSwitch("-out", "");

        let warn_on_invalid_files = CommandLine::GetFull().find("-warninvalid").is_some();

        std::process::exit(CreateAddonFile(str_folder, str_target, warn_on_invalid_files));
    }

    if str_command == "extract" || String::File::GetFileExtension(str_command.clone()) == "gma" {
        let mut str_file = CommandLine::GetSwitch("-file", "");

        if str_file.is_empty() && str_command != "extract" {
            str_file = str_command.clone();
        }

        if str_file.is_empty() {
            Output::Msg("Missing -file (the addon you want to extract)\n");
            std::process::exit(1);
        }

        let mut str_target = CommandLine::GetSwitch("-out", "");

        std::process::exit(ExtractAddonFile(str_file, str_target));
    }

    Output::Msg("\nUsage:\n\n");
    Output::Msg("\tDrag'n'drop a .gma onto the gmad.exe to extract it\n");
    Output::Msg("\tDrag'n'drop a folder onto the gmad.exe to convert it to a .gma\n\n");
    Output::Msg("\tgmad.exe create -folder path/to/folder -out path/to/gma.gma\n");
    Output::Msg("\tgmad.exe create -folder path/to/folder\n");
    Output::Msg("\tgmad.exe extract -file path/to/gma.gma -out path/to/folder\n");
    Output::Msg("\tgmad.exe extract -file path/to/gma.gma\n\n");
    Output::Msg("\tAdd -warninvalid to automatically skip invalid files\n\n");

    #[cfg(target_os = "windows")]
    {
        // Make sure they see how to use it
        // Linux folks are too smart for this
        std::process::Command::new("cmd")
            .args(&["/C", "pause"])
            .status()
            .unwrap();
    }
}