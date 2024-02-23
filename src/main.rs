use std::env;
use ttl::config::parse_args;
use ttl::exec_ops;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let commands = match parse_args(&args) {
        Ok(cmds) => cmds,
        Err(err) => {
            panic!("AAAAAAAA: {}", err);
        }
    };

    exec_ops(&commands);
}
