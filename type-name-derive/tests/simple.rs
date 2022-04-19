#[test]
fn simple() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("type-name-derive"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}
