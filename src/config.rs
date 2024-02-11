pub struct Config {
    pub command: String,
    pub opts: Vec<String>
}

impl Config {
    pub fn to_the_linux(args: &[String]) -> Result<Self, &str> {
        let mut command = String::from("");
        let mut opts = Vec::<String>::new();

        if args.len() <= 1 {
            return Err("Args not enough");
        }

        if args.len() >= 1 {
            command = args[1].clone();
        }
        if args.len() >= 2 {
            for i in 2..args.len() {
                opts.push(args[i].clone());    
            }
        }

        println!("{:?}", opts); 

        return Ok(Config {command, opts});
    }
}