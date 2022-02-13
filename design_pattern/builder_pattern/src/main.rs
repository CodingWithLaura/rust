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
