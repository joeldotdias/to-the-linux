#[derive(Debug)]
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

#[derive(Debug)]
pub struct Config {
    pub command: Command,
    pub opts: Vec<String>
}

impl Config {
    pub fn from_str(config_str: &str) -> Result<Self, String> {
        let parts = config_str.split(" ").collect::<Vec<&str>>();
        let command = match Command::from_str(parts[0]) {
            Ok(command) => command,
            Err(err) => {
                return Err(err);
            }
        };

        let opts = parts[1..].into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        return Ok(Config { command, opts });
    }
}

pub fn parse_commands(args: &[String]) -> Result<Vec<Config>, String> {
    
    if args.len() <= 1 {
        return Err(String::from("Args not enough"));
    }
    
    let raw_args = args[1..].join(" ");
    let config_strs = raw_args.split(" into ").collect::<Vec<&str>>();

    
    let configs: Vec<Config> = config_strs.into_iter()
        .map(|conf_str| Config::from_str(&conf_str).unwrap())
        .collect();


    println!("{:?}", configs);

    return Ok(configs);
}