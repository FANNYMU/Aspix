//! # Aspix - ASCII Art Image Converter
//! 
//! Aspix adalah library Rust yang powerful untuk mengkonversi gambar menjadi ASCII art.
//! Library ini menyediakan berbagai fitur untuk mengkustomisasi output, termasuk:
//! 
//! - Konversi gambar ke ASCII art dengan berbagai tingkat detail
//! - Penyesuaian ukuran output
//! - Kontrol atas brightness dan contrast
//! - Dukungan untuk berbagai format gambar
//! - Opsi untuk membalik hasil (invert)
//! - Mode color untuk ASCII art berwarna
//! - Dukungan penggunaan karakter densitas tinggi
//! 
//! ## Contoh Penggunaan Dasar
//! 
//! ```rust
//! use aspix::AsciiConverter;
//! 
//! // Buat converter dengan ukuran default
//! let converter = AsciiConverter::new(100, 50);
//! 
//! // Konversi gambar
//! match converter.convert("path/to/image.jpg") {
//!     Ok(ascii_art) => println!("{}", ascii_art),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//! 
//! ## Penggunaan dengan Konfigurasi Kustom
//! 
//! ```rust
//! use aspix::{AsciiConverter, AsciiConfig};
//! 
//! // Buat konfigurasi kustom
//! let config = AsciiConfig {
//!     width: 120,
//!     height: 60,
//!     use_detailed_chars: true,
//!     use_color: true, 
//!     use_high_density: true,
//!     invert: false,
//!     contrast: 1.2,
//!     brightness: 1.1,
//!     ..Default::default()
//! };
//! 
//! // Buat converter dengan konfigurasi kustom
//! let converter = AsciiConverter::with_config(config);
//! 
//! // Konversi dan simpan hasilnya
//! if let Ok(ascii_art) = converter.convert("input.jpg") {
//!     converter.save_to_file(&ascii_art, "output.html").unwrap();
//! }
//! ```

use image::{DynamicImage, GenericImageView, GrayImage, io::Reader as ImageReader, imageops::FilterType};
use std::path::Path;
use std::fs;

/// Set karakter ASCII dasar yang digunakan untuk konversi, diurutkan dari gelap ke terang.
/// Cocok untuk output yang sederhana dan jelas.
const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

/// Set karakter ASCII yang lebih detail untuk hasil yang lebih halus.
/// Menyediakan gradasi yang lebih baik antara area gelap dan terang.
const DETAILED_ASCII_CHARS: &[u8] = b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

/// Set karakter densitas tinggi untuk hasil yang sangat detail.
/// Menggunakan kombinasi karakter untuk menciptakan berbagai tingkat gelap-terang.
const HIGH_DENSITY_CHARS: &[&str] = &[
    "█", "▓", "▒", "░", "▄", "▀", "■", "▪", "●", "◆", 
    "◉", "◍", "◎", "○", "☉", "◌", "◊", "♦", "♢", "•", 
    ".", " "
];

/// Konfigurasi untuk mengatur perilaku konversi ASCII.
/// 
/// Struct ini memungkinkan kustomisasi penuh atas proses konversi,
/// termasuk dimensi output, tingkat detail, dan penyesuaian gambar.
/// 
/// # Fields
/// 
/// * `width` - Lebar output ASCII dalam karakter
/// * `height` - Tinggi output ASCII dalam baris
/// * `use_detailed_chars` - Menggunakan set karakter detail untuk hasil yang lebih halus
/// * `use_high_density` - Menggunakan karakter densitas tinggi (Uni3ode blocks) untuk detail ekstrim
/// * `use_color` - Menghasilkan output berwarna (format HTML)
/// * `color_saturation` - Intensitas warna (0.0 - 1.0)
/// * `invert` - Membalik hasil konversi (gelap menjadi terang dan sebaliknya)
/// * `contrast` - Nilai contrast (1.0 adalah normal, >1.0 menambah contrast, <1.0 mengurangi)
/// * `brightness` - Nilai brightness (1.0 adalah normal, >1.0 lebih terang, <1.0 lebih gelap)
/// * `scale` - Skala resolusi internal (lebih tinggi = lebih detail, default 1.0)
#[derive(Debug, Clone)]
pub struct AsciiConfig {
    pub width: u32,
    pub height: u32,
    pub use_detailed_chars: bool,
    pub use_high_density: bool,
    pub use_color: bool,
    pub color_saturation: f32,
    pub invert: bool,
    pub contrast: f32,
    pub brightness: f32,
    pub scale: f32,
}

impl Default for AsciiConfig {
    /// Membuat konfigurasi default dengan nilai yang umum digunakan.
    /// 
    /// # Returns
    /// 
    /// Mengembalikan `AsciiConfig` dengan nilai default:
    /// * width: 100
    /// * height: 50
    /// * use_detailed_chars: false
    /// * use_high_density: false
    /// * use_color: false
    /// * color_saturation: 0.7
    /// * invert: false
    /// * contrast: 1.0
    /// * brightness: 1.0
    /// * scale: 1.0
    fn default() -> Self {
        Self {
            width: 100,
            height: 50,
            use_detailed_chars: false,
            use_high_density: false,
            use_color: false,
            color_saturation: 0.7,
            invert: false,
            contrast: 1.0,
            brightness: 1.0,
            scale: 1.0,
        }
    }
}

/// Struct utama untuk mengkonversi gambar menjadi ASCII art.
/// 
/// `AsciiConverter` menyediakan metode-metode untuk mengkonversi gambar
/// menjadi ASCII art dengan berbagai opsi kustomisasi.
pub struct AsciiConverter {
    config: AsciiConfig,
}

impl AsciiConverter {
    /// Membuat instance baru `AsciiConverter` dengan ukuran tertentu dan konfigurasi default lainnya.
    /// 
    /// # Arguments
    /// 
    /// * `width` - Lebar output ASCII dalam karakter
    /// * `height` - Tinggi output ASCII dalam baris
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use aspix::AsciiConverter;
    /// 
    /// let converter = AsciiConverter::new(80, 40);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            config: AsciiConfig {
                width,
                height,
                ..Default::default()
            }
        }
    }

    /// Membuat instance baru dengan konfigurasi kustom.
    /// 
    /// # Arguments
    /// 
    /// * `config` - Struct `AsciiConfig` yang berisi semua pengaturan kustom
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use aspix::{AsciiConverter, AsciiConfig};
    /// 
    /// let config = AsciiConfig {
    ///     width: 120,
    ///     height: 60,
    ///     use_detailed_chars: true,
    ///     use_high_density: true,
    ///     use_color: true,
    ///     ..Default::default()
    /// };
    /// 
    /// let converter = AsciiConverter::with_config(config);
    /// ```
    pub fn with_config(config: AsciiConfig) -> Self {
        Self { config }
    }

    /// Mengkonversi gambar dari path file menjadi ASCII art.
    /// 
    /// # Arguments
    /// 
    /// * `image_path` - Path ke file gambar yang akan dikonversi
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - ASCII art dalam bentuk string jika berhasil
    /// * `Err(String)` - Pesan error jika gagal
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use aspix::AsciiConverter;
    /// 
    /// let converter = AsciiConverter::new(100, 50);
    /// match converter.convert("image.jpg") {
    ///     Ok(ascii) => println!("{}", ascii),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn convert(&self, image_path: &str) -> Result<String, String> {
        let img = ImageReader::open(&Path::new(image_path))
            .map_err(|e| format!("Gagal membuka gambar: {}", e))?
            .decode()
            .map_err(|e| format!("Gagal mendekode gambar: {}", e))?;

        self.process_image(&img)
    }

    /// Mengkonversi data bytes gambar menjadi ASCII art.
    /// 
    /// Berguna untuk memproses gambar dari memory atau stream data.
    /// 
    /// # Arguments
    /// 
    /// * `bytes` - Data bytes gambar
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - ASCII art dalam bentuk string jika berhasil
    /// * `Err(String)` - Pesan error jika gagal
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use aspix::AsciiConverter;
    /// 
    /// let converter = AsciiConverter::new(100, 50);
    /// let image_bytes = std::fs::read("image.jpg").unwrap();
    /// if let Ok(ascii) = converter.convert_from_bytes(&image_bytes) {
    ///     println!("{}", ascii);
    /// }
    /// ```
    pub fn convert_from_bytes(&self, bytes: &[u8]) -> Result<String, String> {
        let img = image::load_from_memory(bytes)
            .map_err(|e| format!("Gagal memuat gambar dari bytes: {}", e))?;

        self.process_image(&img)
    }

    /// Memproses gambar DynamicImage menjadi ASCII art.
    /// 
    /// Fungsi internal yang melakukan konversi utama.
    fn process_image(&self, img: &DynamicImage) -> Result<String, String> {
        let target_width = (self.config.width as f32 * self.config.scale) as u32;
        let target_height = (self.config.height as f32 * self.config.scale) as u32;
        
        let mut processed = img.resize_exact(
            target_width,
            target_height,
            FilterType::Lanczos3
        );

        processed = self.apply_image_adjustments(&processed);
        
        if self.config.use_color {
            Ok(self.image_to_colored_ascii(&processed))
        } else {
            let grayscale = processed.into_luma8();
            Ok(self.image_to_ascii(&grayscale))
        }
    }

    /// Mengkonversi gambar grayscale menjadi string ASCII.
    /// 
    /// Fungsi internal yang menghasilkan ASCII art dari gambar grayscale.
    fn image_to_ascii(&self, image: &GrayImage) -> String {
        let mut ascii_output = String::new();
        
        if self.config.use_high_density {
            // Gunakan karakter densitas tinggi
            for y in 0..self.config.height {
                for x in 0..self.config.width {
                    let scale_factor = self.config.scale as u32;
                    let base_x = (x * scale_factor) as u32;
                    let base_y = (y * scale_factor) as u32;
                    
                    // Hitung rata-rata brightness untuk blok piksel
                    let mut total_brightness = 0.0;
                    let mut count = 0.0;
                    
                    for dy in 0..scale_factor {
                        for dx in 0..scale_factor {
                            if base_x + dx < image.width() && base_y + dy < image.height() {
                                let pixel = image.get_pixel(base_x + dx, base_y + dy);
                                let mut brightness = pixel[0] as f32 / 255.0;
                                
                                if self.config.invert {
                                    brightness = 1.0 - brightness;
                                }
                                
                                total_brightness += brightness;
                                count += 1.0;
                            }
                        }
                    }
                    
                    let avg_brightness = if count > 0.0 { total_brightness / count } else { 0.0 };
                    let index = (avg_brightness * (HIGH_DENSITY_CHARS.len() - 1) as f32) as usize;
                    ascii_output.push_str(HIGH_DENSITY_CHARS[index]);
                }
                ascii_output.push('\n');
            }
        } else {
            // Gunakan karakter ASCII normal atau detail
            let chars = if self.config.use_detailed_chars {
                DETAILED_ASCII_CHARS
            } else {
                ASCII_CHARS
            };

            for y in 0..self.config.height {
                for x in 0..self.config.width {
                    let scale_factor = self.config.scale as u32;
                    let base_x = (x * scale_factor) as u32;
                    let base_y = (y * scale_factor) as u32;
                    
                    // Hitung rata-rata brightness untuk blok piksel
                    let mut total_brightness = 0.0;
                    let mut count = 0.0;
                    
                    for dy in 0..scale_factor {
                        for dx in 0..scale_factor {
                            if base_x + dx < image.width() && base_y + dy < image.height() {
                                let pixel = image.get_pixel(base_x + dx, base_y + dy);
                                let mut brightness = pixel[0] as f32 / 255.0;
                                
                                if self.config.invert {
                                    brightness = 1.0 - brightness;
                                }
                                
                                total_brightness += brightness;
                                count += 1.0;
                            }
                        }
                    }
                        
                    let avg_brightness = if count > 0.0 {
                        total_brightness / count
                    } else { 
                        0.0 
                    };
                    
                    let index = (avg_brightness * (chars.len() - 1) as f32) as usize;
                    ascii_output.push(chars[index] as char);
                }
                ascii_output.push('\n');
            }
        }
        
        ascii_output
    }
    
    /// Mengkonversi gambar berwarna menjadi ASCII art dengan warna.
    /// 
    /// Menghasilkan HTML dengan karakter ASCII yang berwarna.
    fn image_to_colored_ascii(&self, image: &DynamicImage) -> String {
        let mut html_output = String::from(
            "<!DOCTYPE html>\n<html>\n<head>\n<style>\n\
            body { background-color: #000; margin: 0; padding: 10px; }\n\
            pre { font-family: monospace; font-size: 10px; line-height: 0.9; }\n\
            </style>\n</head>\n<body>\n<pre>\n"
        );
        
        let chars = if self.config.use_detailed_chars {
            DETAILED_ASCII_CHARS
        } else if self.config.use_high_density {
            // Menggunakan blok karakter ASCII untuk densidade tinggi
            b"@%#*+=-:. "
        } else {
            ASCII_CHARS
        };
        
        for y in 0..self.config.height {
            for x in 0..self.config.width {
                let scale_factor = self.config.scale as u32;
                let base_x = (x * scale_factor) as u32;
                let base_y = (y * scale_factor) as u32;
                
                // Hitung rata-rata warna dan brightness untuk blok piksel
                let mut total_r = 0.0;
                let mut total_g = 0.0;
                let mut total_b = 0.0;
                let mut total_brightness = 0.0;
                let mut count = 0.0;
                
                for dy in 0..scale_factor {
                    for dx in 0..scale_factor {
                        if base_x + dx < image.width() && base_y + dy < image.height() {
                            let pixel = image.get_pixel(base_x + dx, base_y + dy);
                            let r = pixel[0] as f32 / 255.0;
                            let g = pixel[1] as f32 / 255.0;
                            let b = pixel[2] as f32 / 255.0;
                            
                            // Brightness menggunakan formula standar (R*0.3 + G*0.59 + B*0.11)
                            let brightness = r * 0.3 + g * 0.59 + b * 0.11;
                            
                            total_r += r;
                            total_g += g;
                            total_b += b;
                            total_brightness += brightness;
                            count += 1.0;
                        }
                    }
                }
                
                if count > 0.0 {
                    let avg_r = total_r / count;
                    let avg_g = total_g / count;
                    let avg_b = total_b / count;
                    let avg_brightness = if self.config.invert {
                        1.0 - (total_brightness / count)
                    } else {
                        total_brightness / count
                    };
                    
                    // Hitung karakter berdasarkan brightness
                    let char_index = (avg_brightness * (chars.len() - 1) as f32) as usize;
                    let character = if self.config.use_high_density && char_index < HIGH_DENSITY_CHARS.len() {
                        HIGH_DENSITY_CHARS[char_index].to_string()
                    } else {
                        let char_bytes = &[chars[char_index]];
                        std::str::from_utf8(char_bytes)
                            .map(|s| s.to_string())
                            .unwrap_or_else(|_| " ".to_string())
                    };
                    
                    // Terapkan saturasi warna
                    let sat = self.config.color_saturation;
                    let r = ((avg_r * sat + (1.0 - sat) * 0.5) * 255.0) as u8;
                    let g = ((avg_g * sat + (1.0 - sat) * 0.5) * 255.0) as u8;
                    let b = ((avg_b * sat + (1.0 - sat) * 0.5) * 255.0) as u8;
                    
                    // Tambahkan karakter dengan warna ke output HTML
                    html_output.push_str(&format!("<span style=\"color:rgb({},{},{})\">{}</span>", r, g, b, character));
                }
            }
            html_output.push_str("<br/>\n");
        }
        
        html_output.push_str("</pre>\n</body>\n</html>");
        html_output
    }

    /// Menyimpan hasil ASCII art ke file.
    /// 
    /// # Arguments
    /// 
    /// * `ascii` - String ASCII art yang akan disimpan
    /// * `output_path` - Path file output
    /// 
    /// # Returns
    /// 
    /// * `Ok(())` - Jika berhasil menyimpan
    /// * `Err(String)` - Pesan error jika gagal
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use aspix::AsciiConverter;
    /// 
    /// let converter = AsciiConverter::new(100, 50);
    /// if let Ok(ascii) = converter.convert("input.jpg") {
    ///     converter.save_to_file(&ascii, "output.txt").unwrap();
    /// }
    /// ```
    pub fn save_to_file(&self, ascii: &str, output_path: &str) -> Result<(), String> {
        fs::write(output_path, ascii)
            .map_err(|e| format!("Gagal menyimpan file: {}", e))
    }

    /// Menerapkan penyesuaian contrast dan brightness pada gambar.
    /// 
    /// Fungsi internal untuk memodifikasi gambar sebelum konversi ke ASCII.
    fn apply_image_adjustments(&self, img: &DynamicImage) -> DynamicImage {
        let mut adjusted = img.to_rgba8();
        
        // Iterasi melalui setiap pixel
        for pixel in adjusted.pixels_mut() {
            // Proses setiap channel warna (R, G, B)
            for c in 0..3 {
                // Normalisasi nilai warna ke range 0.0 - 1.0
                let mut color = pixel[c] as f32 / 255.0;
                
                // Terapkan penyesuaian contrast
                // Formula: (color - 0.5) * contrast + 0.5
                // Dijaga dalam range 0.0 - 1.0
                color = ((color - 0.5) * self.config.contrast + 0.5)
                    .clamp(0.0, 1.0);
                
                // Terapkan penyesuaian brightness
                // Formula: color * brightness
                // Dijaga dalam range 0.0 - 1.0
                color = (color * self.config.brightness)
                    .clamp(0.0, 1.0);
                
                // Konversi kembali ke range 0-255
                pixel[c] = (color * 255.0) as u8;
            }
        }

        // Kembalikan gambar yang telah disesuaikan
        DynamicImage::ImageRgba8(adjusted)
    }
}
