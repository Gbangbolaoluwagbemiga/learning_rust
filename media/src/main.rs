mod content;
use content::catalog::Catalog;
use content::media::Media;

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
