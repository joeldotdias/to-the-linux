use std::str::FromStr;
use crate::error::CmdParseError;

#[derive(Debug)]
pub enum Command {
    Help,
    Cat,
    Wc,
    Head,
    Tail,
}

impl FromStr for Command {
    type Err = CmdParseError;
    
    fn from_str(cmd_str: &str) -> Result<Self, CmdParseError> {
        let cmd = match cmd_str {
            "help" => Command::Help,
            "cat" => Command::Cat,
            "wc" => Command::Wc,
            "head" => Command::Head,
            "tail" => Command::Tail,
            
            _ => {
                return Err(CmdParseError::from(cmd_str));
            }
        };

        return Ok(cmd);
    }
}

#[derive(Debug)]
pub struct Config {
    pub command: Command,
    pub opts: Vec<String>
}

impl FromStr for Config {
    type Err = CmdParseError;

    fn from_str(config_str: &str) -> Result<Self, CmdParseError> {
        let parts = config_str.split(" ").collect::<Vec<&str>>();
        let command = match Command::from_str(parts[0]) {
            Ok(command) => command,
            Err(err) => {
                return Err(err);
            }
        };

        let opts = parts[1..].into_iter()
            .map(|opt| opt.to_string())
            .collect::<Vec<String>>();

        return Ok(Config { command, opts });
    }
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Vec<Self>, String> {
        if args.len() <= 1 {
            return Err(String::from("Args not enough"));
        }

        let raw_args = args[1..].join(" ");
        let config_strs = raw_args.split(" into ");

        let configs = config_strs.map(|conf_str| {
            match Config::from_str(&conf_str) {
                Ok(conf) => conf,
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }).collect::<Vec<Config>>();

        // println!("{:?}", configs);

        return Ok(configs);
    }
}
