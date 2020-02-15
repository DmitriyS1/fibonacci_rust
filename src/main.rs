use std::io;

fn main() {
    println!("Fibonacci generator");
    loop {
        println!("Enter count of numbers that you want");
        let mut count = String::new();
        io::stdin().read_line(&mut count)
            .expect("Faild to read count");
    
        let count: u32 = match count.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("You must to type a number");
                continue;
            },
        };

        fibonacci_maker(count);
        break;
    }
}

fn fibonacci_maker(mut count: u32){
    let mut prev_num: u32 = 0;
    let mut result: u32 = 1;

    while count > 0{
        print!("{} ", prev_num);
        let tmp = result;
        result += prev_num;
        prev_num = tmp;
        count -= 1;
    }    
}