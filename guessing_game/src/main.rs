// preclude 预导入内容 （rust自动导入）

// 不在预导入的crates（库），手动导入
use rand::Rng; //
use std::cmp::Ordering;
use std::io;

// 可执行程序入口函数
fn main() {
    // 调用println宏（）
    println!("Guess the number!"); //带！表示调用的是宏

    // let 变量名:数据类型，定义一个不可变的变量。
    let secret_number = rand::thread_rng().gen_range(1..=100); // 生成1到100随机数 也可以这么写gen_range(1,101),左开右闭

    // loop 循环
    loop {
        println!("Please input your guess.");

        // let mut 变量名:数据类型，该变量是可变的；let 变量名:数据类型，该变量是不可变的。
        let mut guess = String::new(); // new() 关联函数，类似于其他编程语言的静态方法

        io::stdin()
            // 从标准输入流中读取数据，把数据读入guess变量
            .read_line(&mut guess)
            // 异常处理
            .expect("Failed to read line");

        // match arm 分支选择，trim（）去掉左右两边的空白字符（空格，"\n"等）并转成字符串，parse（）转成u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // 正常，原样返回num
            Err(_) => continue, // '_'忽略错误信息，continue跳出当前循环，继续执行下一个循环。
        };

        println!("You guessed: {guess}");

        // cmp（）比较方法，返回Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 终止循环
            }
        };
        println!("Hello world!");
    }
}
