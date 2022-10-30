use super::task4;
use console::Term;
use std::collections::HashMap;
use round::round;
use std::io;

pub fn task5(){
    
    //initiate machine
    let coins= initiate_machine();
    //check machine coins or proceed to product selection
    check_or_proceed(coins);
}

pub fn initiate_machine()->HashMap<&'static str,i32>{
    return task4::initiate_machine();
}

pub fn check_or_proceed(mut coins:HashMap<&str,i32>) ->HashMap<&str, i32>{
    let price;
    let paid;
    let key = Term::buffered_stdout();
    println!("press 'e' to check machine coins or 'c' to continue to proceed to product list");
        if let Ok('e') = key.read_char() {
            println!("{:?}",coins);
            println!("press 'a' to add coins or 'r' to remove coins");
            if let Ok('a') = key.read_char() {
                coins=fill_machine(coins);
            }else if let Ok('r') = key.read_char() {
                coins=remove_coins(coins);
            }
        }else if let Ok('c') = key.read_char(){
            price=task4::choose_product();
            paid=task4::pay_product(price);
            let change=round(paid -price,2);
            coins=task4::insert_coins(paid,change,coins);
            task4::calculate_change(price, paid);
        }
    return coins;
}

pub fn fill_machine(mut coins:HashMap<&str,i32>) ->HashMap<&str, i32>{
 
    'add: loop {
        println!("type the coin to add to your machine or 's' to stop ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input = user_input.trim();
        if user_input == "2.00"{
            if *coins.get("2.00").unwrap() != 50 {
                coins.insert("2.00", *coins.get("2.00").unwrap()+1);
            }else if *coins.get("2.00").unwrap() == 50{
                println!("can't insert coin of amount 2.00 machine reach the maximum amount");
            }
            
        }else if user_input=="1.00" {
            if *coins.get("1.00").unwrap() != 50 {
                coins.insert("1.00", *coins.get("1.00").unwrap()+1);
            }else if *coins.get("1.00").unwrap() == 50{
                println!("can't insert coin of amount 1.00 machine reach the maximum amount");
            }
            
        }else if user_input=="0.50" {
            if *coins.get("0.50").unwrap() != 50 {
                coins.insert("0.50", *coins.get("0.50").unwrap()+1);
            }else if *coins.get("0.50").unwrap() == 50{
                println!("can't insert coin of amount 0.50 machine reach the maximum amount");
            }
            
        }else if user_input=="0.20" {
            if *coins.get("0.20").unwrap() != 50 {
                coins.insert("0.20", *coins.get("0.20").unwrap()+1);
            }else if *coins.get("0.20").unwrap() == 50{
                println!("can't insert coin of amount 0.20 machine reach the maximum amount");
            }
            
        }else if user_input=="0.10" {
            if *coins.get("0.10").unwrap() != 50 {
                coins.insert("0.10", *coins.get("0.10").unwrap()+1);
            }else if *coins.get("0.10").unwrap() == 50{
                println!("can't insert coin of amount 0.10 machine reach the maximum amount");
            }
            
        }else if user_input=="0.05" {
            if *coins.get("0.05").unwrap() != 50 {
                coins.insert("0.05", *coins.get("0.05").unwrap()+1);
            }else if *coins.get("0.05").unwrap() == 50{
                println!("can't insert coin of amount 0.05 machine reach the maximum amount");
            }
            
        }else if user_input=="0.02" {
            if *coins.get("0.02").unwrap() != 50 {
                coins.insert("0.02", *coins.get("2.00").unwrap()+1);
            }else if *coins.get("0.02").unwrap() == 50{
                println!("can't insert coin of amount 0.02 machine reach the maximum amount");
            }
            
        }else if user_input=="0.01" {
            if *coins.get("0.01").unwrap() != 50 {
                coins.insert("0.01", *coins.get("0.01").unwrap()+1);
            }else if *coins.get("0.01").unwrap() == 50{
                println!("can't insert coin of amount 0.01 machine reach the maximum amount");
            }
            
        }else if user_input =="s"{
            break 'add ;
        }else{
            println!("insert coin in the format : 2.00 1.00 0.50 0.20 0.10 0.05 0.02 and 0.01")

        }
        
        println!("{:?}",coins); 
    }
    return coins;
}

pub fn remove_coins(mut coins:HashMap<&str,i32>) -> HashMap<&str,i32>{
 
'remove: loop {    
    println!("type the coin to remove from the machine or 's' to stop");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim();
    if user_input == "2.00"{
        if *coins.get("2.00").unwrap() != 0 {
            coins.insert("2.00", *coins.get("2.00").unwrap()-1);
        }else if *coins.get("2.00").unwrap() == 0{
            println!("can't remove coin of amount 2.00 machine reach the maximum amount");
        }
        
    }else if user_input=="1.00" {
        if *coins.get("1.00").unwrap() != 0 {
            coins.insert("1.00", *coins.get("1.00").unwrap()-1);
        }else if *coins.get("1.00").unwrap() == 0{
            println!("can't remove coin of amount 1.00 machine reach the maximum amount");
        }
        
    }else if user_input=="0.50" {
        if *coins.get("0.50").unwrap() != 0 {
            coins.insert("0.50", *coins.get("0.50").unwrap()-1);
        }else if *coins.get("0.50").unwrap() == 0{
            println!("can't remove coin of amount 0.50 machine reach the maximum amount");
        }
        
    }else if user_input=="0.20" {
        if *coins.get("0.20").unwrap() != 0 {
            coins.insert("0.20", *coins.get("0.20").unwrap()-1);
        }else if *coins.get("0.20").unwrap() == 0{
            println!("can't remove coin of amount 0.20 machine reach the maximum amount");
        }
        
    }else if user_input=="0.10" {
        if *coins.get("0.10").unwrap() != 0 {
            coins.insert("0.10", *coins.get("0.10").unwrap()-1);
        }else if *coins.get("0.10").unwrap() == 0{
            println!("can't remove coin of amount 0.10 machine reach the maximum amount");
        }
        
    }else if user_input=="0.05" {
        if *coins.get("0.05").unwrap() != 0 {
            coins.insert("0.05", *coins.get("0.05").unwrap()-1);
        }else if *coins.get("0.05").unwrap() == 0{
            println!("can't remove coin of amount 0.05 machine reach the maximum amount");
        }
        
    }else if user_input=="0.02" {
        if *coins.get("0.02").unwrap() != 0 {
            coins.insert("0.02", *coins.get("2.00").unwrap()-1);
        }else if *coins.get("0.02").unwrap() == 0{
            println!("can't remove coin of amount 0.02 machine reach the maximum amount");
        }
        
    }else if user_input=="0.01" {
        if *coins.get("0.01").unwrap() != 0 {
            coins.insert("0.01", *coins.get("0.01").unwrap()-1);
        }else if *coins.get("0.01").unwrap() == 0{
            println!("can't remove coin of amount 0.01 machine reach the maximum amount");
        }
        
    }else if user_input =="s"{
        break 'remove;
    }else{
        println!("insert coin in the format : 2.00 1.00 0.50 0.20 0.10 0.05 0.02 and 0.01")

    }
    println!("{:?}",coins);
}
return coins;
}