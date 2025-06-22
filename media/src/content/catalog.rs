use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    pub fn get_items(&self, id: usize) -> Option<&Media> {
        if self.items.len() > id {
            Some(&self.items[id])
        } else {
            None
        }
    }
}
