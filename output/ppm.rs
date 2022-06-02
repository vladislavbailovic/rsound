use std::fs::File;
use std::io::{Write,BufWriter};

const MIN_WIDTH: i32 = 800;
const MAX_HEIGHT: i32 = 600;
const VERTICAL_PADDING: i32 = 50;

#[derive(Debug)]
pub enum Align {
    Center,
    Bottom,
}

#[derive(Debug)]
pub enum Snap {
    Height,
    Width,
}

#[derive(Debug)]
pub struct Graph {
    sequence: Vec<f32>,
    width: i32,
    height: i32,
    align: Align,
    snap: Snap,
}

impl Graph {
    pub fn new(sequence: &[f32]) -> Self {
        Self {
            sequence: sequence.to_vec(),
            width: MIN_WIDTH,
            height: MAX_HEIGHT,
            align: Align::Center,
            snap: Snap::Height
        }
    }

    pub fn align(&mut self, x: Align) -> &mut Graph {
        self.align = x;
        self
    }

    pub fn snap(&mut self, x: Snap) -> &mut Graph {
        let ppm_width: i32 = match x {
            Snap::Height => sequence_width(&self.sequence),
            Snap::Width => MIN_WIDTH, // TODO: implement snapping by width
        };
        let ppm_height: i32 = match x {
            Snap::Height => MAX_HEIGHT,
            Snap::Width => MAX_HEIGHT, // TODO: implement snapping by width
        };
        self.snap = x;
        self.height = ppm_height;
        self.width = ppm_width;
        self
    }

    pub fn save(&self, name: &str) -> std::io::Result<()> {
        let max_size = 3 * self.width * self.height;
        let mut buffer = vec![0; max_size as usize];

        let mut i = 0;
        for sample in &self.sequence {
            let y = self.sample_y_at(i, sample);
            let x = self.sample_x_at(i, sample);
            let offset = ((y * self.width * 3) + (x * 3)) as usize;

            if offset >= max_size as usize {
                continue;
            }

            buffer[offset] = 255;
            buffer[offset+1] = 255;
            buffer[offset+2] = 0;

            i+=1;
        }

        self.write(name, &buffer)
    }

    fn write(&self, name: &str, buffer: &[u8]) -> std::io::Result<()> {
        let mut p = BufWriter::new(File::create("foo.ppm")?);
        p.write(format!("P6 {} {} 255\n", self.width, self.height).as_bytes())?;
        p.write(&buffer)?;

        Ok(())
    }

    fn sample_y_at(&self, i: i32, sample: &f32) -> i32 {
        let amplitude_height = self.height - 2*VERTICAL_PADDING;
        let y = match self.align {
            Align::Center => ((sample+1.0) * amplitude_height as f32 / 2.0),
            Align::Bottom => (sample+1.0) * amplitude_height as f32 
        };
        y as i32 + VERTICAL_PADDING
    }

    fn sample_x_at(&self, i: i32, sample: &f32) -> i32 {
        let scale = self.width as f32 / self.sequence.len() as f32;
        (i as f32 * scale) as i32
    }
}

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
