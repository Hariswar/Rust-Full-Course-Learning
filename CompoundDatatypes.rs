//Compound Data Types - Chapter 2
//arrays, tuples, slices and string (slic string)

//Arrays
fn main() {
    //Data type first, then size of the array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    //Arrays for strings.
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);
    ///////////////////////////////////////////////////

    //Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mixed Tuple: {:?}", my_mix_tuple);
}
