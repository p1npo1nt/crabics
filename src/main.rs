mod bucket;
mod drop; //all modules used in this main file

use crate::bucket::Bucket;
use crate::drop::pour::Pour;

fn main() {
    let mut bucket = Bucket::new();
    let action = Pour::new(2.5, "Apple Juice");

    bucket.pour(action);

    println!("Bucket now contains: {}", bucket.describe());
}
