// use tonic_build::configure;

fn main(){
    const PROTOC_ENVAR: &str = "PROTOC";

    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    tonic_prost_build::configure()
        .build_server(true)       // 如果你要生成 server
        .build_client(true)       // 如果你要生成 client
        .out_dir("src/generated") // 输出到 src/generated
        .compile_protos(
            &["protos/shared.proto", "protos/shredstream.proto"],
            &["protos"],
        ).unwrap();
}
