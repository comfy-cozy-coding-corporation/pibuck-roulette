use std::{fs::{self, OpenOptions}, io};
use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::Deref;
use clap::Parser;
use console::{Emoji, style};
use sedregex::ReplaceCommand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short='f', long, required = true, help="Path to Buckshot Roulette binary")]
    binary_file: String
}

fn load_args() -> Args {
    let args: Args = Args::parse();
    args
}

pub(crate) fn write_welcome_message() {
    let intro_logo = r#"
______ _______            _     ______            _      _   _       
| ___ (_) ___ \          | |    | ___ \          | |    | | | |      
| |_/ /_| |_/ /_   _  ___| | __ | |_/ /___  _   _| | ___| |_| |_ ___ 
|  __/| | ___ \ | | |/ __| |/ / |    // _ \| | | | |/ _ \ __| __/ _ \
| |   | | |_/ / |_| | (__|   <  | |\ \ (_) | |_| | |  __/ |_| ||  __/
\_|   |_\____/ \__,_|\___|_|\_\ \_| \_\___/ \__,_|_|\___|\__|\__\___|
        
        "#;
    log::info!(
        "{}\n\t\t{}{}\n\n",
        style(intro_logo).green(),
        style("Trans rights are human rights!").magenta().bright(),
        Emoji("⚧️ 💜", "")
    );
}

fn main() {
    env_logger::Builder::new().init();
    let args: Args = load_args();
    let binary_path = args.binary_file;
    write_welcome_message();
    patch_game(binary_path);
}

fn patch_game(binary_path: String) {
    // Load file
    let contents = fs::read(binary_path.clone()).expect("File could not be read!");
    // Patch binary content
    let mut new = ReplaceCommand::new(r"s/properties\.stat_number_of_deaths += 1/properties\.stat_number_of_deaths += 1\n\tprint('player_shot')/1").expect("Could not patch game!").execute(contents);
    new = ReplaceCommand::new(r"s/DeathRequest(shot_from_direction/DeathRequest(shot_fro/1").expect("Could not patch game!").execute(new);
    new = ReplaceCommand::new(r"s/UserDeath_ThirdPerson(shot_from_direction/UserDeath_ThirdPerson(shot_fro/1").expect("Could not patch game!").execute(new);
    
    // Write new binary
    let mut file = OpenOptions::new().write(true).truncate(true).open(binary_path).expect("Could not open file!");
    file.write(new.as_bytes()).expect("Could not write new binary!");
}

