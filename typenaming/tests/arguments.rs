#[test]
fn arguments_no() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_type_name() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    #[typenameable(type_name = "type_renamed")]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("type_renamed", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_name() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    #[typenameable(crate_name = "crate_renamed")]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("crate_renamed"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_module() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    #[typenameable(crate_module = "module_renamed")]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(Some("module_renamed"), type_name.crate_module().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_version() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    #[typenameable(crate_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::Version::new(1, 2, 3)),
        type_name.crate_version()
    );
    assert_eq!(0, type_name.generics().len());
}
#[test]
fn arguments_rustc_version() {
    use ::typenaming::TypeNameable;
    #[derive(TypeNameable)]
    #[typenameable(rustc_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_info());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::Version::new(1, 2, 3)),
        type_name.rustc_version()
    );
    assert_eq!(0, type_name.generics().len());
}
