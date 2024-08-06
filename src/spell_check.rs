use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    usize,
};

use anyhow::{anyhow, Context, Result};

use crate::hash;
use crate::header;

#[derive(Debug, Clone)]
struct OutputParams {
    bin_path: PathBuf,
    word: String,
}

pub fn spell_check(bin_path: String, word: String) -> Result<()> {
    let op_params = validate(bin_path, word).context("Couldnt validate the input params")?;

    let file_handle = File::open(op_params.bin_path).context("Couldnt open bloomfilter")?;

    let mut buffer = BufReader::new(file_handle);

    let mut header_chunk = [0; header::HEADER_ENCODE_SIZE];

    buffer
        .read_exact(&mut header_chunk)
        .context("Couldn't read header info from buffered reader")?;
    let header: header::Header = header::deserialize(header_chunk)
        .context("Coulndt read header info from buffered reader")?;

    let buffer_length = hash::round_up_to_next_multiple_of_8(header.bin_length) / 8;
    let mut bf_chunk = vec![0_u8; buffer_length as usize];
    buffer
        .read(&mut bf_chunk)
        .context("Couldnt read bloom filter values")?;

    let hashes_found: u8 = hash::hash(op_params.word, header.bin_length, header.hash_count)
        .iter()
        .map(|x| -> u8 {
            let byte: usize = (x / 255).try_into().unwrap();
            let byte = bf_chunk[byte];

            let position = x % 8;
            println!("{:08b}", byte);
            println!("num: {}; byte: {:?} ; position: {:?}", x, byte, position);
            if get_bit_at(byte, position) {
                1
            } else {
                0
            }
        })
        .sum();
    println!("{}", hashes_found);
    Ok(())
}

fn get_bit_at(input: u8, n: u64) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn validate(bin_path: String, word: String) -> Result<OutputParams> {
    let bin_path = PathBuf::from(bin_path);
    if !bin_path.exists() {
        return Err(anyhow!("Bloomfilter doesnt exist: {}", bin_path.display()));
    }

    Ok(OutputParams { bin_path, word })
}
