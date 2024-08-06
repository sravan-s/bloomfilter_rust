use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    u64,
};

use anyhow::{anyhow, Context, Ok, Result};
use bit_vec::BitVec;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    hash,
    header::{self, HEADER_ENCODE_SIZE},
};
#[derive(Debug, Clone)]
pub struct FilterParams {
    path: PathBuf,
    output: PathBuf,
    m: u64,
    k: u8,
}

const K_MIN: u8 = 2;
const M_MIN: u64 = 64;
const VERSION: u8 = 1;

pub fn make_filter(params: FilterParams) -> Result<()> {
    let mut header = header::serialize(header::Header {
        version: VERSION,
        bin_length: params.m,
        hash_count: params.k,
    });
    let handle = File::open(params.path).context("Trying to open dictonary")?;
    let f = BufReader::new(handle);
    // using rayon for parallel processing
    let vec_size: usize = params.m.try_into().context("Cannot create bit_array")?;

    let list = Arc::new(Mutex::new(BitVec::from_elem(
        vec_size + HEADER_ENCODE_SIZE,
        false,
    )));

    // let now = Instant::now();
    f.lines().par_bridge().for_each(|line_result| {
        let line = line_result.unwrap_or_else(|_| "".to_string());
        let h: Vec<u64> = hash::hash(line, params.m, params.k);
        let mut list = list.lock().unwrap();
        h.into_iter().for_each(|m| {
            let m = m as usize;
            list.set(m, true);
        })
    });
    // println!("elapse: {:?}", now.elapsed().as_nanos());
    let mut list = list.lock().unwrap().to_owned();
    header.append(&mut list);
    let mut op_handle = File::create(params.output).context("Trying to write output")?;
    let unwrapped_list = header.to_bytes();
    op_handle
        .write_all(&unwrapped_list)
        .context("Write to output file failed")?;
    Ok(())
}

pub fn parse(path: String, output: String, m: String, k: String) -> Result<FilterParams> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err(anyhow!("Path doesnt exist: {}", path.display()));
    }

    let output = PathBuf::from(output);

    let m_parse_error = format!("Couldnt parse m: {}", m.clone());
    let m: u64 = m.parse().context(m_parse_error)?;

    let k_parse_error = format!("Couldnt parse k: {}", k.clone());
    let k: u8 = k.parse().context(k_parse_error)?;

    if m < M_MIN || k < K_MIN {
        let minimum_size_error = format!("m: {} and k: {} must be larger than zero", M_MIN, K_MIN);
        return Err(anyhow!(minimum_size_error));
    }
    Ok(FilterParams { path, output, m, k })
}
