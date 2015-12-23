extern crate serde_json;
extern crate ocf;

#[test]
fn parse_config() {
    let config = include_str!("config.json");
    let runtime = include_str!("runtime.json");

    let spec = serde_json::from_str::<ocf::Spec>(config);
    assert!(spec.is_ok());

    let runtime = serde_json::from_str::<ocf::RuntimeSpec>(runtime);
    assert!(runtime.is_ok());
}
