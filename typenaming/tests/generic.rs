#[test]
fn simple() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct A<T>(T);
    let type_name = dbg!(A::<u32>::type_info());
    assert_eq!("A", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(1, type_name.generics().len());
    assert_eq!("u32", type_name.generics()[0].type_name());
    assert_eq!(
        Some("core"),
        type_name.generics()[0].crate_name().as_deref()
    );
}

#[test]
fn two() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct A<T, U>(T, U);
    let type_name = dbg!(A::<u32, bool>::type_info());
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
