#[derive(Debug)]
struct Museum {
    paintings: Vec<String>,// names of paintings
    value:u32, // money for the painting
}

impl Museum{
    fn new()->Self{
        Self { paintings: vec![], value:0 }
    }

    // add a painting into vector
    fn buy_painting(&mut self, painting:&str){
        self.paintings.push(painting.to_string());
    }

    // set the value of a painting
    fn sell_ticket(&mut self){
        // assume that the value of all apintings is $ 25
        self.value +=25;
    }

    fn has_impressive_collection(&self)->bool{
        self.paintings.len()>2
    }
}

// we can create a mod and put the test functions in it
// the name of mod can can be given arbitralily from us
mod tests{
    #[test]
    fn museum_sells_ticket_to_increase_revenue(){
        // create an new mut instance of Museum
        // invokesell_ticket method
        // assert result

        // what about crate keyword?
        // crate keyword provides an absolute path
        // starting from the root which is the lib crate
        // so the code above
        let mut museum = crate::Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.value, 25);
    }
}

// in the mod tests we can have fn without the #[test]
// attribute just as helpful function for the test functions
// compiler can accept it