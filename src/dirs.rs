use std::env;


pub struct Dirs {
    cwd: String,
    libdir: String,
    testdir: String,
    failtestdir: String
}


impl Dirs {

    pub fn new() -> Dirs {
        Dirs {
            cwd: env::current_dir().unwrap().display().to_string(),
            libdir: String::from("src/dartlib"),
            testdir: String::from("test"),
            failtestdir: String::from("testfail")
        }
    }


    pub fn cwd(&self) -> String {
        self.cwd.clone()
    }


    pub fn libdir(&self) -> String {
        format!("{}/{}", self.cwd, self.libdir)
    }


    pub fn testdir(&self) -> String {
        format!("{}/{}", self.cwd, self.testdir)
    }


    pub fn failtestdir(&self) -> String {
        format!("{}/{}", self.cwd, self.failtestdir)
    }
}
