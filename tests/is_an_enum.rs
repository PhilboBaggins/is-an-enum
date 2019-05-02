use is_an_enum::*;

#[allow(dead_code)]
#[derive(IsAnEnum)]
#[derive(PartialEq)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[test]
fn test_is_an_enum() {
    assert!(true  == Day::Sunday.is_a_sunday());
    assert!(false == Day::Sunday.is_a_monday());
    assert!(false == Day::Sunday.is_a_tuesday());
    assert!(false == Day::Sunday.is_a_wednesday());
    assert!(false == Day::Sunday.is_a_thursday());
    assert!(false == Day::Sunday.is_a_friday());
    assert!(false == Day::Sunday.is_a_saturday());
}
