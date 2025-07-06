struct Has<'a> {
    liftime: &'a str,
}

fn main() {
    let long = String::from("long");
    let mut has = Has { liftime: &long };

    assert_eq!(has.liftime, "long");
    {
        let short = String::from("short");
        has.liftime = &short;
        assert_eq!(has.liftime, "short");

        has.liftime = &long;
        assert_eq!(has.liftime, "long");
    }

    assert_eq!(has.liftime, "long");
}
