use crate::{parse, tests, Definition, Type};

#[test]
fn test_interface_simple() {
    let interface = match parse(&tests::load_test_file("interface_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Interface(interface) => interface,
        _ => panic!("Parsed definition is not an interface."),
    };

    assert!(interface.ext_attrs.is_empty());
    assert!(interface.partial == false);
    assert!(interface.identifier == "Foo");
    assert!(interface.inheritance == None);
    assert!(interface.members.is_empty());
}

#[test]
fn test_interface_mixin_simple() {
    let mixin = match parse(&tests::load_test_file("interface_mixin_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::InterfaceMixin(mixin) => mixin,
        _ => panic!("Parsed definition is not an interface mixin."),
    };

    assert!(mixin.ext_attrs.is_empty());
    assert!(mixin.partial == false);
    assert!(mixin.identifier == "Foo");
    assert!(mixin.members.is_empty());
}

#[test]
fn test_includes_simple() {
    let includes = match parse(&tests::load_test_file("includes_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Includes(includes) => includes,
        _ => panic!("Parsed definition is not an includes."),
    };

    assert!(includes.ext_attrs.is_empty());
    assert!(includes.interface == "Foo");
    assert!(includes.mixin == "Bar");
}

#[test]
fn test_callback_interface_simple() {
    let cb_interface = match parse(&tests::load_test_file("callback_interface_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::CallbackInterface(cb_interface) => cb_interface,
        _ => panic!("Parsed definition is not a callback interface."),
    };

    assert!(cb_interface.ext_attrs.is_empty());
    assert!(cb_interface.identifier == "Foo");
    assert!(cb_interface.members.is_empty());
}

#[test]
fn test_namespace_simple() {
    let namespace = match parse(&tests::load_test_file("namespace_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Namespace(namespace) => namespace,
        _ => panic!("Parsed definition is not a namespace."),
    };

    assert!(namespace.ext_attrs.is_empty());
    assert!(namespace.partial == false);
    assert!(namespace.identifier == "Foo");
    assert!(namespace.members.is_empty());
}

#[test]
fn test_dictionary_simple() {
    let dictionary = match parse(&tests::load_test_file("dictionary_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Dictionary(dictionary) => dictionary,
        _ => panic!("Parsed definition is not a dictionary."),
    };

    assert!(dictionary.ext_attrs.is_empty());
    assert!(dictionary.partial == false);
    assert!(dictionary.identifier == "Foo");
    assert!(dictionary.inheritance == None);
    assert!(dictionary.members.is_empty());
}

#[test]
fn test_enumeration_simple() {
    let r#enum = match parse(&tests::load_test_file("enumeration_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Enumeration(r#enum) => r#enum,
        _ => panic!("Parsed definition is not an enumeration."),
    };

    assert!(r#enum.ext_attrs.is_empty());
    assert!(r#enum.identifier == "Foo");
    assert!(r#enum.values.is_empty());
}

#[test]
fn test_callback_function_simple() {
    let cb_function = match parse(&tests::load_test_file("callback_function_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::CallbackFunction(cb_function) => cb_function,
        _ => panic!("Parsed definition is not a callback function."),
    };

    assert!(cb_function.ext_attrs.is_empty());
    assert!(cb_function.identifier == "Foo");
    assert!(cb_function.r#type == Type::from("Bar"));
    assert!(cb_function.arguments.is_empty());
}

#[test]
fn test_typedef_simple() {
    let typedef = match parse(&tests::load_test_file("typedef_simple.idl"))
        .unwrap()
        .remove(0)
    {
        Definition::Typedef(typedef) => typedef,
        _ => panic!("Parsed definition is not a callback function."),
    };

    assert!(typedef.ext_attrs.is_empty());
    assert!(typedef.r#type == Type::from("Foo"));
    assert!(typedef.identifier == "Bar");
}
