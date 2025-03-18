use aspix::AsciiConverter;
use std::fs;

fn main() {
    let converter = AsciiConverter::new(100, 50);
    
    // Gunakan path relatif ke root proyek
    match fs::read("examples/image.png") {
        Ok(image_bytes) => {
            match converter.convert_from_bytes(&image_bytes) {
                Ok(ascii) => println!("{}", ascii),
                Err(e) => eprintln!("Error konversi: {}", e)
            }
        },
        Err(e) => eprintln!("Error membaca file: {}", e)
    }
} 