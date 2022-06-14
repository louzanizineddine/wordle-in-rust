
#[derive(Debug)]
struct Wordle {
    to_day_word : String,
    guess_word : String,
    is_game_over: bool,
    number_of_treis: i8,
}


impl Wordle {
    fn print_info(&self){
        println!("the right word is {:?}" , self.to_day_word);
    }


    fn read_user_guess(&mut self) {
        if self.number_of_treis == 0 {
            println!("Guess a five letter word you have only six tries");
        } else {
            println!("try again ......")
        }
        self.guess_word = String::from("");
        std::io::stdin().read_line(&mut self.guess_word).unwrap();
        self.trim_guess();
    } 

    fn evaluate_guess(&mut self) {
        self.number_of_treis = self.number_of_treis + 1;
        if self.guess_word == self.to_day_word {
            self.is_game_over = true;
            println!("You gussed it right .... wow ... and only {} tries " , self.number_of_treis)
        }else {
            self.eval_every_letter();
        }
    }

    // when I read input from the user using the stdin.read_line
    // it includes the new line for example "zineddine\n"
    // this function removes \n
    fn trim_guess(&mut self){
        self.guess_word.pop();
    }


    fn eval_every_letter(&self) {
        // convecting the two strings into a vector of chars so it's easier to index them     
        let today_wrod_vec: Vec<char> = self.to_day_word.chars().collect();
        let guess_word_vec: Vec<char> = self.guess_word.chars().collect();
        
        for i in 0..today_wrod_vec.len() {
            if guess_word_vec[i] == today_wrod_vec[i] {
                println!("the {} letter is in the right place" , guess_word_vec[i])
            }
            else {
                if today_wrod_vec.contains(&guess_word_vec[i]) {
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
        is_game_over: false,
        number_of_treis: 0
    };

    for _i in 0..5 {
        if !wordle_one.is_game_over {
            wordle_one.read_user_guess();
            wordle_one.print_info();
            wordle_one.evaluate_guess();
        }

    }
  
}