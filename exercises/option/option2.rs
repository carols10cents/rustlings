// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    let optional_value = Some(String::from("rustlings"));
    //make this an if let statement - value is "Some" type
    value = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("optional value does not have a value!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // you can stack Option<T>'s into while let and if let
    value = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
