use super::task2;
use std::collections::HashMap;
use std::io;

pub fn task3(){

   
    let mut price :f64=0.0;
    let mut paid=0.0;
    
    //choose product
    price=choose_product();
    
    //pay product
    paid=pay_product(price);

    //return coins to the user
    return_coins(price, paid);

    

}

pub fn choose_product()->f64{
    let mut price:f64=0.0;
    let mut checked_product_name = false;
    let products = HashMap::from([    
        ("Snickers", 1.4),
        ("Clif", 1.7),
        ("Pop-Tarts", 1.0),
        ("Granola Bars", 1.5),
        ("Mars", 1.5),
        ("Pretzels", 2.5),
        ("Chex Mix", 0.5),
        ("lanters Trail Mix", 3.5),
        ("Reese’s Peanut Butter Cups", 4.5),
        ("Sun Chips", 1.8)
    ]);

    //display the products and get the product name from the user
    println!("Choose a product : \n {:#?}",products);
    println!("Product name : ");

    //check the product name
    while !checked_product_name {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();
        //check product name written by the user exists in the vending machine
        if check_product_exists(user_input,&products) {
            checked_product_name = true;
            //get the product price
            price = get_product_price(user_input, &products);
            
        }else{
            println!("Please write a valid product name :")
        }
    }
    return price;
} 

fn check_product_exists(user_input:&str,products:&HashMap<&str,f64>)->bool{
    return products.contains_key(user_input);
}

fn get_product_price(user_input:&str,products:&HashMap<&str,f64>) ->f64{
    return *products.get(user_input).unwrap();
}
pub fn pay_product(price:f64)->f64{
    //run task2
    let paid=task2::pay_product(price);
    return paid;
}
pub fn return_coins(price:f64,paid:f64){
    task2::return_coins(price, paid);
}