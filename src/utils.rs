

pub fn dprint<S: Into<String>>(s: S) {
    let debug = true;

    if debug {
        println!("{}", s.into());
    }
}

