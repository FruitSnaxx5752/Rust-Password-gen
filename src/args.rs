use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about)]
pub struct Args{
    /// Length of new password (Max length: 255) 0 Will be Deault
    #[arg(short= 'l',long = "length", default_value_t = 15)]
    length: u8,
    // Option for Lowercase charecters
    #[arg(short= 'L', long = "Lowercase", default_value = "false")]
    lowercase: bool,
    // Option for Uppercase charecters
    #[arg(short = 'U', long = "Uppercase", default_value = "false")]
    uppercase: bool,
    // Option for Numbers charecters
    #[arg(short= 'N', long = "Numerical", default_value = "false")]
    numerical: bool,
    // Option for Special charecters
    #[arg(short= 'S', long = "Specail", default_value = "false")]
    special: bool,
}

pub fn length()-> u8{
    let args: Args = Args::parse();
    let len: u8 = args.length;
    return pass_len(len);
}

fn pass_len(mut len:u8) -> u8{
    if len == 0{
        len = 15;
        return len;
    }else{
        return len;
    }
}

pub fn charlist() -> (bool, bool, bool, bool){
    let args: Args = Args::parse();
    let mut lowercase: bool = args.lowercase;
    let mut uppercase: bool = args.uppercase;
    let mut numeric: bool = args.numerical;
    let mut special: bool = args.special;
    if lowercase == false && uppercase == false && numeric == false && special == false{
        lowercase = true;
        uppercase = true;
        numeric = true;
        special = true;
        return (lowercase, uppercase, numeric, special);
    }else {
        return (lowercase, uppercase, numeric, special);
    }
}