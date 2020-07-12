use std::{error::Error};

use structopt::StructOpt;

use base::{app, ui};
use ui::Actions;
use utils::files::read_file_lines;

mod base;
mod utils;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn get_path_from_args() -> String {
    let args = Cli::from_args();
    String::from(args.path.to_str().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let names = read_file_lines(get_path_from_args()).unwrap();
    let mut app = app::App::new(names);

    let mut ui = ui::UI::new();

    loop {
        ui.render(&mut app)?;
        match ui.check_events(&mut app) {
            Actions::QUIT => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
