# Belajar Rust

Kumpulan proyek Rust sederhana untuk mempraktikkan konsep dasar bahasa Rust, seperti struktur data, pengolahan input pada terminal, dan algoritma.

## Isi repositori

Repositori ini menggunakan [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) dan berisi dua aplikasi biner:

| Proyek | Deskripsi |
| --- | --- |
| [`apps-1`](apps-1) | Aplikasi terminal sederhana untuk menambah, melihat, mencari, dan menghapus data siswa. |
| [`anagram`](anagram) | Contoh pemeriksaan apakah dua kata adalah anagram. |

## Prasyarat

Pastikan Rust versi stabil sudah terpasang. Proyek ini menggunakan Rust edition 2024.

### Linux, macOS, atau WSL

Pasang Rust melalui `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Tutup lalu buka kembali terminal, kemudian periksa instalasinya:

```bash
rustc --version
cargo --version
```

### Termux (Android)

```bash
pkg update && pkg upgrade -y
pkg install rust
```

Panduan instalasi untuk platform lain tersedia di [situs resmi Rust](https://www.rust-lang.org/tools/install).

## Memulai

Clone repositori, lalu masuk ke direktori proyek:

```bash
git clone <URL-REPOSITORI>
cd module-rust
```

Pastikan seluruh workspace dapat dikompilasi:

```bash
cargo check --workspace
```

## Menjalankan contoh

Jalankan perintah berikut dari direktori root repositori.

### Aplikasi data siswa

```bash
cargo run -p apps-1
```

Aplikasi akan membuka menu interaktif untuk:

- menambah siswa (nama dan NISN);
- menampilkan daftar siswa;
- mencari siswa berdasarkan nama atau NISN; dan
- menghapus siswa berdasarkan nama.

Data disimpan hanya selama program berjalan dan akan kembali kosong ketika aplikasi ditutup. Saat mulai berjalan, aplikasi menyediakan dua data contoh: `asep` dan `agus`.

### Pemeriksa anagram

```bash
cargo run -p anagram
```

Contoh ini membandingkan `kasur` dan `rusak`, lalu menampilkan hasilnya:

```text
kasur dan rusak -> true
```

Implementasi anagram saat ini ditujukan untuk kata ASCII huruf kecil (`a`–`z`).

## Struktur proyek

```text
.
├── Cargo.toml          # Konfigurasi workspace
├── apps-1/             # Aplikasi pengelolaan data siswa di terminal
│   └── src/main.rs
└── anagram/            # Contoh algoritma anagram
    └── src/main.rs
```

## Perintah Cargo yang berguna

```bash
# Memeriksa seluruh workspace tanpa membuat executable rilis
cargo check --workspace

# Menjalankan pemeriksaan kode dengan linter bawaan Rust
cargo clippy --workspace

# Memformat kode sumber
cargo fmt --all
```

## Lisensi

Belum ada lisensi yang ditetapkan untuk repositori ini.
