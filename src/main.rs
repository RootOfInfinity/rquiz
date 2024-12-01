extern crate json;

use std::{fs, io};

use json::JsonValue;
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct Quiz {
    name: String,
    cards: Vec<Card>,
}
impl Quiz {
    fn new(json: JsonValue) -> Self {
        let name = json["name"].as_str().unwrap().to_string();
        let cardlen = json["cards"][0].as_i32().unwrap();
        let mut cards: Vec<Card> = Vec::new();
        for i in 1..=cardlen {
            let obj = json["cards"][i as usize].clone();
            cards.push(Card {
                id: i - 1,
                fr: obj["fr"].as_str().unwrap().to_string(),
                bk: obj["bk"].as_str().unwrap().to_string(),
            });
        }
        Quiz { name, cards }
    }
    fn disp(&self) {
        println!("{}", self.name);
        println!();
        for i in 0..self.cards.len() {
            println!("Front: {}", self.cards[i].fr);
            println!("Back: {}", self.cards[i].bk);
            println!();
        }
    }
    fn mult_choice(&self, anscard: Card) -> Option<Wrong> {
        let mut rng = thread_rng();
        let mut ansvec = vec![anscard.clone()];
        for _ in 0..3 {
            loop {
                let ind = rng.gen_range(0..self.cards.len());
                if !ansvec.contains(&self.cards[ind]) {
                    ansvec.push(self.cards[ind].clone());
                    break;
                }
            }
        }
        let mut temp = vec![];
        for _ in 0..ansvec.len() {
            temp.push(ansvec.remove(rng.gen_range(0..ansvec.len()) as usize));
        }
        ansvec = temp;
        println!("What is on the back of: {}", anscard.fr);
        for (i, card) in ansvec.iter().enumerate() {
            println!("{}. {}", i + 1, card.bk);
        }
        let chosen = {
            let mut temp = input_i32();
            loop {
                if temp > 0 && temp <= 4 {
                    break;
                } else {
                    println!("Please choose a number between 1-4");
                    temp = input_i32();
                }
            }
            temp
        };
        if ansvec[chosen as usize - 1].id == anscard.id {
            //correct
            println!("CORRECT!");
            println!();
            return None;
        } else {
            //incorrect

            let right_num = {
                let mut temp = None;
                for (i, coolcard) in ansvec.iter().enumerate() {
                    if coolcard.id == anscard.id {
                        temp = Some(i as i32);
                        break;
                    }
                }
                match temp {
                    Some(n) => n + 1,
                    None => panic!("Critical error, correct answer is not in ansvec"),
                }
            };
            let log = Wrong::Mult(MultWrong {
                question: format!("What is behind: {}", anscard.fr),
                right_ans_num: right_num,
                right_ans: anscard.bk.clone(),
                wrong_ans_num: chosen,
                wrong_ans: ansvec[chosen as usize - 1].bk.clone(),
            });
            println!("Sorry bro, incorrect.");
            println!("The answer was actually #{}: \"{}\"", right_num, anscard.bk);
            println!("A log of your missed answers will be available at the end of the quiz.");
            println!();
            return Some(log);
        }
    }
    fn mult_choice_test(&self) {
        if self.cards.len() < 4 {
            println!("You need at least 4 cards to do Multiple Choice mode.");
            return ();
        }
        let mut quiz = self.clone();
        //shuffle it
        let mut temp: Vec<Card> = Vec::with_capacity(quiz.cards.len());
        let mut rng = thread_rng();
        for _ in 0..quiz.cards.len() {
            temp.push(
                quiz.cards
                    .remove(rng.gen_range(0..quiz.cards.len()) as usize),
            );
        }
        quiz.cards = temp;

        //for saving incorrect values as a summary after the test
        struct SavedIncorrect {
            question: String,
            right_ans_num: i32,
            right_ans: String,
            wrong_ans_num: i32,
            wrong_ans: String,
        }

        let mut wronglogs: Vec<SavedIncorrect> = Vec::new();
        let mut correct = 0;

        for card in &quiz.cards {
            println!("What is behind: {}", card.fr);
            let mut ansvec = vec![card.clone()];
            for _ in 0..3 {
                loop {
                    let ind = rng.gen_range(0..quiz.cards.len());
                    if !ansvec.contains(&quiz.cards[ind]) {
                        ansvec.push(quiz.cards[ind].clone());
                        break;
                    }
                }
            }
            let mut temp = vec![];
            for _ in 0..ansvec.len() {
                temp.push(ansvec.remove(rng.gen_range(0..ansvec.len()) as usize));
            }
            ansvec = temp;
            //cant use temp anymore
            for (i, c) in ansvec.iter().enumerate() {
                println!("{}. {}", i + 1usize, c.bk);
            }
            println!();
            let chosen = {
                let mut temp = input_i32();
                loop {
                    if temp > 0 && temp <= 4 {
                        break;
                    } else {
                        println!("Please choose a number between 1-4");
                        temp = input_i32();
                    }
                }
                temp
            };
            if ansvec[chosen as usize - 1].id == card.id {
                //correct
                println!("CORRECT!");
                println!();
                correct += 1;
            } else {
                //incorrect

                let right_num = {
                    let mut temp = None;
                    for (i, coolcard) in ansvec.iter().enumerate() {
                        if coolcard.id == card.id {
                            temp = Some(i as i32);
                            break;
                        }
                    }
                    match temp {
                        Some(n) => n + 1,
                        None => panic!("Critical error, correct answer is not in ansvec"),
                    }
                };
                let log = SavedIncorrect {
                    question: format!("What is behind: {}", card.fr),
                    right_ans_num: right_num,
                    right_ans: card.bk.clone(),
                    wrong_ans_num: chosen,
                    wrong_ans: ansvec[chosen as usize - 1].bk.clone(),
                };
                wronglogs.push(log);
                println!("Sorry bro, incorrect.");
                println!("The answer was actually #{}: \"{}\"", right_num, card.bk);
                println!("A log of your missed answers will be available at the end of the quiz.");
                println!();
            }
        }
        println!("You have completed the {} quiz", quiz.name);
        println!("Stats: {}/{} correct, {}%", correct, quiz.cards.len(), {
            let ans: f64;
            ans = correct as f64 / quiz.cards.len() as f64;
            ans
        });
        if wronglogs.len() > 0 {
            println!(
                "It looks like you got some answers wrong. Would you like to look at the logs?"
            );
            println!("1. yes\n2. no");
            let choice = {
                let mut temp = input_i32();
                loop {
                    if temp == 1 || temp == 2 {
                        break;
                    } else {
                        temp = input_i32();
                    }
                }
                temp
            };
            if choice == 2 {
                println!("Sounds good, you probably know what you missed.");
            } else {
                for log in wronglogs {
                    println!("Question: {}", log.question);
                    println!("Your answer:\n{}. {}", log.wrong_ans_num, log.wrong_ans);
                    println!(
                        "The right answer:\n{}. {}",
                        log.right_ans_num, log.right_ans
                    );
                    println!();
                }
            }
        }
    }
    fn typing(&self, anscard: Card) -> Option<Wrong> {
        println!("What is on the back of: {}", anscard.fr);

        let mut buf = String::from("");
        match io::stdin().read_line(&mut buf) {
            Ok(_o) => (),
            Err(e) => {
                eprintln!("Error in reading user input. Error: {}", e);
                panic!("AHH!");
            }
        };
        buf = buf.trim().to_lowercase();
        if buf == anscard.bk.to_lowercase() {
            println!("Correct!");
            println!();
            return None;
        } else {
            println!("Incorrect");
            println!("A log of your incorrect answers will be available at the end.");
            println!();
            let log = Wrong::Type(TypeWrong {
                question: format!("What is on the back of: {}", anscard.fr),
                wrong_ans: buf,
                correct_ans: anscard.bk.clone(),
            });
            return Some(log);
        }
    }
    fn typing_test(&self) {
        let mut quiz = self.clone();
        let mut rng = thread_rng();
        //shuffles the deck
        let mut temp: Vec<Card> = vec![];
        for _ in 0..quiz.cards.len() {
            temp.push(quiz.cards.remove(rng.gen_range(0..quiz.cards.len())));
        }
        quiz.cards = temp;
        struct IncorrectAnswer {
            question: String,
            wrong_ans: String,
            correct_ans: String,
        }
        let mut wronglogs: Vec<IncorrectAnswer> = vec![];
        let mut correct: i32 = 0;
        for card in quiz.cards.iter() {
            println!("What is on the back of: {}", card.fr);
            let mut buf = String::from("");
            match io::stdin().read_line(&mut buf) {
                Ok(_o) => (),
                Err(e) => {
                    eprintln!("Error in reading user input. Error: {}", e);
                    panic!("AHH!");
                }
            };
            buf = buf.trim().to_lowercase();
            if buf == card.bk {
                println!("Correct!");
                println!();
                correct += 1;
            } else {
                println!("Incorrect");
                println!("A log of your incorrect answers will be available at the end.");
                println!();
                let temp = IncorrectAnswer {
                    question: format!("What is on the back of: {}", card.fr),
                    wrong_ans: buf,
                    correct_ans: card.bk.clone(),
                };
                wronglogs.push(temp);
            }
        }
        println!("Complete!");
        println!("Stats: {}/{} correct or {}%", correct, quiz.cards.len(), {
            correct as f64 / quiz.cards.len() as f64
        });
        if wronglogs.len() > 0 {
            println!(
                "It looks like you got some questions wrong, would you like to look at the logs?"
            );
            println!("1. yes\n2. no");
            let choice = {
                let mut temp = input_i32();
                loop {
                    if temp == 1 || temp == 2 {
                        break;
                    } else {
                        temp = input_i32();
                    }
                }
                temp
            };
            if choice == 2 {
                println!("Sounds good, you probably know what you missed.");
            } else {
                for log in wronglogs {
                    println!("Question: {}", log.question);
                    println!("Your answer:\n{}", log.wrong_ans);
                    println!("Right answer:\n{}", log.correct_ans);
                    println!();
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Card {
    id: i32,
    fr: String,
    bk: String,
}

enum Wrong {
    Mult(MultWrong),
    Type(TypeWrong),
}
struct MultWrong {
    question: String,
    right_ans_num: i32,
    right_ans: String,
    wrong_ans_num: i32,
    wrong_ans: String,
}
struct TypeWrong {
    question: String,
    wrong_ans: String,
    correct_ans: String,
}
fn input_i32() -> i32 {
    print!("> ");
    let mut buf = String::from("");
    match io::stdin().read_line(&mut buf) {
        Ok(_o) => (),
        Err(e) => {
            eprintln!("Error occured, invalid read of data: {}", e);
            return input_i32();
        }
    };
    let ans = match buf.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Not a valid number: {}", e);
            input_i32()
        }
    };
    ans
}

fn main() {
    let test_data = json::parse(
        fs::read_to_string("./tests/quiz1.json")
            .expect("no file in sight")
            .as_str(),
    )
    .expect("Not valid JSON!");
    let quiz: Quiz = Quiz::new(test_data);
    quiz.disp();
    quiz.mult_choice(quiz.cards[0].clone());
    quiz.typing(quiz.cards[0].clone());
}
