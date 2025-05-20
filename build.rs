
fn main() -> anyhow::Result<()> {
    femtopb_build::compile_protos(
        &["src/api.proto", "src/api_options.proto"],
        &["src"],
    )
}