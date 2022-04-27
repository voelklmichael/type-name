#[test]
fn check_crate_version_number() {
    use typenaming::TypeNameable;
    let type_name = dbg!(::typenaming_example_base::TestVersionNumber::type_info());
    assert_eq!("TestVersionNumber", type_name.type_name());
    assert_eq!(
        "typenaming-example-base",
        type_name.crate_name().as_deref().unwrap()
    );
    assert_eq!(
        &Some(typenaming::Version::new(0, 4, 2)),
        type_name.crate_version()
    );
    assert_eq!(0, type_name.generics().len());
}
