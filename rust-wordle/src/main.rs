use std::io::stdin;

#[derive(Debug)]
struct Wordle {
    to_day_word : String,
    guess_word : String,
    is_game_over: bool
}


impl Wordle {
    fn print_info(&self){
        println!("the word is {:?} and the gussed word is {:?}" , self.to_day_word , self.guess_word);
    }

    fn read_user_guess(&mut self) {
        println!("Enter your guess :");
        self.guess_word = String::from("");
        std::io::stdin().read_line(&mut self.guess_word).unwrap();
        self.trim_guess();
    } 

    fn evaluate_guess(&self) {
        if (self.guess_word == self.to_day_word) {
            println!("good guess")
        }else {
            self.eval_every_letter();
        }
    }

    fn trim_guess(&mut self){
        self.guess_word.pop();
    }

    fn eval_every_letter(&self) {
        
        let today_wrod_vec: Vec<char> = self.to_day_word.chars().collect();
        let guess_word_vec: Vec<char> = self.guess_word.chars().collect();
        
        for i in 0..today_wrod_vec.len() {
            if guess_word_vec[i] == today_wrod_vec[i] {
                println!("the {} letter is in the right place" , guess_word_vec[i])
            }
            else {
                if (today_wrod_vec.contains(&guess_word_vec[i])) {
                    println!("the {} letter exists but not in this place " , guess_word_vec[i])
                }
                else {
                    println!("the {} letter does not exist in the original word" , guess_word_vec[i])
                }
            }
        }

    }
        
}


fn main () {
    let mut wordle_one = Wordle {
        to_day_word: String::from("zined"),
        guess_word: String::from(""),
    };

    for i in 0..5 {
        wordle_one.read_user_guess();
        wordle_one.print_info();
        wordle_one.evaluate_guess();

    }
  
}