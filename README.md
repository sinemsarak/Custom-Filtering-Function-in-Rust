# Custom-Filtering-Function-in-Rust
This is a simple command-line program written in Rust that allows you to determine the number of occurrences of a specific letter within a given text.

# How It Works
1- The program prompts you to enter a single letter to be counted within the text. After receiving your input, it validates that it is a single character and assigns it to the variable input_letter.
2- Next, you are prompted to enter the text for which you want to determine the number of occurrences of the letter you provided. Once you enter the text, it is read and stored in the input variable.
3- The program creates a FilterCondition instance with the provided input_letter. It applies the custom_filter function to the text, which filters out all the characters that match the provided letter.
4- The filtered characters are collected into a vector called filtered_letters.
5- Finally, the program displays the count of occurrences and the filtered letters in the order they appeared.

# Additional Details

The program utilizes a FilterCondition struct, which allows you to define custom filtering conditions based on the type T. 
In this case, T represents the type of the letter, which must implement the PartialEq trait.

The custom_filter function takes an input slice and a filter condition, and it returns a vector containing the filtered elements.
It uses the filter method to apply the filter condition to each element and the cloned method to clone the matching elements. 
This is necessary because characters are not cloneable by default.

The program utilizes the standard input/output library (std::io) for user input and output operations.


