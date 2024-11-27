extern crate json;

use std::fs;

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
    fn mult_choice_test(&self) {
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
        //cant use temp after this
        //print front of card
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
        }
        //THINGS LEFT TO DO:
        //collect user input
        //log incorrect answers
        //make percentage of correct answers
    }
}

#[derive(Clone, PartialEq)]
struct Card {
    id: i32,
    fr: String,
    bk: String,
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
    quiz.mult_choice_test();
}
