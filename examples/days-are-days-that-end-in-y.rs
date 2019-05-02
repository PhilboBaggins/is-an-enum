use is_an_enum::*;

#[allow(dead_code)]
#[derive(IsAnEnum)]
#[derive(Debug)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    fn does_it_end_in_y(&self) -> bool {
        let s = format!("{:?}", self);
        s.ends_with('y') || s.ends_with('Y')
    }
}

fn main() {
    if Day::Sunday.is_a_sunday() && Day::Sunday.does_it_end_in_y() {
        println!("Sunday is a Sunday and it ends in \"y\"");
    } else {
        println!("The world is ending");
    }
}
