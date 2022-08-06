#[cfg(feature = "headers")]
fn main() -> anyhow::Result<()> {
    wmjtyd_libstock_ffi::header::generate_header(::std::env::args_os().nth(1).as_deref())
}
