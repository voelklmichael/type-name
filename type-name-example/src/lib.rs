#[test]
fn check_crate_version_number() {
    use type_name::TypeName;
    let type_name = dbg!(::type_name_example_base::TestVersionNumber::type_name_static());
    assert_eq!("TestVersionNumber", type_name.type_name());
    assert_eq!(
        "type-name-example-base",
        type_name.crate_name().as_deref().unwrap()
    );
    assert_eq!(
        &Some(type_name::TypeNameSemverVersion::new(0, 4, 2)),
        type_name.crate_version()
    );
    assert_eq!(0, type_name.generics().len());
}
