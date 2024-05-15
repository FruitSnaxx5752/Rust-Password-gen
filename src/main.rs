use rand::{thread_rng, Rng};

mod args;

fn main(){

    let char_list:(bool, bool, bool, bool) = args::charlist();
    let alpha_lower: Vec<char> = 
        vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let alpha_upper: Vec<char> = 
        vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let numeric: Vec<char> =
        vec!['1','2','3','4','5','6','7','8','9','0'];
    let special: Vec<char> =
        vec!['!','@','#','$','%','^','&','*','(',')','-','=','_','+','<',',','>','.','/',':','~',';','|','`'];
    let mut pass_length:u8 = args::length();
    let mut gen_password: Vec<char> = vec![];
    while pass_length > 0 {
        let random_selection: u8 = thread_rng().gen_range(1..5);
        if random_selection == 1 && char_list.0 == true{
            let random_char_select = thread_rng().gen_range(0..alpha_lower.len().try_into().unwrap()); 
            let random_char: char = alpha_lower[random_char_select].clone();
            gen_password.push(random_char);
            pass_length = pass_length -1;
        }else if random_selection == 2 && char_list.1 == true{
            let random_char_select = thread_rng().gen_range(0..alpha_upper.len().try_into().unwrap());
            let random_char: char = alpha_upper[random_char_select].clone(); 
            gen_password.push(random_char);
            pass_length = pass_length -1;
        }else if random_selection == 3 && char_list.2 == true{
            let random_char_select = thread_rng().gen_range(0..numeric.len().try_into().unwrap());
            let random_char: char = numeric[random_char_select].clone();
            gen_password.push(random_char);
            pass_length = pass_length -1;
        }else if random_selection == 4 && char_list.3 == true{
            let random_char_select = thread_rng().gen_range(0..special.len().try_into().unwrap());
            let random_char: char = special[random_char_select].clone(); 
            gen_password.push(random_char);
            pass_length = pass_length -1;
        }
    }

    let password: String = gen_password.clone().into_iter().map(|i| i.to_string()).collect::<String>();
    println!("{}",password);

}