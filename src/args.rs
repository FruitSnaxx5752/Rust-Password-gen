use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args{
    ///Length of new password
    #[arg(short= 'l',long = "length")]
    length: i8,

}
pub fn args()-> i8{
    let args: Args = Args::parse();
    let len: i8 = args.length;
    return len;
}