use std::process::exit;
use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(name = "psw-gen")]
#[command(version = "1.0")]
#[command(about = "Simple password generator")]
struct Args {
    /// Password length
    #[arg()]
    length: u16,

    /// Special characters
    #[arg(short = 's', long = "no-special-chars", action = clap::ArgAction::SetFalse)]
    special_char: bool,

    /// Upper chars
    #[arg(short = 'u', long = "no-upper-chars", action = clap::ArgAction::SetFalse)]
    upper_char: bool,

    /// Numbers
    #[arg(short = 'n', long = "no-numbers", action = clap::ArgAction::SetFalse)]
    numbers: bool,

    /// Lower chars
    #[arg(short = 'l', long = "no-lower-chars", action = clap::ArgAction::SetFalse)]
    lower_char: bool
}

fn main() {
    let args = Args::parse();

    if !args.numbers && !args.upper_char && !args.lower_char && !args.special_char {
        eprintln!("Invalid option configuration");
        exit(1);
    }

    let generator = Generator {
        length: args.length,
        special_char: args.special_char,
        upper_char: args.upper_char,
        numbers: args.numbers,
        lower_char: args.lower_char,
    };

    let gen = generator.gen();
    print!("{}", gen);
}
struct Generator {
    length: u16,
    special_char: bool,
    upper_char: bool,
    numbers: bool,
    lower_char: bool,
}
impl Generator {
    pub fn gen(&self) -> String {
        let mut password: String = "".to_string();
        let mut rng = rand::thread_rng();
        for _ in 0..self.length {
            let typ = self.gen_type();

            match typ {
                2 => {
                    // Upper char
                    let ch: char = char::from(rng.gen_range(b'A'..=b'Z')).to_ascii_uppercase();
                    password.push(ch)
                }
                3 => {
                    // Special char
                    let index = rng.gen_range(33..48);
                    let ch = char::from(index);
                    password.push(ch)
                }
                4 => {
                    // number
                    let index = rng.gen_range(0..9);
                    password.push_str(&index.to_string());
                }
                i => {
                    if i > 4 {
                        // normal letter
                        let ch = char::from(rng.gen_range(b'a'..=b'z')).to_ascii_lowercase();
                        password.push(ch)
                    }
                }
            }
        }
        password
    }

   fn gen_type(&self) -> u16 {
       let mut rng = rand::thread_rng();
       let mut typ = rng.gen_range(3..10);

       while (!self.special_char && typ == 3) ||
           (!self.upper_char && typ == 2) ||
           (!self.numbers && typ == 4) ||
           (!self.lower_char && typ > 4) {
           typ = rng.gen_range(0..5);
       }

       typ
   }
}