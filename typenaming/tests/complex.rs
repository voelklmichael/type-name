#[test]
fn complex() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct A<'a, 'b, T, S>(&'a T, &'b S);
    let type_name = A::<&'static u32, &'static bool>::type_info();
    assert_eq!("A", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(2, type_name.generics().len());
    assert_eq!("u32", type_name.generics()[0].type_name());
    assert_eq!(
        Some("core"),
        type_name.generics()[0].crate_name().as_deref()
    );
    assert_eq!("bool", type_name.generics()[1].type_name());
    assert_eq!(
        Some("core"),
        type_name.generics()[1].crate_name().as_deref()
    );
}
