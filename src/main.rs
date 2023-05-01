use std::{thread, time, env};
use chrono::Local;
use mouse_rs::{Mouse};
use rdev::{simulate, EventType, Key, SimulateError};

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
        let mut last_x = cur_x;
        let mut last_y = cur_y;
        for i in 0..(fixed_width/multi) {
            thread::sleep(MILISEC_100);
            // println!("{}, {}",(cur_x + i * multi) % fixed_width, (cur_y + i * multi) % fixed_width);
            let (x, y) = match mousers.get_position() {
                Ok(mouse_pos) => (mouse_pos.x, mouse_pos.y),
                Err(_) => (0,0),
            };
            if last_x != x || last_y != y {
                println!("Mouse moved, skip moving...");
                break;
            }

            send(&EventType::MouseMove {
                x: f64::from((cur_x + i * multi) % fixed_width),
                y: f64::from((cur_y + i * multi) % fixed_width)
            });

            last_x = (cur_x + i * multi) % fixed_width;
            last_y = (cur_y + i * multi) % fixed_width

            // enigo.mouse_move_to(
            //     (cur_x + i * multi) % fixed_width,
            //     (cur_y + i * multi) % fixed_width
            // )
        }

        for i in 0..keypress_total {
            // enigo.key_click(enigo::Key::Shift);
            send(&EventType::KeyPress(Key::ShiftLeft));

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

fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(MILISEC_100);
}