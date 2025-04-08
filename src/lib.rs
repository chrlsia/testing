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

#[test]
fn museum_sells_ticket_to_increase_revenue(){
    // create an new mut instance of Museum
    // invokesell_ticket method
    // assert result
    let mut museum = Museum::new();
    museum.sell_ticket();
    assert_eq!(museum.value, 25);
}