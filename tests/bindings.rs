use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::{Config, Generator};
    use interoptopus_backend_csharp::overloads::{DotNet, Unity};

    let config = Config {
        dll_name: "bhaptics_test".to_string(),
        namespace_mappings: NamespaceMappings::new("Bhaptics.Test"),
        ..Config::default()
    };

    Generator::new(config, bhaptics_test::my_inventory())
        .add_overload_writer(DotNet::new())
        //.add_overload_writer(Unity::new())
        .write_file("FFI.cs")?;

    Ok(())
}