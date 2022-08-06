//! For `bin/gen-header.rs`. Generate the C header.

use std::ffi::OsStr;

pub fn generate_header(filename: Option<&OsStr>) -> anyhow::Result<()> {
    use anyhow::Context;

    let builder = ::safer_ffi::headers::builder();

    if let Some(filename) = filename {
        builder
            .to_file(filename)
            .context("Failed to create builder.")?
            .generate()
            .context("Failed to generate header.")?;
    } else {
        builder
            .to_writer(::std::io::stdout())
            .generate()
            .context("Failed to generate header.")?;
    }

    Ok(())
}
