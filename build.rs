use std::path::PathBuf;

use glob::glob;

fn main() -> std::io::Result<()> {
    let descriptor_path =
        PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    let protos: Vec<PathBuf> = glob("protos/data-plane-api/envoy/**/v3/*.proto")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(descriptor_path.clone())
        .compile_well_known_types(true)
        .include_file("mod.rs")
        .compile_protos(
            &protos,
            &[
                "protos/client_model/",
                "protos/opentelemetry-proto/",
                "protos/data-plane-api/",
                "protos/xds",
                "protos/googleapis/",
		"protos/protoc-gen-validate",
		"protos/cel-spec/proto"
            ],
        )?;

    Ok(())
}
