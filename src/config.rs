pub enum Command {
    Help,
    Cat,
    Wc,
    Head,
    Tail,
}

impl Command {
    fn from_str(cmd_str: &str) -> Result<Self, String> {
        match cmd_str {
            "help" => Ok(Command::Help),
            "cat" => Ok(Command::Cat),
            "wc" => Ok(Command::Wc),
            "head" => Ok(Command::Head),
            "tail" => Ok(Command::Tail),
            _ => Err(format!("Command {} not found", cmd_str))
        }
    }
}

pub struct Config {
    pub command: Command,
    pub opts: Vec<String>
}

impl Config {
    pub fn to_the_linux(args: &[String]) -> Result<Self, &str> {

        if args.len() <= 1 {
            return Err("Args not enough");
        }

        let command = match Command::from_str(&args[1]) {
            Ok(command) => command,
            Err(err_str) => {
                println!("{}", err_str);
                std::process::exit(1);
            }
        };
        
        let mut opts = Vec::<String>::new();
        if args.len() >= 2 {
            for i in 2..args.len() {
                opts.push(args[i].clone());    
            }
        }

        // println!("{:?}", opts); 

        return Ok(Config {command, opts});
    }
}
