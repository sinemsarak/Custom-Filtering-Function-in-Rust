use std::io;

struct FilterCondition<T>{
    field: T,
}

impl <T: PartialEq>  FilterCondition<T>{
    fn is_match(&self, item: &T) -> bool {
        item == &self.field
    }
}

fn custom_filter<T>(input: &[T], filter: &FilterCondition<T>) -> Vec<T> 
    where 
        T: PartialEq,
        T: Clone,
    {
        input
            .iter()
            .filter(|item| filter.is_match(item))
            .cloned()
            .collect()
    }

fn main() {
    let input_letter: char;

    loop {
        println!("Please provide a letter to be counted within the text.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");

        if let Some(letter) = input.trim().chars().next() {
            input_letter = letter;
            break;
        } else {
            println!("Invalid input. Please enter a single character.");
        }
    }
    println!("Please enter a text for which you would like to determine the number of the letter {} ", input_letter);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    
    let string_slice: &str = input.trim();
    let filter_condition = FilterCondition { field: input_letter};
    let filtered_letters = custom_filter(string_slice.chars().collect::<Vec<char>>().as_slice(), &filter_condition); // String'i karakterlerine ayırarak filtreleme işlemi yapıldı
    println!("There are {} {}'s in the text. And here they are in order :D : {:?}",filtered_letters.len(), input_letter,filtered_letters);
}
