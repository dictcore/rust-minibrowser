use std::collections::HashMap;

pub struct Globals {
    css_named_colors: HashMap<String,String>,
}

pub fn make_globals() -> Globals {
    let mut map:HashMap<String,String> = HashMap::new();
    map.insert(String::from("aqua"), String::from("#00ffff"));
    let globals = Globals {
        css_named_colors:map
    };
    return globals;
}

#[test]
fn test_make_globals() {
    let globals = make_globals();
    assert_eq!(globals.css_named_colors.get("aqua").unwrap(), "#00ffff");
}
