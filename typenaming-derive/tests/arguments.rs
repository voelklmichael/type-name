#[test]
fn arguments_no() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_type_name() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(type_name = "type_renamed")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("type_renamed", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_name() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(crate_name = "crate_renamed")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("crate_renamed"), type_name.crate_name().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_module() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(crate_module = "module_renamed")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(Some("module_renamed"), type_name.crate_module().as_deref());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_crate_version() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(crate_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.crate_version()
    );
    assert_eq!(0, type_name.generics().len());
}
#[test]
fn arguments_rustc_version() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(rustc_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("typenaming-derive"), type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.rustc_version()
    );
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_default_to_none() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(default_to_none = true)]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(None, type_name.crate_name().as_deref());
    assert_eq!(&None, type_name.crate_module());
    assert_eq!(&None, type_name.crate_version());
    assert_eq!(&None, type_name.rustc_version());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_default_to_none_crate_version() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(default_to_none = true, crate_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(None, type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.crate_version()
    );
    assert_eq!(&None, type_name.rustc_version());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_default_to_none_rustc_version() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(default_to_none = true, rustc_version = "1.2.3")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(None, type_name.crate_name().as_deref());
    assert_eq!(
        &Some(::typenaming::TypeNameSemverVersion::new(1, 2, 3)),
        type_name.rustc_version()
    );
    assert_eq!(&None, type_name.crate_version());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_default_to_none_crate_name() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(default_to_none = true, crate_name = "test")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(Some("test"), type_name.crate_name().as_deref());
    assert_eq!(&None, type_name.crate_version());
    assert_eq!(&None, type_name.rustc_version());
    assert_eq!(0, type_name.generics().len());
}

#[test]
fn arguments_default_to_none_crate_module() {
    use ::typenaming::TypeName;
    #[derive(TypeName)]
    #[typename(default_to_none = true, crate_module = "test")]
    struct B {}
    let type_name = dbg!(B::type_name_static());
    assert_eq!("B", type_name.type_name());
    assert_eq!(None, type_name.crate_name().as_deref());
    assert_eq!(Some("test"), type_name.crate_module().as_deref());
    assert_eq!(&None, type_name.crate_version());
    assert_eq!(&None, type_name.rustc_version());
    assert_eq!(0, type_name.generics().len());
}
