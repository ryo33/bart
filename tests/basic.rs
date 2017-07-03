#[macro_use] extern crate bart_derive;

#[test]
fn it_works() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World".to_owned() })
    );
}

#[test]
fn it_finds_template_files() {
    #[derive(BartDisplay)]
    #[template="tests/templates/basic/it_finds_template_files.html"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World".to_owned() })
    );
}

#[test]
fn it_handles_names_with_underscore() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{your_name}}"]
    struct Test { your_name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { your_name: "World".to_owned() })
    );
}

#[test]
fn it_handles_tuple_struct_field_names() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{0}}"]
    struct Test<'a>(&'a str);

    assert_eq!(
        "Hello, World",
        format!("{}", Test("World"))
    );
}

#[test]
fn it_handles_some_whitespace() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{  name  }}"]
    struct Test { name: String }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World".to_owned() })
    );
}

#[test]
fn it_can_borrow() {
    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    struct Test<'a> { name: &'a str }

    assert_eq!(
        "Hello, World",
        format!("{}", Test { name: "World" })
    );
}

#[test]
fn it_performs_escaping() {
    #[derive(BartDisplay)]
    #[template_string="{{txt}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "&lt;&amp;&quot;&apos;",
        format!("{}", Test { txt: "<&\"'" })
    );
}

#[test]
fn it_passes_through() {
    #[derive(BartDisplay)]
    #[template_string="{{{txt}}}"]
    struct Test<'a> { txt: &'a str }

    assert_eq!(
        "<&\"'",
        format!("{}", Test { txt: "<&\"'" })
    );
}

#[test]
fn template_root_element() {
    struct Nested<'a> { name: &'a str }

    #[derive(BartDisplay)]
    #[template_string="Hello, {{name}}"]
    #[template_root="0"]
    struct Test<'a>(Nested<'a>);

    assert_eq!(
        "Hello, World",
        format!("{}", Test(Nested { name: "World" }))
    );
}
