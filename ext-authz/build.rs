fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .include_file("mod.rs")
        .compile(
            &["proto/envoy/api/envoy/service/auth/v3/external_auth.proto"],
            &[
                "proto/envoy/api",
                "proto/protoc-gen-validate",
                "proto/googleapis",
                "proto/udpa",
            ],
        )
        .unwrap();
}
