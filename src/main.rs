use chrono::Local;
use mouse_rs::types::Point;
use mouse_rs::Mouse;
use rdev::{simulate, EventType, Key, SimulateError};
use std::{env, thread, time};

const ONE_MIN: time::Duration = time::Duration::from_secs(60);
// const ONE_SEC: time::Duration= time::Duration::from_secs(1);
const MILISEC_100: time::Duration = time::Duration::from_millis(100);
const TIME_FORMAT: &str = "%Y-%M-%d %H:%M:%S";

fn main() {
    let wait_time = get_wait_time();
    println!("Stay awake at {}", Local::now().format(TIME_FORMAT));
    println!("Each interval will sleep for {} min", wait_time);
    println!("################");

    let mousers = Mouse::new();

    let mut interval = 1;
    let steps = get_steps();
    println!("Steps: {}", steps);
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

        let (init_x, init_y) = match mousers.get_position() {
            Ok(mouse_pos) => (mouse_pos.x, mouse_pos.y),
            Err(_) => (0, 0),
        };
        let mut last_x = init_x;
        let mut last_y = init_y;
        let mut reversed = false;
        let mut c = 0;
        let mut direction = 1;
        for i in 0..steps {
            thread::sleep(MILISEC_100);

            // println!("c: {}, x: {}, y: {}", c, (init_x + c * multi), (init_y + c * multi));
            if detect_mouse_moved(
                &mousers,
                Point {
                    x: last_x,
                    y: last_y,
                },
            ) {
                println!("Mouse moved, skip moving...");
                break;
            }

            send(&EventType::MouseMove {
                x: f64::from(init_x + c * multi),
                y: f64::from(init_y + c * multi),
            });

            last_x = init_x + c * multi;
            last_y = init_y + c * multi;

            if i >= steps / 2 - 1 && !reversed {
                println!("reversing...");
                reversed = true;
                direction = direction * -1;
            }
            c = c + direction;
        }

        for i in 0..keypress_total {
            send(&EventType::KeyPress(Key::ShiftLeft));
            println!("Key pressed {}/{}", i + 1, keypress_total);
        }

        println!(
            "Movement {} made at {}",
            interval,
            Local::now().format(TIME_FORMAT)
        );
        println!("================");
        interval += 1;
    }
}

fn get_wait_time() -> i32 {
    let args: Vec<String> = env::args().collect();
    let mut wait_time: i32 = 3;
    if args.len() >= 2 {
        wait_time = i32::from_str_radix(&args[1], 10).unwrap_or(wait_time);
    }

    return wait_time;
}

fn get_steps() -> i32 {
    let args: Vec<String> = env::args().collect();
    let mut steps: i32 = 200;
    if args.len() >= 3 {
        steps = i32::from_str_radix(&args[2], 10).unwrap_or(steps);
    }

    return steps;
}

fn detect_mouse_moved(mousers: &Mouse, pos: Point) -> bool {
    let (x, y) = match mousers.get_position() {
        Ok(mouse_pos) => (mouse_pos.x, mouse_pos.y),
        Err(_) => (0, 0),
    };

    if pos.x != x || pos.y != y {
        return true;
    }

    return false;
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
