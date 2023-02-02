use std::{thread, time, env};
use chrono::Local;
use mouse_rs::{Mouse};
use keyboard_types;

const ONE_MIN: time::Duration= time::Duration::from_secs(60);
// const ONE_SEC: time::Duration= time::Duration::from_secs(1);
const MILISEC_100: time::Duration= time::Duration::from_millis(100);
const TIME_FORMAT: &str = "%Y-%M-%d %H:%M:%S";

fn main() {
    let wait_time = get_wait_time();
    println!("Stay awake at {}", Local::now().format(TIME_FORMAT) );
    println!("Each interval will sleep for {} min", wait_time);
    println!("################");

    let mousers = Mouse::new();

    let mut interval = 1;
    // let init_pos = 0;
    let fixed_width = 800;
    let multi = 4;
    let keypress_total = 3;

    loop {
        let mut sleep_counter = 1;
        while sleep_counter <= wait_time {
            thread::sleep(ONE_MIN);
            println!("Slept for 1 min, {}/{}", sleep_counter, wait_time);
            sleep_counter += 1;
        }

        println!("Moving...");

        let (cur_x, cur_y) = match mousers.get_position() {
            Ok(mouse_pos) => (mouse_pos.x, mouse_pos.y),
            Err(_) => (0,0),
        };

        for i in 0..(fixed_width/multi) {
            thread::sleep(MILISEC_100);
            // println!("{}, {}",(cur_x + i * multi) % fixed_width, (cur_y + i * multi) % fixed_width);
            mousers.move_to(
                (cur_x + i * multi) % fixed_width,
                (cur_y + i * multi) % fixed_width
            ).expect("failed on mouse moving")
        }

        for i in 0..keypress_total {
            // Not too sure this function would actual send keys to system
            // original enigo lib has a bug which cause panic on windows
            thread::sleep(MILISEC_100);
            keyboard_types::webdriver::send_keys(keyboard_types::Code::ShiftLeft.to_string().as_str());

            println!("Key pressed {}/{}", i+1,keypress_total);
        }

        println!("Movement {} made at {}", interval, Local::now().format(TIME_FORMAT));
        println!("================");
        interval += 1;
    }
}

fn get_wait_time() -> i32 {
    let args: Vec<String> = env::args().collect();
    let mut wait_time :i32 = 3;
    if args.len() >= 2 {
        wait_time = i32::from_str_radix(&args[1], 10).unwrap_or(wait_time);
    }

    return wait_time;
}