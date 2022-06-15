use rand::Rng;
use rand::thread_rng;

#[macro_use]
extern crate colour;
#[derive(Debug)]
struct Wordle {
    to_day_word : String,
    guess_word : String,
    is_game_over: bool,
    number_of_treis: i8,
}


impl Wordle {
    fn print_info(&self){
        print!("the right word is ");
        blue_ln!("{:?}" , self.to_day_word)
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
                green!(" \t {}", guess_word_vec[i])
            }
            else {
                if today_wrod_vec.contains(&guess_word_vec[i]) {
                    yellow!("\t {}" , guess_word_vec[i])
                }
                else {
                    red!("\t {}" , guess_word_vec[i])
                }
            }
        }
        println!()
    }
        
}


fn main () {
    let word_list = vec!["abuse"
    ,"adult"
    ,"agent"
    ,"anger"
    ,"apple"
    ,"award"
    ,"basis"
    ,"beach"
    ,"birth"
    ,"block"
    ,"blood"
    ,"board"
    ,"brain"
    ,"bread"
    ,"break"
    ,"brown"
    ,"buyer"
    ,"cause"
    ,"chain"
    ,"chair"
    ,"chest"
    ,"chief"
    ,"child"
    ,"china"
    ,"claim"
    ,"class"
    ,"clock"
    ,"coach"
    ,"coast"
    ,"court"
    ,"cover"
    ,"cream"
    ,"crime"
    ,"cross"
    ,"crowd"
    ,"crown"
    ,"cycle"
    ,"dance"
    ,"death"
    ,"depth"
    ,"doubt"
    ,"draft"
    ,"drama"
    ,"dream"
    ,"dress"
    ,"drink"
    ,"drive"
    ,"Earth"
    ,"Enemy"
    ,"Entry"
    ,"error"
    ,"event"
    ,"faith"
    ,"fault"
    ,"field"
    ,"fight"
    ,"final"
    ,"floor"
    ,"focus"
    ,"force"
    ,"frame"
    ,"frank"
    ,"front"
    ,"fruit"
    ,"glass"
    ,"grant"
    ,"grass"
    ,"green"
    ,"group"
    ,"guide"
    ,"heart"
    ,"henry"
    ,"horse"
    ,"hotel"
    ,"house"
    ,"image"
    ,"index"
    ,"input"
    ,"issue"
    ,"japan"
    ,"jones"
    ,"judge"
    ,"knife"
    ,"laura"
    ,"layer"
    ,"level"
    ,"lewis"
    ,"light"
    ,"limit"
    ,"lunch"
    ,"major"
    ,"march"
    ,"match"
    ,"metal"
    ,"model"
    ,"money"
    ,"month"
    ,"motor"
    ,"mouth"
    ,"music"
    ,"night"
    ,"noise"
    ,"north"
    ,"novel"
    ,"Nurse"
    ,"offer"
    ,"order"
    ,"other"
    ,"owner"
    ,"panel"
    ,"paper"
    ,"party"
    ,"peace"
    ,"peter"
    ,"phase"
    ,"phone"
    ,"piece"
    ,"pilot"
    ,"pitch"
    ,"place"
    ,"plane"
    ,"plant"
    ,"plate"
    ,"point"
    ,"pound"
    ,"power"
    ,"press"
    ,"price"
    ,"pride"
    ,"prize"
    ,"proof"
    ,"queen"
    ,"radio"
    ,"range"
    ,"ratio"
    ,"reply"
    ,"right"
    ,"river"
    ,"round"
    ,"route"
    ,"rugby"
    ,"scale"
    ,"scene"
    ,"scope"
    ,"score"
    ,"sense"
    ,"shape"
    ,"share"
    ,"sheep"
    ,"sheet"
    ,"shift"
    ,"shirt"
    ,"shock"
    ,"sight"
    ,"simon"
    ,"skill"
    ,"sleep"
    ,"smile"
    ,"smith"
    ,"smoke"
    ,"sound"
    ,"south"
    ,"space"
    ,"speed"
    ,"spite"
    ,"sport"
    ,"squad"
    ,"staff"
    ,"stage"
    ,"start"
    ,"state"
    ,"steam"
    ,"steel"
    ,"stock"
    ,"stone"
    ,"store"
    ,"study"
    ,"stuff"
    ,"style"
    ,"sugar"
    ,"table"
    ,"taste"
    ,"terry"
    ,"theme"
    ,"thing"
    ,"title"
    ,"total"
    ,"touch"
    ,"tower"
    ,"track"
    ,"trade"
    ,"train"
    ,"trend"
    ,"trial"
    ,"trust"
    ,"truth"
    ,"uncle"
    ,"union"
    ,"unity"
    ,"value"
    ,"video"
    ,"visit"
    ,"voice"
    ,"waste"
    ,"watch"
    ,"water"
    ,"while"
    ,"white"
    ,"whole"
    ,"woman"
    ,"world"
    ,"youth"];
    let mut rng = thread_rng();

    let mut wordle_one = Wordle {
        to_day_word: String::from(word_list[rng.gen_range(0..word_list.len())]),
        guess_word: String::from(""),
        is_game_over: false,
        number_of_treis: 0
    };

    for _i in 0..6 {
        if !wordle_one.is_game_over {
            wordle_one.read_user_guess();
            wordle_one.evaluate_guess();
        }
    }

    wordle_one.print_info();
  
}