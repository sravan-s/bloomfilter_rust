mod hash;
mod make_filter;
mod spell_check;
use std::env;

use anyhow::{Context, Ok, Result};

fn print_options() {
    print!(
        "
         To build bloomfilter: bloomfilter-rs make path_to_dict_src path_to_dict_output m k \n
         To spell check: bloomfilter-rs check path_to_bloom_filter word \n
         m = Number of bits in the filter min:2^6 max:2^64\n
         k = Number of hash functions min=2 max:2^8 \n
    "
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_, ref cmd, ref dict_path, ref output_path, ref m, ref k] if cmd == "make" => {
            let params = make_filter::parse(
                dict_path.to_owned(),
                output_path.to_owned(),
                m.to_owned(),
                k.to_owned(),
            )
            .context("Error in parsing params")?;
            make_filter::make_filter(params)?;
        }
        [_, ref cmd] if cmd == "check" => spell_check::spell_check(),
        [_, ref cmd] if cmd == "help" => print_options(),
        _ => print_options(),
    };
    Ok(())
}
