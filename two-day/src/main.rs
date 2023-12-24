/*数值类型越界*/
// fn main(){
//     let x:u8 = 255;
//     // let b = x.wrapping_add(20);
//     // let b = x.checked_add(20);
//     // let (b,c) = x.overflowing_add(20);
//     let b = x.saturating_add(20);
//     println!("b is :{:?}", b)
// }

/*数值类型定义*/
// fn main(){
//     // 类型由编译器自动推导
//     let twenty = 20;
//     // 手动进行类型标注
//     let twenty_one:i32 = 20;
//     // 通过类型后缀的方式进行类型标注
//     let twenty_two = 20i32;
//
//     // 相同类型才可以进行运算
//     let addition = twenty + twenty_one + twenty_two;
//     println!("{} + {} + {} = {}",twenty, twenty_one, twenty_two, addition);
//
//     // 定义较长的数值类型
//     let one_million: i64 = 1_000_000;
//     println!("{}",one_million.pow(2));
//
//     // 定义f32数组, 42.0会被推断为f32类型
//     let forty_twos = [
//       42.0,
//         42.0f32,
//         42.0_f32
//     ];
//     // 打印数组第一项，并控制保留两位小数
//     println!("{:.2}",forty_twos[0]);
// }

/*位运算*/
// fn main() {
//     let a = 2;
//     let b = 3;
//
//     // &与运算 相同为1 否则为0
//     println!("(a & b ) value is {}", a & b);
//     // |或运算 有1为1  否则为0
//     println!("(a | b) value is {}", a | b);
//     // ^异或元素 不同为1 否则为0
//     println!("(a ^ b) value is {}", a ^ b);
//     // !位非运算 将位数进行取反，1变成0 0变成1
//     // 注意：由于需要对所有位数取反，包括符号位，因此取反后的二进制需要进行补码
//     // 补码： 将所有位数取反后加1
//     // 例如： !2的结果是-3
//     // 运算过程:
//     // 原始值： 2的8位二进制表示是: 0000 0010
//     // 进行位非运算：1111 1101  符号为1因此最后的结果肯定也是负数
//     // 进行补码:    0000 0010
//     // 加1：       0000 0011
//     // 转换成十进制后的结果：   -3
//     println!("(!a) value is {}", !a);
//     // <<左移 所有位向左移动 右位补0  2 >> 3 等价于 2 * 2的3次方
//     println!("(a << b) value is {}", a << b);
//     // >>右移  所有位向右带符号移动  正数左边补0  负数左边补1
//     println!("(a >> b) value is {}", a >> b);
//
//     let mut a = a;
//     // 将2左移3位 并赋值给a
//     a <<= b;
//     println!("(a <<= b) value is {}", a)
// }

/*序列(Range)*/
// fn main() {
//     // 循环1 - 5，不包括5
//     for i in 1..5 {
//         println!("{}",i);
//     }
//     // 循环1-5,包括5
//     for i in 1..=5 {
//         println!("{}",i);
//     }
//     // 遍历字符
//     for i in 'a'..='z' {
//         println!("{}",i)
//     }
// }

/*有理数和复数*/
// use num::complex::Complex;
// fn main() {
//     let a = Complex { re: 2.1, im: -1.2};
//     let b = Complex::new(11.2,22.2);
//     let result = a + b;
//
//     println!("{} + {}i", result.re, result.im);
// }

/*表达式*/
// fn main() {
//     let y = {
//         let x = 1;
//          x + 1
//     };
//     println!("{}", y);
// }

/*if语句块表达式*/
// fn main() {
//     println!("{:?}",assert_eq!(ret_unit_type(), ()));
// }
//
// fn ret_unit_type() {
//     let x = 1;
//     // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
//     // 类似三元运算符，在Rust里我们可以这样写
//     let y = if x % 2 == 1 {
//         "odd"
//     } else {
//         "even"
//     };
//     println!("y value is {}", y);
//     // 或者写成一行
//     let z = if x % 2 == 1 { "odd" } else { "even" };
//     println!("z value is {}", z);
// }

/*函数参数*/
// fn main(){
//     another_function(1,2.2);
// }
// fn another_function(x: i32, y: f32)  {
//     println!("x value is {}",x);
//     println!("y value is {}",y);
// }

fn plus_five(x:i32) -> i32{
    if x > 5 {
        return x ;
    }
    return x + 1;
}
fn main(){
   let x =  plus_five(4);
    println!("x value is {}", x);
}
