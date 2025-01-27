use prost_build;
use protobuf_codegen;

fn main() {
    prost_build::Config::new()  // Where to generate the Rust code
        .compile_protos(&["src/protos/ProtocolMessage.proto"], &["src"]).unwrap();

        protobuf_codegen::Codegen::new()
        .include("src")
        .inputs(["src/protos/ProtocolMessage.proto", "src/protos/DeviceInfoMessage.proto"].iter().map(|s| s.to_string()))
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}
