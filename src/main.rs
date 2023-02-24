use std::{io::Read, process::Command};

use enigo::{Enigo, MouseControllable};

fn main() {
    println!("Hello, world!");
    //let mut en = Enigo::new();

    let ml = MouseLocation::get();

    println!("{:?}", ml);
    //    en.mouse_move_to(100, 100);
}

#[derive(Debug)]
struct MouseLocation {
    x: u32,
    y: u32,
    screen: u32,
    window: u32,
}

impl MouseLocation {
    fn get() -> Self {
        let mouse_loc = Command::new("xdotool")
            .arg("getmouselocation")
            .output()
            .expect("Please install xdotool")
            .stdout
            .iter()
            .map(|f| *f as char)
            .collect::<String>();

        let parsed = mouse_loc.split(' ').fold(Vec::new(), |mut i, f| {
            f.split(':').skip(1).for_each(|f| {
                // should just have one item here
                match f.trim().parse::<u32>() {
                    Ok(x) => i.push(x),
                    Err(e) => panic!("{}", e), // FIXME find out how to propagate this properly
                };
            });
            i
        });

        Self {
            x: parsed[0],
            y: parsed[1],
            screen: parsed[2],
            window: parsed[3],
        }
    }
}
