use is_an_enum::*;

#[allow(dead_code)]
#[derive(IsAnEnum)]
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
fn is_it_a_sunday() {
    assert_eq!(true,  Day::Sunday.is_a_sunday());
    assert_eq!(false, Day::Sunday.is_a_monday());
    assert_eq!(false, Day::Sunday.is_a_tuesday());
    assert_eq!(false, Day::Sunday.is_a_wednesday());
    assert_eq!(false, Day::Sunday.is_a_thursday());
    assert_eq!(false, Day::Sunday.is_a_friday());
    assert_eq!(false, Day::Sunday.is_a_saturday());
}

#[test]
fn is_it_a_monday() {
    assert_eq!(false, Day::Monday.is_a_sunday());
    assert_eq!(true,  Day::Monday.is_a_monday());
    assert_eq!(false, Day::Monday.is_a_tuesday());
    assert_eq!(false, Day::Monday.is_a_wednesday());
    assert_eq!(false, Day::Monday.is_a_thursday());
    assert_eq!(false, Day::Monday.is_a_friday());
    assert_eq!(false, Day::Monday.is_a_saturday());
}

#[test]
fn is_it_a_tuesday() {
    assert_eq!(false, Day::Tuesday.is_a_sunday());
    assert_eq!(false, Day::Tuesday.is_a_monday());
    assert_eq!(true,  Day::Tuesday.is_a_tuesday());
    assert_eq!(false, Day::Tuesday.is_a_wednesday());
    assert_eq!(false, Day::Tuesday.is_a_thursday());
    assert_eq!(false, Day::Tuesday.is_a_friday());
    assert_eq!(false, Day::Tuesday.is_a_saturday());
}
