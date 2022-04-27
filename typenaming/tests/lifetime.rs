#[test]
fn simple() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct A<'a, T>(&'a T);
    let type_name = dbg!(A::<&'static u32>::type_info());
    assert_eq!("A", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(1, type_name.generics().len());
    assert_eq!("u32", type_name.generics()[0].type_name());
    assert_eq!(
        Some("core"),
        type_name.generics()[0].crate_name().as_deref()
    );
}
