//drop module which allows us to add different things to a bucket

pub struct Pour {
    pub amount: f32, //amount to pour in liters
    pub name: String, //liquid name
}

impl Pour {
    pub fn new(amount: f32, name: &str) -> Self {
        Pour {
            amount,
            name: name.to_string(),
        }
    }

    fn add(mut self, n: f32) {
        self.amount = self.amount + n;
    }
}