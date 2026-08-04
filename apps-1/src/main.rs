// TODO:
// fitur yang ditambahkan:
// 1. membuat struct `KartuSiswa`
// 2. membuat getter & setter pads `KartuSiswa`

#[derive(Debug)]
struct KartuSiswa {
    name: String,
    nisn: i32,
}

impl KartuSiswa {
    const fn new(name: String, nisn: i32) -> Self {
        Self { name, nisn }
    }

    /// getter method for name
    const fn name(&self) -> &String {
        &self.name
    }

    /// setter method for name
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// getter method for nisn
    const fn nisn(&self) -> i32 {
        self.nisn
    }

    /// setter method for nisn
    const fn set_nisn(&mut self, nisn: i32) {
        self.nisn = nisn;
    }

    fn info(&self) {
        println!("nama : {}", self.name());
        println!("nisn : {}", self.nisn());
    }
}

fn main() {
    #[allow(clippy::unreadable_literal)]
    let mut kartu = KartuSiswa::new(String::new(), 0);

    kartu.set_name("bambang".to_string());
    #[allow(clippy::unreadable_literal)]
    kartu.set_nisn(308442715);

    kartu.info();
}
