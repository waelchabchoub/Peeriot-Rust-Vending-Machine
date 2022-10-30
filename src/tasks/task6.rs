use super::task5;
use std::collections::HashMap;
use std::io::{self};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn task6(){
    //path to the file
    let path = "./src/tasks/data/machine.json";

    //read the file
    let mut coins:HashMap<&str,i32> = HashMap::new();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let json: serde_json::Value = serde_json::from_str(&contents).expect("JSON does not have correct format.");
    for (key, value) in json.as_object().unwrap() {
        coins.insert(key, value.as_i64().unwrap() as i32);
    }
    println!("file loaded successfully : {:?}",coins);

    //proceed task 5
    coins=task5::check_or_proceed(coins);

    //save the final state of the machine
    let result = write_to_file(coins,path);

}

fn write_to_file(coins:HashMap<&str,i32>,path:&str) -> io::Result<()>{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &coins)?;
    writer.flush()?;
    println!("File saved successfully!");
    Ok(())
}
