mod utils {
    pub mod file;
}
mod handler {
    pub mod command;
}

use handler::command::get_command_name;

fn main() {
    let x = get_command_name();

    println!("{:?}", x);
}
