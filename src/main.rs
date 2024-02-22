use std::env;
use ttl::config::parse_commands;
use ttl::do_the_ops;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let commands = match parse_commands(&args) {
        Ok(cmds) => cmds,
        Err(err) => {
            panic!("AAAAAAAA: {}", err);
        }
    };

    do_the_ops(&commands);
}
