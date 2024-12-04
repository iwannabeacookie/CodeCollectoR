use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Writer where Self: Write {
    writer: BufWriter<File>,
}

impl Writer {
    pub fn new(output_path: &Path) -> Result<Self> {
        let file = File::create(output_path)
            .with_context(|| format!("Creating output file {:?}", output_path))?;
        let writer = BufWriter::new(file);
        Ok(Self { writer })
    }
}

impl std::ops::Deref for Writer {
    type Target = BufWriter<File>;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl std::ops::DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        if let Ok(out) = self.writer.write(buf).with_context(|| "Writing to output file") {
            Ok(out)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to write to output file"))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl Clone for Writer {
    fn clone(&self) -> Self {
        Self {
            writer: BufWriter::new(self.writer.get_ref().try_clone().unwrap()),
        }
    }
}
