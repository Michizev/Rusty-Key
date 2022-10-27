// Adding the reference to the class here
pub mod korean;

use std::collections::HashMap;

 
/// Abstraction of the Alphabet as a Hashmap of a single character to a transcription. 
pub trait Alphabet<'a>
{
    fn alphabet(&self) -> &HashMap<char, &'a str>;
}
