/*
1- Create a new Rust project and open the main.rs file.
2- Define the FilterCondition struct with the desired type for filtering.
3- Implement the is_match method on the FilterCondition struct.
4- Define the custom_filter function to filter elements based on the condition.
5- Create a collection and a FilterCondition object in the main function.
6- Call the custom_filter function and store the result.
7- Print the filtered result to the console.
8- Compile and run the program to test its functionality.
*/

fn main() {
    
    let elements = vec![1,2,3,5,7,10]; // Create a collection (vector) with some elements
    
    let filter_condition = FilterCondition{value: 2}; // Initialize a FilterCondition object with the desired value

    let filtered_condition = custom_filter(elements, &filter_condition); // Call the custom_filter function with the collection and FilterCondition

    println!("{:?}", filtered_condition); //Printing 
  
}

// Define the FilterCondition struct
struct FilterCondition<T> {
    
    value: T,
    
}

//Implement the is_match method for FilterCondition
impl <T: PartialEq> FilterCondition<T> {

    fn is_match(&self, item:&T) -> bool {
        item == &self.value 
    }

}

//Define the custom_filter function
fn custom_filter<T>(items: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>

where
    T: PartialEq,
{
    items.into_iter().filter(|item| condition.is_match(item)).collect()
}
