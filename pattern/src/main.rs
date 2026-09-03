// *****
// *****
// *****
// *****
// *****

fn kotak() {
    for _ in 0..5 {
        for _ in 0..5 {
            print!("*");
        }
        println!();
    }
}

// *
// **
// ***
// ****
// *****

fn segitiga() {
    let mut lines: i32 = 0;
    for i in 0..5 {
        if i <= lines {
            for _ in 0..=lines {
                print!("*");
            }
            lines += 1;
        }
        println!();
    }
}

// *****
// ****
// ***
// **
// *

fn segitiga_terbalik() {
    let mut lines: i32 = 5;
    for i in (0..=5).rev() {
        if i >= lines {
            for _ in 0..lines {
                print!("*");
            }
            lines -= 1;
        }
        println!();
    }
}

/// main entry
fn main() {
    println!("pattern kotak : ");
    kotak();

    println!("\npattern segitiga : ");
    segitiga();

    println!("\npattern segitiga terbalik : ");
    segitiga_terbalik();
}

#[test]
fn kotak_pattern() {
    kotak();
}

#[test]
fn segitiga_pattern() {
    segitiga();
}

#[test]
fn segitiga_terbalik_pattern() {
    segitiga_terbalik();
}
