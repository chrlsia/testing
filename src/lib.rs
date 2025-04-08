#[derive(Debug)]
struct Museum {
    paintings: Vec<String>, // names of paintings
    value: u32,             // money for the painting
}

impl Museum {
    fn new() -> Self {
        Self {
            paintings: vec![],
            value: 0,
        }
    }

    // add a painting into vector
    fn buy_painting(&mut self, painting: &str) {
        self.paintings.push(painting.to_string());
    }

    // set the value of a painting
    fn sell_ticket(&mut self) {
        // assume that the value of all apintings is $ 25
        self.value += 25;
    }

    fn has_impressive_collection(&self) -> bool {
        self.paintings.len() > 2
    }
}

// compiler do not include the test code with
// the binary file you build or run
// in other words execute the code below
// when we run tests
#[cfg(test)]
// we can create a mod and put the test functions in it
// the name of mod can can be given arbitralily from us
mod tests {

    // better to bring into to scope
    // all of the above module
    // * is the globe operator
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        // create an new mut instance of Museum
        // invokesell_ticket method
        // assert result

        // and now we can call Museum directly
        let mut museum = Museum::new();
        museum.sell_ticket();
        // assert_eq!(museum.value, 25);

        // here we introduce the assert_ne! macro
        assert_ne!(museum.value,0);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.value, 50); // there is a failure
    }

    #[test]
    fn museum_can_have_impressive_art_collection(){
        let mut museum = Museum::new();
        museum.buy_painting("Mona Lisa");
        museum.buy_painting("The Starry NIght");
        museum.buy_painting("Girl with a pearl earring");
        // for now the len of vec should be 3
        // duw to the fact that we bought 3 paintings
        // assert_eq!( museum.has_impressive_collection(),true);
       assert!(museum.has_impressive_collection());
    }
}
// we we have multiple tests to run rust runs then in
// multiple threads. so if one test fails the program
// does not exit but still runs the rest of the tests.

// assert! asserts that some value or condition is true
// assert! gets one arguments which evaluates to true or false
// if it is true the test pass otherwise fails

/* we want to have dependencies into our program BUT only for us for development and NOT to include them into binary code...=>
in Cargo.toml file create a new entry an under there put
the dependencies you want:

[dev-dependencies]
pretty_assertions="1.4.1"

under mod you have to use:
use pretty_assetions::assert_eq

*/