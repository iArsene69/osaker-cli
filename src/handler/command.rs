use crate::utils::file::get_file_name;

pub fn get_command_name() -> Vec<String> {
    let command_folder = "src/commands";
    let command_folders = get_file_name(command_folder, false);

    let mut command_names = Vec::new();

    for commands in command_folders {
        let mut command_files = get_file_name(commands.to_str().unwrap(), false);
        command_files.sort();

        let command_name = commands.file_name().unwrap().to_str().unwrap().to_string();

        if let Some(index) = command_name.find('.') {
            let truncated_name = &command_name[..index];
            command_names.push(truncated_name.to_string());
        }
    }

    command_names
}
