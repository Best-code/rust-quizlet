use std::{
    collections::HashMap,
    default,
    fs::File,
    hash::Hash,
    io::{self, BufRead, BufReader, Read, Write},
    process::{exit, Output},
};

use rand::{Rng, seq::SliceRandom};

fn main() {
    let mut delim: String = String::from(" answer: ");
    let mut file: String = String::from("./fos_exam.txt");

    let mut question_answer: HashMap<String, String> = fill_hashmap(&file, &delim);

    menu(&mut question_answer, &mut delim, &mut file);
}

fn menu(question_answer: &mut HashMap<String, String>, delim: &mut String, file: &mut String) {
    loop {
        println!("Welcome to Rust-Quizlet!");
        println!(
            "Your current study set is {}, with the delimiter being '{}'\n",
            file, delim
        );
        println!("1. Study");
        println!("2. Test");
        println!("3. Add Flash Card");
        println!("4. Set Delimiter");
        println!("5. Set Study Set");
        println!("6. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                study(question_answer);
            }
            "2" => {
                test(&question_answer);
            }
            "3" => {
                *question_answer = add_qa(delim, file);
            }
            "4" => {
                change_delim(question_answer, file, delim);
            }
            "5" => {
                change_file(question_answer, file, delim);
            }
            _default => break,
        }
    }
}

fn change_delim(
    question_answer: &mut HashMap<String, String>,
    file: &mut String,
    delim: &mut String,
) {
    print!("What would you like the new delimiter to be: ");
    delim.clear();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    delim.push_str(input.trim_end_matches("\n"));

    *question_answer = fill_hashmap(file, delim);
}

fn change_file(
    question_answer: &mut HashMap<String, String>,
    file: &mut String,
    delim: &mut String,
) {
    println!("What would you like the new study set to be: ");
    file.clear();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    file.push_str(input.trim());
    println!("{} is your new study set", file);

    *question_answer = fill_hashmap(file, delim);
}

fn study(question_answer: &HashMap<String, String>) {
    // Length for finding a kv in bounds
    let length = question_answer.len();

    // Studying each individual term
    for _ in 0..question_answer.len() {
        // Getting random kv pair
        let (_qa_pair, question, answer) = get_kv(length, question_answer);
        // Requiring input to move on
        println!("{question}\nPress enter to see answer.");

        let mut enter = String::new();
        io::stdin().read_line(&mut enter).unwrap();

        // Allow exit of mode through entering 0
        if enter.trim() == "0" {
            break;
        }

        println!("{answer}\n");
        // Requiring input to move on
        io::stdin().read_line(&mut enter).unwrap();
    }
}

fn test(question_answer: &HashMap<String, String>) {
    println!("How many questions would you like: ");
    let mut question_count_input = String::new();
    io::stdin().read_line(&mut question_count_input).unwrap();
    let question_count: i32 = question_count_input.trim().parse().unwrap_or(10);

    // Keep track of how many correct on first try
    let mut score = 0;
    // Determine whether question is a multiple choice or a true_false question
    for _ in 0..question_count {
        let tf_or_mc = rand::thread_rng().gen_bool(1.0 / 2.0);
        if tf_or_mc {
            score = mult_choice(&mut score, &question_answer);
        } else {
            score = true_false(&mut score, &question_answer);
        }

        if score == -1 {
            break;
        }
    }

    println!("You scored a {score}/{question_count}");
}

fn fill_hashmap(file: &str, delim: &str) -> HashMap<String, String> {
    let mut question_answer: HashMap<String, String> = HashMap::new();

    let mut question = String::new();
    let mut answer = String::new();

    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);
    // Get all the lines from the file
    for line in reader.lines() {
        // Read each line individual
        // Turn line in to readable string
        if let Ok(to_be) = line {
            let read_in = String::from(to_be);
            // Find where the delimiter is
            if read_in.find(delim) != None {
                // If delimiter exist on this line we must be at the question part
                if let Some(split) = read_in.find(delim) {
                    // If this is the question part and we already have a previous quetsion then
                    // push to question_answer and clear the values
                    if !question.is_empty() && !answer.is_empty() {
                        question_answer.insert(question.clone(), answer.clone());
                        question.clear();
                        answer.clear();
                    }
                    question.push_str(&read_in[0..split]);
                    answer.push_str(&read_in[split + delim.len()..]);
                }
            } else {
                answer.push_str(&read_in);
            }
        }
    }

    // Adding any left over lines
    if !question.is_empty() && !answer.is_empty() {
        question_answer.insert(question.clone(), answer.clone());
    }

    question_answer
}

fn get_kv(length: usize, qna_s: &HashMap<String, String>) -> (usize, &String, &String) {
    // Find random answer question pair
    let qa_pair: usize = rand::thread_rng().gen_range(0..length);
    // Assign respective part to variable
    let question: &String = qna_s.keys().nth(qa_pair).unwrap();
    let answer: &String = qna_s.get(question).unwrap();

    (qa_pair, question, answer)
}

fn true_false(score: &mut i32, qna_s: &HashMap<String, String>) -> i32 {
    // Get length of qna_s so we can find random KV pair in bounds
    let qna_len = qna_s.len();
    // Set KV pair to respective variables
    let (qa_pair, question, answer) = get_kv(qna_len, qna_s);

    // Get a random answer and give it a 50% chance of being right
    let chance = rand::thread_rng().gen_bool(1.0 / 2.0);
    let mut random_answer = String::new();
    if chance {
        let random_answer_index = rand::thread_rng().gen_range(0..qna_len);
        random_answer.push_str(qna_s.values().nth(random_answer_index).unwrap());
    } else {
        random_answer.push_str(&answer);
    }

    let mut correct_index: usize = 2;
    let options = ["True", "False", "", ""];

    // If the random and the real answer are correct, set correct_index to True
    if &random_answer == answer {
        correct_index = 1;
    }

    let mut choice = 5;
    // Print all questions
    println!("\n{question}\n{random_answer}\n\n");

    for (index, option) in options.iter().enumerate() {
        if options[index] != "" {
            println!("{}: {option}", index + 1);
        }
    }

    println!("\nEnter your guess: ");

    // Take input and store it in string
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // Convert string to usize
    choice = input.trim().parse().unwrap_or(1);

    println!("You chose {choice}");

    if choice == correct_index {
        *score += 1;
        println!("That's correct!");
    } else {
        println!("That's incorrect.");
    }

    if choice == 0 {
        *score = -1;
    }
    *score
}

fn mult_choice(score: &mut i32, qna_s: &HashMap<String, String>) -> i32 {
    // Hold length of qna_size so we can use in random number generator bounds
    let qna_len = qna_s.len();

    // Grab a random K,V pair from qna
    let (qa_pair, question, answer) = get_kv(qna_len, qna_s);

    // Create array with four &strs
    let mut options: [&str; 4] = [""; 4];

    // Fill the array with options
    for option in 0..options.len() {
        let mut random_answer_index = qa_pair;
        // Find a random K,V pair, if it is the same as the correct one, find a new pair
        while random_answer_index == qa_pair {
            random_answer_index = rand::thread_rng().gen_range(0..qna_len);
        }
        let random_answer = qna_s.values().nth(random_answer_index).unwrap();
        options[option] = random_answer;
    }

    // Placing the correct answer into our options
    options[0] = answer;

    // Shuffling all answer choices around
    // let options = *shuffle_array(&mut options);
    shuffle_array(&mut options);

    // Finding which answer is correct
    let mut correct_index = 5;
    for (index, option) in options.iter().enumerate() {
        if option == answer {
            // Adding 1 for 1-4 and not 0-3
            correct_index = index + 1;
        }
    }

    // Create loop
    let mut choice = 5;
    let mut tries = 0;
    while choice != correct_index {
        // Print all questions
        println!("\nQuestion: {question}\n");
        for (index, option) in options.iter().enumerate() {
            println!("{}: {option}", index + 1);
        }

        println!("\nEnter your guess: ");

        // Take input and store it in string
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // Convert string to usize
        choice = input.trim().parse().unwrap_or(1);

        println!("You chose {choice}");

        if choice == correct_index {
            if tries == 0 {
                *score += 1;
            }
            println!("That's correct!");
            tries = 0;
        } else {
            println!("That's incorrect.");
            tries += 1;
        }
    }

    if choice == 0 {
        *score = -1;
    }
    *score
}

fn shuffle_array(options: &mut [&str; 4])
{
    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng)
}

fn add_qa(delim: &str, file_str: &str) -> HashMap<String, String> {
    // Reading in user input to respective variables
    let mut question = String::new();
    let mut answer = String::new();

    println!("Input the question: ");
    io::stdin().read_line(&mut question).unwrap();
    println!("Input the answer: ");
    io::stdin().read_line(&mut answer).unwrap();

    // Formatting before appending to file
    let qa_pair = String::from("\n\n".to_owned() + &question.trim() + delim + &answer.trim());

    // Appending to file
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(file_str)
        .unwrap();

    file.write(&qa_pair.as_bytes()).unwrap();

    // Returning new hashmap
    let question_answer = fill_hashmap(file_str, delim);

    question_answer
}

