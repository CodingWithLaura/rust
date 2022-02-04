use std::fs::File;
use std::io;
use std::io::Write;

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?; //? to ask if its there, return Result
    write!(file, "P6\n{} {} 255\n", width, height)?;
    Ok(())
}

fn main() {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;
    const OUTPUT_PATH: &str = "output.ppm";
    let mut pixels = [0u32; WIDTH * HEIGHT];
    save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT);
}
