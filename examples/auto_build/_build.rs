use cornucopi::{CodegenSettings, Error};

// This script will generate a new cornucopi file every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";
    let destination = "src/cornucopi.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopi::generate_managed(
        queries_path,
        &[schema_file],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}
