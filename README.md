# Hello-Rust
Repo for my Rust development documents, challenges and mini projects.

## What's in this repo?
### 1- Implement a basic program that uses ownership concepts
A simple application that demonstrates Ownership concept with Rust language.

### 2- Create a simple calculator using enums and pattern matching

### Steps
1. Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
2. Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
3. Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
4. In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
5. Parse the user input into appropriate variables.
6. Create an Operation enum instance with the parsed input values.
7. Call the calculate function with the created Operation enum instance.
8. Print the result to the console.
9. Compile and run the program to ensure it works as expected.

### Checklist
✅ Define the Operation enum with the appropriate variants and values.

✅ Write the calculate function signature.

✅ Implement the calculate function using pattern matching to perform the appropriate arithmetic operation.

✅ Prompt the user to input the first number, operation, and second number.

✅ Parse the user input into appropriate variables.

✅ Create an Operation enum instance with the parsed input values.

✅ Call the calculate function with the created Operation enum instance.

✅ Print the result to the console.

✅ Compile and run the program to test its functionality.


### 2- Building a Custom Filtering Function in Rust

### Steps
1. Define a struct called FilterCondition with a single field of the desired type for filtering.
2. Implement a method called is_match on the FilterCondition struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.
3. Define a function called custom_filter that takes a collection (e.g., a vector) and a reference to a FilterCondition object as arguments. The function should iterate over the elements in the collection and return a new collection containing only the elements that match the filter condition.
4. In the main function, create a collection (e.g., a vector) with some elements and initialize a FilterCondition object with the desired value.
5. Call the custom_filter function with the collection and the FilterCondition object, storing the result in a new variable.
6. Print the filtered result to the console.Compile and run the program to test its functionality.

### Checklist
✅ Define the FilterCondition struct with the desired type for filtering.

✅ Implement the is_match method on the FilterCondition struct.

✅ Define the custom_filter function to filter elements based on the condition.

✅ Create a collection and a FilterCondition object in the main function.

✅ Call the custom_filter function and store the result.

✅ Print the filtered result to the console.

✅ Compile and run the program to test its functionality.

### Installation
To run the Rust file with [VS Code.](https://code.visualstudio.com/)
After pasting the codes into your editor, open a terminal and enter the commands below.

```sh
cargo init
cargo build
cargo run
```

### License
MIT
**Free Software, Hell Yeah!**
