use std::collections::HashMap;

// Subject trait
trait Image {
    fn display(&mut self);
}

// RealImage struct
struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: &str) -> Self {
        println!("Loading image: {}", filename);
        Self {
            filename: filename.to_string(),
        }
    }
}

impl Image for RealImage {
    fn display(&mut self) {
        println!("Displaying image: {}", self.filename);
    }
}

// ProxyImage struct with caching
struct ProxyImage {
    filename: String,
    cache: HashMap<String, RealImage>,
}

impl ProxyImage {
    fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            cache: HashMap::new(),
        }
    }

    fn display(&mut self) {
        if !self.cache.contains_key(&self.filename) {
            println!("Proxy is loading the real image...");
            let real_image = RealImage::new(&self.filename);
            self.cache.insert(self.filename.clone(), real_image);
        }
        self.cache.get_mut(&self.filename).unwrap().display();
    }
}

impl Image for ProxyImage {
    fn display(&mut self) {
        self.display();
    }
}

fn main() {
    let mut image1 = ProxyImage::new("image1.jpg");
    let mut image2 = ProxyImage::new("image2.jpg");

    // First display will load and cache images
    image1.display();
    image2.display();

    // Subsequent displays use cached images
    image1.display();
    image2.display();
}
