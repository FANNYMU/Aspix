# 🎨 Aspix

[![Crates.io](https://img.shields.io/crates/v/aspix.svg)](https://docs.rs/crate/aspix/latest)
[![Documentation](https://docs.rs/aspix/badge.svg)](https://docs.rs/aspix)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Aspix adalah library Rust yang powerful untuk mengkonversi gambar menjadi ASCII art. Library ini menyediakan API yang mudah digunakan dengan berbagai opsi kustomisasi untuk menghasilkan ASCII art berkualitas tinggi.

## ✨ Fitur

- 🖼️ Dukungan untuk berbagai format gambar (JPG, PNG, GIF, BMP, dll.)
- 📐 Penyesuaian ukuran output yang fleksibel
- 🎨 Dua mode karakter ASCII (sederhana dan detail)
- 🔄 Opsi untuk membalik hasil (invert)
- ⚡ Penyesuaian brightness dan contrast
- 💾 Kemampuan untuk menyimpan hasil ke file
- 🚀 Performa yang optimal

## 📦 Instalasi

Tambahkan dependency berikut ke file `Cargo.toml` Anda:

```toml
[dependencies]
aspix = "0.1.0"
```

## 🚀 Penggunaan Cepat

```rust
use aspix::AsciiConverter;

fn main() {
    // Buat converter dengan ukuran default
    let converter = AsciiConverter::new(100, 50);

    // Konversi gambar
    match converter.convert("path/to/image.jpg") {
        Ok(ascii_art) => println!("{}", ascii_art),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## 🛠️ Penggunaan Lanjutan

### Konfigurasi Kustom

```rust
use aspix::{AsciiConverter, AsciiConfig};

fn main() {
    // Buat konfigurasi kustom
    let config = AsciiConfig {
        width: 120,
        height: 60,
        use_detailed_chars: true,
        invert: false,
        contrast: 1.2,
        brightness: 1.1,
        ..Default::default()
    };

    // Buat converter dengan konfigurasi kustom
    let converter = AsciiConverter::with_config(config);

    // Konversi dan simpan hasilnya
    if let Ok(ascii_art) = converter.convert("input.jpg") {
        converter.save_to_file(&ascii_art, "output.txt").unwrap();
    }
}
```

### Konversi dari Bytes

```rust
use aspix::AsciiConverter;
use std::fs;

fn main() {
    let converter = AsciiConverter::new(100, 50);
    let image_bytes = fs::read("image.jpg").unwrap();

    if let Ok(ascii) = converter.convert_from_bytes(&image_bytes) {
        println!("{}", ascii);
    }
}
```

## 📝 Dokumentasi

Dokumentasi lengkap tersedia di [docs.rs](https://docs.rs/aspix).

## 🤝 Kontribusi

Kontribusi sangat diterima! Jika Anda memiliki saran, bug report, atau pull request, silakan buat issue atau PR di repository GitHub.

## 📄 Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

## 🙏 Credits

Dibuat dengan ❤️ menggunakan Rust.

- Inspirasi: [ASCII Art](https://en.wikipedia.org/wiki/ASCII_art)
- Image processing: [image-rs](https://github.com/image-rs/image)
