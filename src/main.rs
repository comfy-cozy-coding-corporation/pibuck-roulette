use std::{fs::{self, OpenOptions}, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::ops::Deref;
use std::process::{exit, Command, Stdio};
use std::time::Duration;
use clap::Parser;
use console::{Emoji, style};
use log::{error, LevelFilter};
use pishock_rs::interpolation::ShockPoint;
use pishock_rs::{PiShockAccount, PiShocker};
use regex::bytes;
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

#[tokio::main]
async fn main() {
    env_logger::Builder::new().filter_level(LevelFilter::Info).init();
    let args: Args = load_args();
    let binary_path = args.binary_file;
    write_welcome_message();
    patch_game(binary_path.clone());
    
    run_game(binary_path).await;
}

fn patch_game(binary_path: String) {
    // Load file
    let contents = fs::read(binary_path.clone()).expect("File could not be read!");
    // Patch binary content
    let mut regex = bytes::Regex::new(r"speaker_glimpse.pitch_scale = randf_range\(.8, 1\)").expect("Could not create replacement regex!");
    let new = regex.replace(&contents, b"print('player_shot'                            )");

    // regex = bytes::Regex::new(r"DeathRequest\(shot_from_direction").expect("Could not create replacement regex!");
    // let new2 = regex.replace(&new, b"DeathRequest(shot_fro");
    // 
    // regex = bytes::Regex::new(r"UserDeath_ThirdPerson\(shot_from_direction").expect("Could not create replacement regex!");
    // let new3 = regex.replace(&new2, b"UserDeath_ThirdPerson(shot_fro");
    
    // Write new binary
    let mut file = OpenOptions::new().write(true).truncate(true).open(binary_path).expect("Could not open file!");
    file.write(&new).expect("Could not write new binary!");
}

async fn run_game(binary_path: String) {


    let shocker_share_code = std::env::var("PISHOCK_SHARECODE").unwrap_or(String::new());
    let shocker_api_key = std::env::var("PISHOCK_APIKEY").unwrap_or(String::new());
    let shocker_api_username = std::env::var("PISHOCK_USERNAME").unwrap_or(String::new());

    println!("Shocker share code (PISHOCK_SHARECODE): {shocker_share_code}");
    println!("Shocker API key (PISHOCK_APIKEY): {shocker_api_key}");
    println!("Shocker API username (PISHOCK_USERNAME): {shocker_api_username}");

    if shocker_share_code.is_empty()
        || shocker_api_key.is_empty()
        || shocker_api_username.is_empty()
    {
        error!("PISHOCK_SHARECODE, PISHOCK_APIKEY and PISHOCK_USERNAME must be set");
        exit(1);
    }

    // Create a new PiShockAccount instance
    let pishock_account = pishock_rs::PiShockAccount::new(
        "pishock_rs example".to_string(),
        shocker_api_username,
        shocker_api_key,
    );

    let test_pishocker_instance = pishock_account
        .get_shocker(shocker_share_code.clone())
        .await
        .unwrap();
    
    
    // let process_handle = Command::new(binary_path).spawn().expect("Failed to execute game!");
    // process_handle.stdout.read(&mut []);
    let stdout = Command::new(binary_path)
        .stdout(Stdio::piped())
        .spawn().expect("")
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).expect("Baeh");

    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        if line.expect("Haiiii") == "player_shot" {
            send_shock(&pishock_account, &test_pishocker_instance).await;
        }
    }
}

async fn send_shock(pishock_account: &PiShockAccount, test_pishocker_instance: &PiShocker) {
    test_pishocker_instance
        .shock(65, Duration::from_secs(1))
        .await;

    // // Get a PiShocker instance
    // let pishocker_instance: PiShocker = match pishock_account.get_shocker(shocker_share_code).await
    // {
    //     Ok(pishock_instance) => pishock_instance,
    //     Err(e) => {
    //         error!("Failed to get PiShocker instance: {e}");
    //         exit(1);
    //     }
    // };
    // 
    // // Print all the PiShocker's details
    // println!("PiShocker details:");
    // println!("  Name: {}", pishocker_instance.get_shocker_name().unwrap());
    // println!(
    //     "  Max intensity: {}",
    //     pishocker_instance.get_max_intensity().unwrap()
    // );
    // println!(
    //     "  Max duration: {:#?}",
    //     pishocker_instance.get_max_duration().unwrap()
    // );
    // println!(
    //     "  Client ID: {}",
    //     pishocker_instance.get_client_id().unwrap()
    // );
    // println!(
    //     "  Online: {}",
    //     pishocker_instance.get_shocker_online().unwrap()
    // );
    // println!(
    //     "  Paused: {}",
    //     pishocker_instance.get_shocker_paused().unwrap()
    // );
}
