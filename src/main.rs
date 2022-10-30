mod tasks;
use console::Term;
fn main() {
    let stdout = Term::buffered_stdout();
    
    'task: loop {
        
        //run task1
        //tasks::task1::task1();
        
        //run task2 
        //tasks::task2::task2();

        //run task3 
        //tasks::task3::task3();
      

        //run task 4
        //tasks::task4::task4();

        //run task 5
        //tasks::task5::task5();

        //run task 6
        tasks::task6::task6();
        
        //stop the program when clicking 'q'
        println!("press any key to continue or 'q' to stop the program");
        if let Ok('q') = stdout.read_char() {
            
            break 'task;
            
        }else {
            continue;
        }
    }
}

