use std::fs::File;
use std::io::{Write, BufWriter, Result};
use crate::color::Color;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32;

pub fn write_bmp_file(
    file_path: &str,
    buffer: &[Color],
    width: usize,
    height: usize,
) -> Result<()>{
    let mut file = BufWriter::new(File::create(file_path)?);

    write_bmp_header(&mut file, width, height)?;
    write_bmp_data(&mut file, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>,
    width: usize,
    height: usize,
) -> Result<()>{
    let file_size = BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8);
    let pixel_data_size = width * height * BMP_BITS_PER_PIXEL / 8;

    // BMP signature
    file.write_all(b"BM")?;

    // File size
    file.write_all(&(file_size as u32).to_le_bytes())?;

    // Reserved bytes and pixel offset
    file.write_all(&[0u8; 4])?;
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;

    // Header size, image width, and image height
    file.write_all(&(BMP_HEADER_SIZE as u32).to_le_bytes())?;
    file.write_all(&(width as u32).to_le_bytes())?;
    file.write_all(&(height as u32).to_le_bytes())?;

    // Color planes and bits per pixel
    file.write_all(&(1u16).to_le_bytes())?;
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;

    // Compression method (none), pixel data size, and resolution
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;

    // Number of colors and important colors
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;

    Ok(())
}

fn write_bmp_data(
    file: &mut BufWriter<File>,
    buffer: &[Color],
    width: usize,
    height: usize,
) -> Result<()>{

    // Calculate padding size for each row
    let padding = (4 - ((width * BMP_BITS_PER_PIXEL / 8) % 4)) % 4;

    // Iterate over rows in reverse order (BMP is stored bottom to top)
    for y in (0..height).rev() {
        for x in 0..width {
            let color = &buffer[y * width + x];

            let b = color.blue;
            let g = color.green;
            let r = color.red;

            // BMP format stores pixels as BGR (Blue-Green-Red) instead of RGB
            let pixel_data = [
                b as u8,
                g as u8,
                r as u8,
                0u8, // Alpha channel, set to 0 for BMP
            ];

            // Write pixel data to file
            file.write_all(&pixel_data)?;
        }

        // Pad each row to a multiple of 4 bytes
        let padding_data = [0u8; 4];
        file.write_all(&padding_data[0..padding])?;
    }

    Ok(())
}

