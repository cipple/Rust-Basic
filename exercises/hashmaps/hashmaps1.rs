// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("mango"), 3);
    basket.insert(String::from("pear"), 4);
    basket.insert(String::from("apple"), 5);
    basket.insert(String::from("peach"), 6);
    basket.insert(String::from("watermelon"), 7);
    basket.insert(String::from("grapes"), 8);
    basket.insert(String::from("strawberry"), 9);
    basket.insert(String::from("lemon"), 10);
    basket.insert(String::from("lime"), 11);
    basket.insert(String::from("orange"), 12);
    basket.insert(String::from("pineapple"), 13);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
