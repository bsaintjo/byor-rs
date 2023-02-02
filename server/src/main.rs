use std::{
    env,
};


fn init_logger() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "trace");
    }
    pretty_env_logger::init_timed();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    server::run()?;
    Ok(())
}