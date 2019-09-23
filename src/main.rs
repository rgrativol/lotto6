mod models;
mod stats;
mod frequency_stack;

use std::path::Path;
use models::load_csv;

fn main() {
    let csv_path = Path::new("./data/lotto_6.csv");
    let draws = load_csv(csv_path);
}
