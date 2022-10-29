
mod alphabets;
//use alphabets::Alphabet;

use console::style;
use console::Style;

struct GameState
{
    score : i32,
}


fn main() {
    
    let mut state = GameState {score: 0};

    let random_data = alphabets::parse_from_csv("data/korean.csv").unwrap();
    // The old static data
    let _korean_data = alphabets::korean::KoreanData::new();

    for (key, value) in random_data
    {
        println!("{}", style(key).cyan());
        //term.write_line("{}", key).unwrap();
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        
        let mut style = Style::new();
        if line.trim().eq(&value)
        {
            state.score += 1;
            style = style.green();
        }else
        {
            style = style.red();
            println!("Correct: {}", value);
        }
        println!("Score: {}", style.apply_to(state.score));
        print!("{}", (8u8 as char));

    }
}
