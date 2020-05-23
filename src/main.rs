fn main() {
    use std::collections::HashMap;

    let text = "hello hello you pretty bastard";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // count is set to 0 the first time, or_insert returns a mut ref
        let count = map.entry(word).or_insert(0);

        //count needs to be de-referenced so it can hold the previous value and be mutilated 
        // I don't know, that's how I understand it now. Probably wrong
        *count +=1; // count becomes 1 if it was nothing
    }

    // will print acutal count for each word in text
    println!("{:?}", map); 
}