fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .file_descriptor_set_path("accounts_descriptor.bin")
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .compile(
            &["../spec/account.proto"],
            &["../spec"],
        )?;

    Ok(())
}