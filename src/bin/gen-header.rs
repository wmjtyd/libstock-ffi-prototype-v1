#[cfg(feature = "headers")]
fn main() -> anyhow::Result<()> {
    wmjtyd_libstock_ffi::header::generate_header()
}
