#[derive(Debug)]
enum Media {
    Book { director: String, title: String },
    Movie { author: String, title: String },
    Audiobook { title: String },
    Placeholder,
    Podcast(u32),
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
            Media::Placeholder => {
                format!("Placeholder")
            }
            Media::Podcast(any) => {
                format!("podcast init {} ", { any })
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
    fn get_items(&self, id: usize) -> Option<&Media> {
        if self.items.len() > id {
            Some(&self.items[id])
        } else {
            None
        }
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
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audio);

    let cataloged = catalog.items.get(10);
    println!("Catalogs are {:#?}", cataloged.unwrap_or(&placeholder));

    match catalog.items.get(100) {
        Some(amount) => {
            println!("Catalog number {:#?}", amount)
        }
        None => {
            println!("There is nothing in here")
        }
    }

    match catalog.get_items(100) {
        Some(id) => {
            println!("Item is {:#?}", id)
        }
        None => {
            println!("No value here")
        }
    }

    // println!("Book description {:#?}", book.description());
    // println!("Audio description {:#?}", movie.description());
    // println!("Movie description {:#?}", audio.description());
}
