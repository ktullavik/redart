use std::process;


pub fn dprint<S: Into<String>>(s: S) {
    let debug = false;

    if debug {
        println!("{}", s.into());
    }
}


pub fn darterror<S: Into<String>>(s: S) -> ! {
    let debug = false;

    if debug {
        panic!("{}", s.into())
    }
    else {
        println!("Error: {}", s.into());
        process::exit(1);
    }
}
