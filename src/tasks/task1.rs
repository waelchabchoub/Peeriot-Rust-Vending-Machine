use rand::Rng;
use round::round;
use std::io;
use regex::Regex;

pub fn task1(){
    
    let paid;
    //generate a random price
    let price=generate_random_price();
    //pay for the product
    paid=pay_product(price);
    //calculate change
    calculate_change(price,paid);
    
}

pub fn pay_product(price:f64)-> f64{
    
    println!("Please you need to pay a sum of : {}€ for your product",price);
    println!("Enter the amount to pay in this format '000.00':");
    let mut input_format = false;
    let mut paid_covered = false;
    let mut paid=0.0;
    while !input_format || !paid_covered {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();

        //check format of the user input
        (input_format,paid_covered,paid) = check_format_and_amount(user_input, price);
        
        if !input_format{
            println!("Error format must be '000.00' ! :");
        }else if !paid_covered {
            println!("Error please enter enough amount to pay you product :")
        }   
    }
    return paid;
}

pub fn generate_random_price()->f64{
    return round(rand::thread_rng().gen_range(1.0..=10.0),2);
}

pub fn check_format_and_amount(user_input:&str,price:f64) -> (bool,bool,f64){
    let mut input_format = false;
    let mut paid_covered = false;
    let mut paid = 0.0;
    //check if only digits exist in the input and '.'
    let mut check_numeric = true;
    for c in user_input.chars() {
        if !c.is_numeric() && c != '.' {
            check_numeric = false;
        }
    }
    // check if the format = '000.00'
    if check_numeric {
        let re = Regex::new(r"[0-9]").unwrap();
        let result = re.replace_all(&user_input, "0");
        if result == "000.00" {
            input_format = true;
            paid = user_input.parse::<f64>().unwrap();
                
            if paid>=price {
                paid_covered = true;
            }
        }
            
    }
    return (input_format,paid_covered,paid);

}
pub fn calculate_change(price:f64,paid:f64)->f64{
    let change = round(paid - price,2);
    println!("Your change is : {}€",change);
    return change;
}
