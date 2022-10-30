use super::task3;
use std::collections::HashMap;
use round::round;
use std::io;

pub fn task4(){

    let coins;
    let price:f64;
    let paid:f64;
    
    //initite the machine
    coins=initiate_machine();
    //choose product
    price=choose_product();
    //insert amount to pay
    paid= pay_product(price);
    //pay the amount of the payement you mentioned
    let change=round(paid -price,2);
    insert_coins(paid,change,coins);
    //calculate the change
    calculate_change(price, paid);
    
    

}
pub fn initiate_machine()-> HashMap<&'static str,i32> {
    
    let coins = HashMap::from([   

        ("2.00", 20),
        ("1.00", 20),
        ("0.50", 20),
        ("0.20", 20),
        ("0.10", 20),
        ("0.05", 20),
        ("0.02", 20),
        ("0.01", 20)

    ]);
    return coins;
}
pub fn choose_product() -> f64{
    let price=task3::choose_product();
    return price;
}
pub fn pay_product(price:f64)->f64{
    let paid:f64= task3::pay_product(price);
    return paid;
}

pub fn insert_coins(paid:f64,change:f64,mut coins:HashMap<&str,i32>) ->HashMap<&str,i32>{
    let mut price_to_pay:f64=paid;
    while price_to_pay > 0.0 {
         //check the change is payable 
        if !check_change_is_payable(change,&coins){
            println!("not enought money to return change");
            break;
        }
        println!("type the coins to pay your product, you have : {}€ left to pay",price_to_pay);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();
        if user_input == "2.00"{
            price_to_pay-=2.0;
            if *coins.get("2.00").unwrap() != 50 {
                coins.insert("2.00", *coins.get("2.00").unwrap()+1);
            }else if *coins.get("2.00").unwrap() == 50{
                println!("can't insert coin of amount 2.00 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="1.00" {
            price_to_pay-=1.00;
            if *coins.get("1.00").unwrap() != 50 {
                coins.insert("1.00", *coins.get("1.00").unwrap()+1);
            }else if *coins.get("1.00").unwrap() == 50{
                println!("can't insert coin of amount 1.00 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.50" {
            price_to_pay-=0.50;
            if *coins.get("0.50").unwrap() != 50 {
                coins.insert("0.50", *coins.get("0.50").unwrap()+1);
            }else if *coins.get("0.50").unwrap() == 50{
                println!("can't insert coin of amount 0.50 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.20" {
            price_to_pay-=0.20;
            if *coins.get("0.20").unwrap() != 50 {
                coins.insert("0.20", *coins.get("0.20").unwrap()+1);
            }else if *coins.get("0.20").unwrap() == 50{
                println!("can't insert coin of amount 0.20 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.10" {
            price_to_pay-=0.10;
            if *coins.get("0.10").unwrap() != 50 {
                coins.insert("0.10", *coins.get("0.10").unwrap()+1);
            }else if *coins.get("0.10").unwrap() == 50{
                println!("can't insert coin of amount 0.10 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.05" {
            price_to_pay-=0.05;
            if *coins.get("0.05").unwrap() != 50 {
                coins.insert("0.05", *coins.get("0.05").unwrap()+1);
            }else if *coins.get("0.05").unwrap() == 50{
                println!("can't insert coin of amount 0.05 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.02" {
            price_to_pay-=0.02;
            if *coins.get("0.02").unwrap() != 50 {
                coins.insert("0.02", *coins.get("2.00").unwrap()+1);
            }else if *coins.get("0.02").unwrap() == 50{
                println!("can't insert coin of amount 0.02 machine reach the maximum amount");
                break;
            }
            
        }else if user_input=="0.01" {
            price_to_pay-=0.01;
            if *coins.get("0.01").unwrap() != 50 {
                coins.insert("0.01", *coins.get("0.01").unwrap()+1);
            }else if *coins.get("0.01").unwrap() == 50{
                println!("can't insert coin of amount 0.01 machine reach the maximum amount");
                break;
            }
            
        }else{
            println!("insert coin in the format : 2.00 1.00 0.50 0.20 0.10 0.05 0.02 and 0.01")

        }
        price_to_pay=round(price_to_pay,2);
    }
    println!("the state of the machine after the operation : {:?}",coins);
    return coins;
}

fn check_change_is_payable(change:f64,coins:&HashMap<&str,i32>)->bool{
    let mut amount_available:f64=0.0; //To calculate the anticipated amount of money in the machine 
    for (key, value) in coins.into_iter() {
        amount_available+= key.parse::< f64 >().unwrap() * *value as f64;
    }
    if amount_available > change {
        return true;
    }else {
        return false;
    }
}
pub fn calculate_change(price:f64,paid:f64)->f64{
    let change = round(paid - price,2);
    println!("Your change is : {}€",change);
    return change;
}
