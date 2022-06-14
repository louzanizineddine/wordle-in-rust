use std::io::stdin;

#[derive(Debug)]
struct Wordle {
    to_day_word : String,
    guess_word : String,
}


impl Wordle {
    fn print_info(&self){
        println!("the word is {:?} and the gussed word is {:?}" , self.to_day_word , self.guess_word);
    }

    fn read_user_guess(&mut self) {
        println!("Enter your name :");
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
        // self.guess_word.pop();
    }

    fn eval_every_letter(&self) {
        
        let today_wrod_vec: Vec<char> = self.to_day_word.chars().collect();
        let guess_word_vec: Vec<char> = self.guess_word.chars().collect();
        
        for i in 0..today_wrod_vec.len() {
            
        }

    }
        
}


fn main () {
    let mut wordle_one = Wordle {
        to_day_word: String::from("zined"),
        guess_word: String::from(""),
    };

    
    wordle_one.read_user_guess();
    wordle_one.print_info();
    wordle_one.evaluate_guess();
}