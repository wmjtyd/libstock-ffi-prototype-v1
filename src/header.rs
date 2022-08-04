//! For `bin/gen-header.rs`. Generate the C header.

pub fn generate_header() -> anyhow::Result<()> {
    use anyhow::Context;

    ::safer_ffi::headers::builder()
        .to_file("libstock.h")
        .context("Failed to create builder.")?
        .generate()
        .context("Failed to generate header.")?;

    Ok(())
}
