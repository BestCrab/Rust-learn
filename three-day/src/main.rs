/*创建字符串*/
// fn main() {
//     // 字符串字面量形式,会被硬编码到程序中，应用场景：适用于编译时已知的文本
//     let _s = "hello";
//
//     // 动态字符串 String类型在堆上分配，因此我们可以在运行时动态地构建和修改字符串，相比字面量而言，它会影响性能
//     // ::表示： 调用String中的from静态方法
//     let mut s = String::from("hello");
//     s.push_str(",world");
//     println!("{}", s);
// }

/*所有权的转移*/
// 所有权规则
// 1、Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 2、一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 3、当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

/*基础类型所有权转移*/
// fn main(){
//     let x = 1;
//     // 将x的值拷贝并赋值给y变量，并不会发生所有权的转移
//     let _y = x;
//     println!("x value is {}", x);
// }

/*所有权转移的错误例子，此代码会报错*/
// fn main() {
//     // String类型创建的字符串存储在堆中，变量s1存储在栈中，它存储了堆指针、字符串长度、字符串容量
//     let s1 = String::from("hello");
//     // 将s1中的堆指针、字符串长度、字符串容量转移给s2  自此s1变量无效
//     let s2 = s1;
//     println!("s1 value is {}", s1);
// }

/*相似的例子： 字符串字面量，此代码不会报错*/
// fn main(){
//     // 字符串字面量创建的值存储在静态内存区域(二进制程序中)，不存储在堆中
//     let s1 = "hello,world";
//     // 因此将s1赋值给s2并不会发生所有权的转移
//     let _s2 = s1;
//     println!("s1 value is {}", s1);
// }

/*解决String类型创建的值发生所有权转移后再次访问报错的问题,*/
// fn main() {
//     let  s1 = String::from("hello");
//     // 使用clone从堆中复制s1的值，并赋值给s2， 会有性能问题
//     let s2 = s1.clone();
//     println!("s1 value is {}, s2 value is {}", s1, s2);
//     // 结果：true  因为这里比较的是s1和s2的实际值
//     println!("s1 == s2 -> {}", s1 == s2);
// }

/*函数的传值与返回*/

// 1、转移与复制
// fn takes_ownership(some_string: String) -> String {
//     some_string
// }
// fn makes_copy(some_integer: i32) -> i32{
//     some_integer
// }
//
// fn main() {
//     // s进入作用域
//     let s = String::from("Hello");
//     // s进入函数，发生所有权转移
//     takes_ownership(s);
//     // 这里s无法使用
//
//     // x进入作用域
//     let x = 5;
//     // x进入函数，由于i32类型，具有Copy Trait
//     makes_copy(x);
//     // 这里仍然可以使用x
// }

// 2、函数返回值的所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn give_ownership() -> String{
    // some_string进入作用域
    let some_string = String::from("hello");
    // 将some_string返回，发生所有权转移
    some_string
}
fn main() {
    // s1将接收give_ownership函数的返回值
    let s1 = give_ownership();

    // s2进入变量作用域
    let s2 = String::from("World");
    // s2传入函数, 发生所有权转移
    let s3 = takes_and_gives_back(s2);
    // s2无法使用
} // 自此，s1离开作用域并丢弃，s2由于所有权转让，已被移走，s3离开作用域并丢弃
