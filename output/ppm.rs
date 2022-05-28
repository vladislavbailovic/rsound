use std::fs::File;
use std::io::{Write,BufWriter};

const MIN_WIDTH: i32 = 800;
const MAX_HEIGHT: i32 = 600;
const VERTICAL_PADDING: i32 = 50;

pub fn save(sequence: &[f32]) -> std::io::Result<()> {
    let ppm_width: i32 = sequence_width(sequence);
    let ppm_height: i32 = MAX_HEIGHT;

    let max_size = 3 * ppm_width * ppm_height;
    let mut buffer = vec![0; max_size as usize];

    let scale = ppm_width as f32 / sequence.len() as f32;
    let amplitude_height = MAX_HEIGHT - 2*VERTICAL_PADDING;

    let mut i = 0;
    for sample in sequence {
        let y = ((sample+1.0) * amplitude_height as f32 / 2.0) as i32 + VERTICAL_PADDING;
        let x = (i as f32 * scale) as i32;
        let offset = ((y * ppm_width * 3) + (x * 3)) as usize;

        // println!("i: {}, scale: {}, sample: {}, x: {}, y: {}, offset: {}, within range: {:?}", i, scale, sample, x, y, offset, max_size > offset as i32);

        if offset >= max_size as usize {
            continue;
        }

        buffer[offset] = 255;
        buffer[offset+1] = 255;
        buffer[offset+2] = 0;

        i+=1;
    }

    let mut p = BufWriter::new(File::create("foo.ppm")?);
    p.write(format!("P6 {} {} 255\n", ppm_width, ppm_height).as_bytes())?;
    p.write(&buffer)?;

    Ok(())
}

fn sequence_width(sequence: &[f32]) -> i32 {
    if sequence.len() as i32 % MIN_WIDTH < 10 {
        MIN_WIDTH
    } else {
        sequence.len() as i32 / 10
    }
}
