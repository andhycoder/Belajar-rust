// TODO:
// fitur yang ditambahkan:
// 1. membuat struct `KartuSiswa`
// 2. membuat getter & setter pads `KartuSiswa`

use std::io::stdin;

enum KartuHandler {
    SiswaTidakDitemukan,
    NisnInvalid,
}

#[derive(Debug)]
struct KartuSiswa {
    data: Vec<(String, i32)>,
}

impl KartuSiswa {
    const fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn tambah_siswa(&mut self, name: String, nisn: i32) {
        let data = (name, nisn);
        self.data.push(data);
    }
}

/// sebuah fungsi menampilkan menu cli sederhana
fn menu(kartu: &mut KartuSiswa) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    loop {
        println!("+---------- Menu ----------+");
        println!("|1. tambah siswa           |");
        println!("|2. list siswa             |");
        println!("|3. cari siswa             |");
        println!("|4. hapus siswa            |");
        println!("|5. keluar                 |");
        println!("+--------------------------+");
        println!("\nPilih (1-5):");

        stdin().read_line(&mut input)?;

        match input.trim().parse::<i16>() {
            Ok(s) => {
                if s <= 5 && s > 0 {
                    if s == 1 {
                        menu_tambah_siswa(kartu);
                        break;
                    } else if s == 2 {
                        menu_list_siswa(kartu);
                    }
                } else {
                    println!("error");
                }
                break;
            }
            Err(e) => {
                println!("{e}: Input invalid");
                input.clear();
            }
        }
    }
    Ok(())
}

/// main entry program
fn main() {
    let mut kartu = KartuSiswa::new();
    kartu.tambah_siswa(String::from("asep"), 80);
    kartu.tambah_siswa(String::from("agus"), 81);

    let _ = menu(&mut kartu);
}

/// menu untuk menambahkan siswa ke dalam list
fn menu_tambah_siswa(kartu: &mut KartuSiswa) {
    let mut nama = String::new();
    let mut nisn = String::new();
    let input: char = 'y';

    loop {
        println!("+--------------------------+");
        println!("|      tambah siswa        |");
        println!("+--------------------------+");
        println!("Masukkan nama : ");
        match stdin().read_line(&mut nama) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}");
                break;
            }
        }

        println!("Masukkan nisn : ");
        match stdin().read_line(&mut nisn) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}");
                break;
            }
        }

        let nisn: i32 = nisn.trim().parse().unwrap();
        if !nama.is_empty() || nisn != 0 {
            kartu.tambah_siswa(nama.trim().to_string(), nisn);
            println!("{nama} berhasil ditambahkan ke dalam data");
            println!("\nKembali ke menu (y/n)");
            if stdin().read_line(&mut input.to_string()).is_ok() {} else {
                break;
            }

            if input == 'y' {
                let _ = menu(kartu);
            } else if input == 'n' {
                break;
            }
            break;
        }
        println!("nama/nisn tidak boleh kosong");
    }
}

/// menu untuk menampilkan daftar siswa
fn menu_list_siswa(kartu: &KartuSiswa) {
    println!("+--------------------------+");
    println!("|        list siswa        |");
    println!("+--------------------------+");

    for i in &kartu.data {
        println!("nama : {}", i.0);
        println!("nisn : {}", i.1);
    }
}
