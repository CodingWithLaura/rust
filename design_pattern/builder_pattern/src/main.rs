fn main() {
    println!("Hello, world!");
}

struct Cake {
    layer_count: i32,
    is_cheesecake: bool,
    cookie_crumbs: bool,
    marshmallow_topping: bool,
    frosting: bool,
}

impl Cake {
    fn print(&self) {
        let layer_layers = if self.layer_count == 1 {
            "layer"
        } else {
            "layers"
        };
        let cake_bool = |val| if val { "" } else { "no " };
        let cake_cheesecake = if self.is_cheesecake { "cheesecake" } else { "" };
        println!(
            "This is a {} with {} {}, {}cookie_crumbs, {}marshmallow-topping and {} frosting",
            cake_cheesecake,
            self.layer_count,
            layer_layers,
            cake_bool(self.cookie_crumbs),
            cake_bool(self.marshmallow_topping),
            cake_bool(self.frosting)
        )
    }
}

struct CakeBuilder {
    layer_count: i32,
    is_cheesecake: bool,
    cookie_crumbs: bool,
    marshmallow_topping: bool,
    frosting: bool,
}

impl CakeBuilder {
    //the standard values can be specified
    //with the help of the constructor

    fn new() -> Self {
        CakeBuilder {
            layer_count: 1,
            is_cheesecake: false,
            cookie_crumbs: false,
            marshmallow_topping: false,
            frosting: true,
        }
    }

    //Now the methods must be defined to set every
    //configurable value seperately

    fn set_layer(mut self, val: i32) -> Self {
        self.layer_count = val;
        self
    }

    fn set_is_cheesecake(mut self, val: bool) -> Self {
        self.is_cheesecake = val;
        self
    }

    fn set_cookie_crumbs(mut self, val: bool) -> Self {
        self.cookie_crumbs = val;
        self
    }

    fn set_marshmallow_topping(mut self, val: bool) -> Self {
        self.marshmallow_topping = val;
        self
    }

    fn frosting(mut self, val: bool) -> Self {
        self.frosting = val;
        self
    }

    //Now the final build method gets implemented
    //using all predefined set-methods

    fn build(&self) -> Result<Cake, String> {
        let cake = Cake {
            layer_count: self.set_layer,
            is_cheesecake: self.set_is_cheesecake,
            cookie_crumbs: self.set_cookie_crumbs,
            marshmallow_topping: self.set_marshmallow_topping,
            frosting: self.set_frosting,
        };

        //check for invalid configuration
        if cake.is_cheesecake && cake.frosting {
            Err("Your dentist is calling to tell you pls stop".to_string())
        } else {
            Ok(cake)
        }
    }
}
