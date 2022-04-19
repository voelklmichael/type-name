#[test]
fn arguments_no() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("type-name"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_type_name() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    #[typename(type_name = "type_renamed")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("type_renamed", type_name.type_name());
    assert_eq!(Some("type-name"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_name() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    #[typename(crate_name = "crate_renamed")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("crate_renamed"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_version() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    #[typename(crate_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("type-name"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::type_name::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.crate_version()
    );
    assert_eq!(0, type_name.generics().len());
}
#[test]
fn arguments_rustc_version() {
    use ::type_name::TypeName;
    #[derive(TypeName)]
    #[typename(rustc_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("type-name"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::type_name::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.rustc_version()
    );
    assert_eq!(0, type_name.generics().len());
}