use std::env;
use std::fs::File;
use std::path::Path;
use anyhow::{Result, Context, anyhow};
use std::io::BufReader;
use jpeg_decoder::Decoder;
fn main() {
    let mut arg_iter = env::args();
    if let Some(_) = arg_iter.next() {
        if let Some(path) = arg_iter.next() {
            match run(path.as_ref()) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Exploded: {}", e);
                    std::process::exit(1);
                }

            }
            return;
        }
    }
    eprintln!("Must set a filename");
    std::process::exit(1);
}

fn run(path: &str) -> Result<()> {
    let file = File::open(Path::new(path)).context("failed to open file")?;
    let mut decoder = Decoder::new(BufReader::new(file));
    decoder.scale(640, 480).context("failed to scale image")?;
    let pixels = decoder.decode().context("failed to decode image")?;
    let info = decoder.info().ok_or(anyhow!("png had no pixels"))?;
    
    let pixels_per_row = info.width * 3;
    dbg!(pixels_per_row);

    pixels.chunks_exact(pixels_per_row.into()).for_each(|row |{
        let str = row.chunks_exact(3)
            .map(|buf| (buf[0], buf[1], buf[2]))
            .map(brightness)
            .map(brightness_to_char)
            .collect::<String>();
        // dbg!(row.len());
        println!("{}", str);
    });
    
    Ok(())
}

fn brightness(pixel: (u8, u8, u8)) -> u8 {
    let (r, g, b) = pixel;
    let res: f32 = 0.21f32 * f32::from(r) + 0.72 * f32::from(g) + 0.07 * f32::from(b);
    res as u8
}

const BRIGHTNESS_TABLE: &'static str = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const BRIGHTNESS_SCALE: f32 = 26f32;

fn brightness_to_char(brightness: u8) -> char {
    let i = (f32::from(brightness) / BRIGHTNESS_SCALE).round() as usize;
    BRIGHTNESS_TABLE.chars().nth(i).unwrap()
}