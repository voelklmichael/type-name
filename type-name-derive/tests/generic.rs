#[test]
fn simple() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    struct A<T>(T);
    let type_name = dbg!(A::<u32>::type_name_static());
    assert_eq!("A", type_name.type_name());
    assert_eq!(Some("type-name-derive"), type_name.crate_name().as_deref());
    assert_eq!(1, type_name.generics().len());
    assert_eq!("u32", type_name.generics()[0].type_name());
    assert_eq!(
        Some("core"),
        type_name.generics()[0].crate_name().as_deref()
    );}
