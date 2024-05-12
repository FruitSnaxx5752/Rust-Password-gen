use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about)]
pub struct Length{
    ///Length of new password (Max length: 255)
    #[arg(short= 'l',long = "length", default_value_t = 15)]
    length: u8,

}
pub fn length()-> u8{
    let length: Length = Length::parse();
    let mut len: u8 = length.length;
    if len == 0{
        len = 15;
    }
    return len;
}