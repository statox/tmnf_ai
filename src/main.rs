mod controller;
mod player;
mod vision;

use enigo::*;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use player::Player;
use vision::Vision;

fn main() {
    start_delay();
    let duration_in_seconds = 40;

    let mut player = Player::new();
    let mut vision = Vision::new();

    let start = Instant::now();
    loop {
        if start.elapsed().as_secs() >= duration_in_seconds {
            break;
        }

        let speed = vision.read_speed();
        player.drive(speed);
    }

    player.show_controller_stats();
    clean_up();
}

fn start_delay() {
    println!("Switch to TM now");
    thread::sleep(Duration::from_secs(3));
    println!("5");
    thread::sleep(Duration::from_secs(1));
    println!("4");
    thread::sleep(Duration::from_secs(1));
    println!("3");
    thread::sleep(Duration::from_secs(1));
    println!("2");
    thread::sleep(Duration::from_secs(1));
    println!("1");
    thread::sleep(Duration::from_secs(1));
    println!("0");
}

fn clean_up() {
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Layout('a'));
    enigo.key_up(Key::LeftArrow);
    enigo.key_up(Key::RightArrow);
}
