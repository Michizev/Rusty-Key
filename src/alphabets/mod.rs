// Adding the reference to the class here
pub mod korean;

use std::{collections::HashMap, fs::File, io::BufReader};
use serde::{Deserialize};
use std::error::Error;
 
/// Abstraction of the Alphabet as a Hashmap of a single character to a transcription. 
pub trait Alphabet<'a>
{
    fn alphabet(&self) -> &HashMap<char, &'a str>;
}

#[derive(Debug, Deserialize)]
pub struct LetterTranscription
{
    letter: char,
    transcript: String,
}


pub fn parse_from_csv(filename:&str) -> Result<HashMap<char, String>, Box<dyn Error>>{
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let mut rdr = csv::Reader::from_reader(reader);

    let mut map = HashMap::new();

    for (_i, result) in rdr.deserialize().enumerate() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: LetterTranscription = result?;
        //println!("{} {:?}", _i, record);

        map.insert(record.letter, record.transcript);
    }
    
    Ok(map)
}