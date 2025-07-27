#[derive(Debug)]
pub struct Men {
    pub eyes: String,
}

impl Men {
    pub fn new() -> Self {
        Men {
            eyes: String::from("You will be fine"),
        }
    }
}
