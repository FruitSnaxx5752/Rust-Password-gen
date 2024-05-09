use rand::{thread_rng, Rng};

fn main(){
    let alpha_lower: Vec<char> = 
        vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let alpha_upper: Vec<char> = 
        vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let numeric: Vec<char> =
        vec!['1','2','3','4','5','6','7','8','9','0'];
    let special: Vec<char> =
        vec!['!','@','#','$','%','^','&','*','(',')','-','=','_','+','<',',','>','.','/',':','~',';','|','`'];
    let mut pass_length:i8 = 15;
    let mut gen_password: Vec<char> = vec![];
    while pass_length > 0 {
        let random_selection: i8 = thread_rng().gen_range(1..5);
        if random_selection == 1 {
            let random_char_select = thread_rng().gen_range(0..alpha_lower.len().try_into().unwrap()); 
            let random_char: char = alpha_lower[random_char_select].clone();
            gen_password.push(random_char);
        }else if random_selection == 2{
            let random_char_select = thread_rng().gen_range(0..alpha_upper.len().try_into().unwrap());
            let random_char: char = alpha_upper[random_char_select].clone(); 
            gen_password.push(random_char);
        }else if random_selection == 3 {
            let random_char_select = thread_rng().gen_range(0..numeric.len().try_into().unwrap());
            let random_char: char = numeric[random_char_select].clone();
            gen_password.push(random_char);
        }else if random_selection == 4 {
            let random_char_select = thread_rng().gen_range(0..special.len().try_into().unwrap());
            let random_char: char = special[random_char_select].clone(); 
            gen_password.push(random_char);
        }
        pass_length = pass_length -1;
    }
    let password: String = gen_password.clone().into_iter().map(|i| i.to_string()).collect::<String>();
    println!("{}",password)
}