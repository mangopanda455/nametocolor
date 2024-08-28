use clap::Parser;
use colored::*;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    first: String,
    middle: String,
    last: String,
}

fn name_to_color(first: String, middle: String, last: String) -> (i32, i32, i32) {
    let skibidi = HashMap::from([
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
        ("d".to_string(), 4),
        ("e".to_string(), 5),
        ("f".to_string(), 6),
        ("g".to_string(), 7),
        ("h".to_string(), 8),
        ("i".to_string(), 9),
        ("j".to_string(), 10),
        ("k".to_string(), 11),
        ("l".to_string(), 12),
        ("m".to_string(), 13),
        ("n".to_string(), 14),
        ("o".to_string(), 15),
        ("p".to_string(), 16),
        ("q".to_string(), 17),
        ("r".to_string(), 18),
        ("s".to_string(), 19),
        ("t".to_string(), 20),
        ("u".to_string(), 21),
        ("v".to_string(), 22),
        ("w".to_string(), 23),
        ("x".to_string(), 24),
        ("y".to_string(), 25),
        ("z".to_string(), 26),
    ]);

    let firstl = first.to_lowercase();
    let middlel = middle.to_lowercase();
    let lastl = last.to_lowercase();

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for i in firstl.chars() {
        if let Some(&value) = skibidi.get(&i.to_string()) {
            r += value;
        }
    }
    for i in middlel.chars() {
        if let Some(&value) = skibidi.get(&i.to_string()) {
            g += value;
        }
    }
    for i in lastl.chars() {
        if let Some(&value) = skibidi.get(&i.to_string()) {
            b += value;
        }
    }
    r *= 4;
    g *= 4;
    b *= 4;
    while r > 255 {
        r -= 256;
    }
    while g > 255 {
        g -= 256;
    }
    while b > 255 {
        b -= 256;
    }
    return (r, g, b);
}

fn main() {
    let stuff = Cli::parse();
    let first = stuff.first;
    let middle = stuff.middle;
    let last = stuff.last;
    let (r, g, b) = name_to_color(first.clone(), middle.clone(), last.clone());
    println!("{}, {}, {}", r, g, b);
    println!(
        "{} {} {}",
        first.custom_color(CustomColor {
            r: r as u8,
            g: g as u8,
            b: b as u8
        }),
        middle.custom_color(CustomColor {
            r: r as u8,
            g: g as u8,
            b: b as u8
        }),
        last.custom_color(CustomColor {
            r: r as u8,
            g: g as u8,
            b: b as u8
        })
    );
    println!(
        "{}",
        String::from("██████████\n██████████\n██████████\n██████████\n██████████").custom_color(
            CustomColor {
                r: r as u8,
                g: g as u8,
                b: b as u8
            }
        )
    )
}
