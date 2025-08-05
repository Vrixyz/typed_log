use typed_log::{Loggable, log_any};

pub struct MyCustomStruct {
    pub to_log: String,
}

impl Loggable for MyCustomStruct {}

fn logger(my_struct: &MyCustomStruct) {
    println!("{}", my_struct.to_log);
}
fn main() {
    typed_log::push_log_impl(&logger);

    log_any(&MyCustomStruct {
        to_log: "inner value".to_string(),
    });
}
