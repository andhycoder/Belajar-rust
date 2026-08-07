// TODO:
// fitur yang ditambahkan:
// 1. menu nenambahkan siswa ke dalam sebuah array.
// 2. menu menampilkan data siswa.
// 3. menu mencari berdasarkan nama/nisn siswa.
// 4. menu menghapus siswa.

use std::io::stdin;

/// struct untuk menyimpan data.
struct KartuSiswa {
    data: Vec<(String, i32)>,
}

impl KartuSiswa {
    /// fungsi untuk membuat data.
    const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// fungsi untuk menambahkan data siswa ke dalam data
    fn tambah_siswa(&mut self, name: String, nisn: i32) {
        let data = (name, nisn);

        let result_name = self.data.iter().find(|x| data.0 == x.0);
        let result_nisn = self.data.iter().find(|&x| data.1 == x.1);

        if self.data.is_empty() {
            self.data.push(data);
        } else {
            for i in &self.data {
                if result_name == Some(i) {
                    println!("nama tidak boleh sama");
                    break;
                } else if result_nisn == Some(i) {
                    println!("nisn tidak boleh sama");
                    break;
                }

                if result_name != Some(i) || result_nisn != Some(i) {
                    self.data.push(data);
                    break;
                }
            }
        }
    }

    /// fungsi untuk mencari data siswa menggunakan filter.
    fn cari_siswa(&self, filter: Option<&(String, i32)>) {
        for i in &self.data {
            if filter == Some(i) {
                println!("nama : {}", i.0);
                println!("nisn : {}", i.1);
                break;
            }
        }
        if filter.is_none() {
            println!("query tidak ditemukan");
        }
    }

    /// fungsi untuk menghapus data siswa.
    fn hapus_siswa(&mut self, search: &str) {
        let filter = self.data.iter().find(|&x| search.trim() == x.0);
        for (count, i) in self.data.iter().enumerate() {
            if filter == Some(i) {
                self.data.remove(count);
                break;
            }
        }
    }
}

/// sebuah fungsi menampilkan menu cli sederhana
fn menu(kartu: &mut KartuSiswa) {
    let mut input = String::new();
    loop {
        println!("+--------------------------+");
        println!("|        menu siswa        |");
        println!("+--------------------------+");
        println!("1. tambah siswa");
        println!("2. list siswa");
        println!("3. cari siswa");
        println!("4. hapus siswa");
        println!("5. keluar");

        println!("\nPilih (1-5):");

        stdin().read_line(&mut input).unwrap();

        if let Ok(s) = input.trim().parse::<i16>() {
            if s <= 5 && s > 0 {
                if s == 1 {
                    menu_tambah_siswa(kartu);
                    break;
                } else if s == 2 {
                    menu_list_siswa(kartu);
                    break;
                } else if s == 3 {
                    menu_cari_siswa(kartu);
                    break;
                } else if s == 4 {
                    menu_hapus_siswa(kartu);
                    break;
                }
            } else {
                println!("pilih 1-5");
            }
            break;
        }
        println!("Input invalid");
        input.clear();
    }
}

/// menu untuk menambahkan siswa ke dalam list
fn menu_tambah_siswa(kartu: &mut KartuSiswa) {
    let mut nama = String::new();
    let mut nisn = String::new();
    let mut input = String::new();

    println!("+--------------------------+");
    println!("|      tambah siswa        |");
    println!("+--------------------------+");
    println!("Masukkan nama : ");
    stdin().read_line(&mut nama).expect("nama invalid");

    println!("Masukkan nisn : ");
    stdin().read_line(&mut nisn).expect("nisn invalid");

    let nisn: i32 = nisn.trim().parse().unwrap();
    kartu.tambah_siswa(nama.trim().to_string(), nisn);

    println!("\ntambah siswa: ");
    stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        menu_tambah_siswa(kartu);
    } else if input.trim() == "n" {
        menu(kartu);
    }
}

/// menu untuk menampilkan daftar siswa
fn menu_list_siswa(kartu: &mut KartuSiswa) {
    let input = 'y';
    println!("+--------------------------+");
    println!("|       list siswa         |");
    println!("+--------------------------+");

    for i in &kartu.data {
        println!("nama : {}", i.0);
        println!("nisn : {}", i.1);
    }

    println!("\npencet apa saja untuk kembali");
    stdin()
        .read_line(&mut input.to_string())
        .expect("Input harus y/n");

    if input == 'y' {
        menu(kartu);
    }
}

/// menu untuk mencari data siswa
fn menu_cari_siswa(kartu: &mut KartuSiswa) {
    let mut pilihan = String::new();
    let mut input = String::new();

    println!("+--------------------------+");
    println!("|        cari siswa        |");
    println!("+--------------------------+");

    println!("1. berdasarkan nama");
    println!("2. berdasarkan nisn");
    println!("3. kembali ke menu");

    println!("Pilih 1-3 : ");
    stdin().read_line(&mut pilihan).expect("pilih 1-3");

    if let Ok(p) = pilihan.trim().parse::<i16>() {
        if p == 1 {
            cari_berdasarkan_nama(kartu);
        } else if p == 2 {
            cari_berdasarkan_nisn(kartu);
        } else if p == 3 {
            menu(kartu);
        } else {
            println!("pilih 1-3");
            menu_cari_siswa(kartu);
        }
    } else {
        println!("invalid");
    }

    println!("\ncari siswa (y/n)");
    stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        menu_cari_siswa(kartu);
    } else if input.trim() == "n" {
        menu(kartu);
    }
}

/// query logic untuk mencari data siswa berdasarkan nama
fn cari_berdasarkan_nama(kartu: &KartuSiswa) {
    let mut search_input = String::new();

    println!("Masukkan nama : ");
    stdin().read_line(&mut search_input).expect("nama invalid");

    let filter = kartu.data.iter().find(|&x| search_input.trim() == x.0);

    kartu.cari_siswa(filter);
}

/// query logic untuk mencari data siswa berdasarkan nisn
fn cari_berdasarkan_nisn(kartu: &KartuSiswa) {
    let mut search_input = String::new();

    println!("Masukkan nisn : ");
    stdin().read_line(&mut search_input).expect("nisn invalid");

    let search_input: i32 = search_input.trim().parse().unwrap();

    let filter = kartu.data.iter().find(|&x| search_input == x.1);

    kartu.cari_siswa(filter);
}

/// menu untuk menghapus data siswa
fn menu_hapus_siswa(kartu: &mut KartuSiswa) {
    let mut nama = String::new();
    let mut input = String::new();

    println!("+--------------------------+");
    println!("|        hapus siswa       |");
    println!("+--------------------------+");
    println!("Masukkan nama yg akan dihapus : ");
    stdin().read_line(&mut nama).unwrap();

    kartu.hapus_siswa(&nama);

    println!("hapus siswa lagi (y/n)");
    stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        menu_hapus_siswa(kartu);
    } else if input.trim() == "n" {
        menu(kartu);
    }
}

/// main entry program
fn main() {
    let mut kartu = KartuSiswa::new();
    kartu.tambah_siswa(String::from("asep"), 80);
    kartu.tambah_siswa(String::from("agus"), 81);

    menu(&mut kartu);
}
