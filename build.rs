extern crate tower_grpc_build;

fn main() {
    println!("Starting build");

    // prefer writing the compiled proto code to src folder
    std::env::set_var("OUT_DIR", "src/");

    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(false)
        .build(&["proto/shorty.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    println!("cargo:rerun-if-changed=proto/shorty.proto");
}
