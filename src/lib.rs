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

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        // create an new mut instance of Museum
        // invokesell_ticket method
        // assert result

        // and now we can call Museum directly
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.value, 25);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.value, 50); // there is a failure
    }
}
// we we have multiple tests to run rust runs then in
// multiple threads. so if one test fails the program
// does not exit but still runs the rest of the tests.
