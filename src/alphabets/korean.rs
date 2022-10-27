use std::collections::HashMap;

use super::Alphabet;

pub struct KoreanData<'a>
{
    korean_alphabet: HashMap<char, &'a str>
}

impl<'a> KoreanData<'a> {
    /// Generating a new object which contains a mapping of different korean symbols to a latin equivalent.
    pub fn new() -> Self {
         
        let korean_simple_vocals = HashMap::from([
            ('ㅏ', "a"),
            ('ㅓ', "eo"),
            ('ㅗ', "o"),
            ('ㅜ', "u"),
            ('ㅡ',"eu"), 
            ('ㅣ', "i"),
            ('ㅐ', "ae"),
            ('ㅔ', "e"),
            ('ㅚ', "oe"), 	
            ('ㅟ', "wi")
        ]);
        let korean_diphthonge = HashMap::from([
            ('ㅑ', "ya"),
            ('ㅕ', "yeo"),
            ('ㅛ', "yo"),
            ('ㅠ', "yu"),
            ('ㅒ',"yae"), 
            ('ㅖ', "ye"),
            ('ㅘ', "wa"),
            ('ㅙ', "wae"),
            ('ㅝ', "wo"), 	
            ('ㅞ', "we"),
            ('ㅢ', "ui")
        ]);
        
        // Join dictionaries:
        let vocals : HashMap<char, &'a str>= korean_simple_vocals.into_iter().chain(korean_diphthonge).collect();
        Self { korean_alphabet : vocals } 
        
        
        }
}


/// Returning the saved alphabet.
impl<'a> Alphabet<'a> for KoreanData<'a>
{
    fn alphabet(&self) -> &HashMap<char, &'a str> {
        &self.korean_alphabet
    }
}