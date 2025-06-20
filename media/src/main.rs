#[derive(Debug)]

enum Media {
    Book { director: String, title: String },
    Movie { author: String, title: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
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
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn main() {
    let book = Media::Book {
        director: String::from("Oluwagbemiga"),
        title: String::from("Life"),
    };

    let movie = Media::Movie {
        author: String::from("Xcel"),
        title: String::from("The Foolish boy"),
    };
    let audio = Media::Audiobook {
        title: String::from("Divinely"),
    };

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio);

    println!("Catalogs are {:#?}", catalog);

    // println!("Book description {:#?}", book.description());
    // println!("Audio description {:#?}", movie.description());
    // println!("Movie description {:#?}", audio.description());
}
