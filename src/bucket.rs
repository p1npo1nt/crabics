use crate::drop::pour::Pour;

pub struct Bucket {
    contents: Vec<String>,
}

impl Bucket {
    pub fn new() -> Self {
        Bucket {
            contents: Vec::new(),
        }
    }

    pub fn pour(&mut self, pour: Pour) {
        let description = format!("{} liters of {}", pour.amount, pour.name);
        self.contents.push(description);
    }

    pub fn describe(&self) -> String {
        self.contents.join(", ")
    }
}