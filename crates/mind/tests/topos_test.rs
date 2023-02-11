use mind::topos::Topos;

#[test]
fn test_print_debug() {
    let none = Option::<String>::None;
    assert_eq!(format!("{:?}", none), "None");
    let some = Option::<String>::Some(String::from("13"));
    assert_eq!(format!("{:?}", some), "Some(\"13\")");

    let topos = Topos::Nil;
    
    assert_eq!(format!("{:?}", topos), "Nil");
}

#[test]
fn test_clone() {
    let none = Topos::Nil;

    match none.clone() {
        Topos::Nil => {},
        _ => panic!("non-matching clone")
    }
}
