use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args{
    ///Length of new password (Max length: 255)
    #[arg(short= 'l',long = "length")]
    length: u8,

}
pub fn args()-> u8{
    let args: Args = Args::parse();
    let len: u8 = args.length;
    return len;
}