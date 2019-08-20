#[derive(Debug)]
struct Question {
    number1: i32,
    number2: i32,
    operation: String,
}

fn calc(question: Question) -> i32 {
    if question.operation == "+" {
        question.number1 + question.number2
        
    } else if question.operation == "-" {
        question.number1 - question.number2
    } else if question.operation == "*" {
        question.number1 * question.number2
    } else {
        println!("Invaild operation");
        12340426
    }


    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
}

fn main() {
    println!("Hello, calculator!");
    let seun_question = Question {
        number1: 10, number2: 9, operation: "*".to_string()
    };

    println!("{:#?}", seun_question);

    let answer = calc(seun_question);

    println!("The answer to your question is: {}", answer);
}
