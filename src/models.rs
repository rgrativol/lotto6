use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub struct Draw {
    draw: String,
    date: String,
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
    n5: u8,
    n6: u8,
    n_bonus: u8,
}

impl Default for Draw{
    fn default() -> Self {
        Draw{
            draw: "".to_string(),
            date: "".to_string(),
            n1: 0,
            n2: 0,
            n3: 0,
            n4: 0,
            n5: 0,
            n6: 0,
            n_bonus: 0
        }
    }
}


pub fn load_csv<P: AsRef<Path>>(file_path: P) -> Result<Vec<Draw>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut draws : Vec<Draw> = Vec::new();
    for line in reader.lines() {
        let _line =  line?;
        let row: Vec<&str> = _line.split(",").collect();
        let mut _draw = Draw::default();
        for (index, column) in row.iter().enumerate() {
            let data = (*column).to_string();
            match index {
                0 => _draw.draw = data,
                1 => _draw.date = data,
                2 => _draw.n1 = data.parse::<u8>().unwrap(),
                3 => _draw.n2 = data.parse::<u8>().unwrap(),
                4 => _draw.n3 = data.parse::<u8>().unwrap(),
                5 => _draw.n4 = data.parse::<u8>().unwrap(),
                6 => _draw.n5 = data.parse::<u8>().unwrap(),
                7 => _draw.n6 = data.parse::<u8>().unwrap(),
                _ => {}
            }
        }
        draws.push(_draw);
    }
    Ok(draws)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_csv() {
        let csv_path = Path::new("./data/lotto_6.csv");
        let draws = load_csv(csv_path);
        assert_eq!(draws.unwrap().len(), 1041);
    }
}
