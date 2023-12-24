/*
    ::语法：表示调用的方法是该类型的关联函数(静态成员)
    .语法： 表示调用的方法存在于实例上
    mut：  表示声明的变量可以改变
    println!(): 用来打印日志，println!("a = {a}")与println!("a = {}", a)相同, {}表示占位符
    loop:  被loop包裹的代码会无限循环类似while循环
    &:     用于获取变量的引用
    match:  用于匹配枚举值的分支条件
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // 1..=100 生成范围为1-100的数字，包含1和100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    // loop: 在代码块内的代码会进入无线循环
    loop {
        println!("Please input your guess.");
        // 创建可变变量
        let mut guess = String::new();
        // 读取命令行输入的字符串内容
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // 将输入的字符串转换成u32无符号整数类型 必须显示的指定parse函数需要转换的类型
        // 通过match表达式对parse函数返回的Result枚举类型 与分支进行匹配 如果转换成功，状态为Ok，如果转换失败，状态为Err
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        println!("You guessed: {guess}");
        // cmp： 比较两个值的大小 返回值为Ordering枚举值
        // 通过match表达式对cmp函数返回的枚举值进行分支匹配
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
