mod sign;

use std::{
    fs::File,
    io::{Error, Read},
};

use anyhow::Context;
use arg::{Args, parse_args};
use sign::sign_identity;

fn read_text_file(file_name: &str) -> Result<String, Error> {
    let mut file = File::open(file_name)?;
    let mut string = String::new();

    file.read_to_string(&mut string)?;

    Ok(string)
}

#[derive(Args, Debug)]
struct FileArgs {
    #[arg(short, long, required)]
    ///Path of the identity file
    identity: String,

    #[arg(short, long = "signing-key", required)]
    ///Path of the signing key
    signing_key: String,
}

fn main() -> anyhow::Result<()> {
    let args: FileArgs = parse_args();

    let id = read_text_file(&args.identity).context("Failed to read identity file")?;
    println!(
        "{}",
        &sign_identity(
            &id,
            &read_text_file(&args.signing_key).context("Failed to read signing key file")?
        )?
    );

    Ok(())
}
