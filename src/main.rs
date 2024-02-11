use std::env;
use ttl::config::Config;
use ttl::take_to_op;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = match Config::to_the_linux(&args) {
        Ok(conf) => conf,
        Err(err) => {
            panic!("AAAAAAAA {}", err);
        }
    };

    // println!("Args: {}, {:#?}", conf.command, conf.opts);
    take_to_op(&conf);
}
