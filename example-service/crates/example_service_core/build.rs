use std::env;
use std::path::PathBuf;

const SERVICE_PROTO: &str = "../../proto/example/service.proto";
const PROTO_DIR: &str = "../../proto";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed={}", PROTO_DIR);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("example.service.bin"))
        .build_server(true)
        .build_client(false)
        .compile(&[SERVICE_PROTO],
                 &[PROTO_DIR]
        )
        .unwrap();

    Ok(())
}