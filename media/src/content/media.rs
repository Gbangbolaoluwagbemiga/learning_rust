#[derive(Debug)]
pub enum Media {
    Book { director: String, title: String },
    Movie { author: String, title: String },
    Audiobook { title: String },
    Placeholder,
    Podcast(u32),
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Audiobook { title } => {
                format!("Audio book title is {}", title)
            }
            Media::Book { director, title } => {
                format!("Book title is {} and the director is {}", title, director)
            }
            Media::Movie { title, author } => {
                format!("Movie title is {} and the author is {}", title, author)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
            Media::Podcast(any) => {
                format!("podcast init {} ", { any })
            }
        }
    }
}
