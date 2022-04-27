#[test]
fn simple() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}
