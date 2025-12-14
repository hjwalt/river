use std::io::Result;

use prost_build::Config;

fn main() -> Result<()> {
    Config::new()
        .out_dir("./src/domain")
        .compile_protos(&["proto/test.proto"], &["proto/"])?;

    Ok(())
}
