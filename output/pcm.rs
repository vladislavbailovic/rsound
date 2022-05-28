use std::fs::File;
use std::io::{Write,BufWriter};

pub fn save(sequence: &[f32]) -> std::io::Result<()> {
    let mut f = BufWriter::new(File::create("foo.pcm")?);
    for sample in sequence {
        f.write(&sample.to_le_bytes())?;
    }
    Ok(())
}
