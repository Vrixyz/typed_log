fn main() {
    typed_log::push_log_impl(|garden: &library::Garden| {
        println!("{:?}", garden);
    });
    typed_log::push_log_impl(|garden: &library::Garden| {
        for _ in 0..garden.flower_amount {
            print!("🌸");
        }
        println!();
        println!("Thanks for visiting my garden.")
    });
    typed_log::push_log_any(|garden: &dyn typed_log::Loggable| {
        println!("the address of this garden is: {:p}", &garden);
    });

    library::make_a_garden();
}

mod library {
    use typed_log::{Loggable, log_any};

    #[derive(Debug)]
    pub struct Garden {
        pub flower_amount: usize,
    }

    impl Loggable for Garden {}

    pub fn make_a_garden() {
        let garden = Garden { flower_amount: 10 };
        log_any(&garden);
    }
}
