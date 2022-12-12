use crate::{
    tests, CallbackFunction, CallbackInterface, Dictionary, Enumeration, Includes, Interface,
    InterfaceMixin, Namespace, Type, Typedef,
};

#[test]
fn test_interface_simple() {
    let interface = Interface {
        ext_attrs: vec![],
        partial: false,
        identifier: "Foo".to_string(),
        inheritance: None,
        members: vec![],
    };
    let expected = tests::load_test_file("interface_simple.idl");

    assert!(interface.to_string() == expected);
}

#[test]
fn test_interface_mixin_simple() {
    let mixin = InterfaceMixin {
        ext_attrs: vec![],
        partial: false,
        identifier: "Foo".to_string(),
        members: vec![],
    };
    let expected = tests::load_test_file("interface_mixin_simple.idl");

    assert!(mixin.to_string() == expected);
}

#[test]
fn test_includes_simple() {
    let includes = Includes {
        ext_attrs: vec![],
        interface: "Foo".to_string(),
        mixin: "Bar".to_string(),
    };
    let expected = tests::load_test_file("includes_simple.idl");

    assert!(includes.to_string() == expected);
}

#[test]
fn test_callback_interface_simple() {
    let cb_interface = CallbackInterface {
        ext_attrs: vec![],
        identifier: "Foo".to_string(),
        members: vec![],
    };
    let expected = tests::load_test_file("callback_interface_simple.idl");

    assert!(cb_interface.to_string() == expected);
}

#[test]
fn test_namespace_simple() {
    let namespace = Namespace {
        ext_attrs: vec![],
        partial: false,
        identifier: "Foo".to_string(),
        members: vec![],
    };
    let expected = tests::load_test_file("namespace_simple.idl");

    assert!(namespace.to_string() == expected);
}

#[test]
fn test_dictionary_simple() {
    let dictionary = Dictionary {
        ext_attrs: vec![],
        partial: false,
        identifier: "Foo".to_string(),
        inheritance: None,
        members: vec![],
    };
    let expected = tests::load_test_file("dictionary_simple.idl");

    assert!(dictionary.to_string() == expected);
}

#[test]
fn test_enumeration_simple() {
    let r#enum = Enumeration {
        ext_attrs: vec![],
        identifier: "Foo".to_string(),
        values: vec![],
    };
    let expected = tests::load_test_file("enumeration_simple.idl");

    assert!(r#enum.to_string() == expected);
}

#[test]
fn test_callback_function_simple() {
    let cb_function = CallbackFunction {
        ext_attrs: vec![],
        identifier: "Foo".to_string(),
        r#type: Type::from("Bar"),
        arguments: vec![],
    };
    let expected = tests::load_test_file("callback_function_simple.idl");

    assert!(cb_function.to_string() == expected);
}

#[test]
fn test_typedef_simple() {
    let typedef = Typedef {
        ext_attrs: vec![],
        r#type: Type::from("Foo"),
        identifier: "Bar".to_string(),
    };
    let expected = tests::load_test_file("typedef_simple.idl");

    assert!(typedef.to_string() == expected);
}
