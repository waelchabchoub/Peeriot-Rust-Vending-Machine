use crate::tasks::task1::{generate_random_price, calculate_change};
use super::task1;
use round::round;

pub fn task2(){

    let price:f64;
    let paid:f64;
    //generate random price
    price = generate_random_price();
    
    //pay product
    paid = pay_product(price);
    //return coins
    return_coins(price,paid);

}

pub fn pay_product(price:f64)->f64{
    //run task 1
    let paid=task1::pay_product(price);
    return paid;
}

pub fn return_coins(price:f64,paid:f64){
    //calculate change
    let mut change = calculate_change(price, paid);
    let mut coins = vec![];
    while change > 0.00 {
        
        if change>=2.00 {
            change-=2.00;
            coins.push(2.00);
        }else if change>=1.00 {
            change-=1.00;
            coins.push(1.00);
        }else if change>=0.50 {
            change-=0.50;
            coins.push(0.50);
        }else if change>=0.20 {
            change-=0.20;
            coins.push(0.20);
        }else if change>=0.10 {
            change-=0.10;
            coins.push(0.10);
        }else if change>=0.05 {
            change-= 0.05;
            coins.push(0.05);
        }else if change>=0.02 {
            change -=0.02;
            coins.push(0.02);
        }else if change>=0.01 {
            change -=0.01;
            coins.push(0.01);
        }
        change =round(change,2);
        }
    println!("the machine is returning change : {:?}",coins);
}