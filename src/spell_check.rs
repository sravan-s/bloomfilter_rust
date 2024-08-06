use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::header;

#[derive(Debug, Clone)]
struct OutputParams {
  bin_path: PathBuf,
  word: String,
}

pub fn spell_check(bin_path: String, word: String) -> Result<()> {
  let op_params = validate(bin_path, word)
    .context("Couldnt validate the input params")?;

  let file_handle = File::open(op_params.bin_path)
    .context("Couldnt open bloomfilter")?;

  let mut buffer = BufReader::new(file_handle);

  let mut header_chunk = [0; header::HEADER_ENCODE_SIZE];
    
  buffer.read_exact(&mut header_chunk)
    .context("Couldn't read header info from buffered reader")?;

  println!("{:?}", header_chunk);

  let header: header::Header = header::deserialize(header_chunk)
    .context("Coulndt read header info from buffered reader")?;

  println!("{:?}", header);
  Ok(())
}

fn validate(bin_path: String, word: String) -> Result<OutputParams> {
  let bin_path = PathBuf::from(bin_path);
  if !bin_path.exists() {
      return Err(anyhow!("Bloomfilter doesnt exist: {}", bin_path.display()));
  }

  Ok(OutputParams {
    bin_path,
    word,
  })
}