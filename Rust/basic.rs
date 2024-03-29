// 程序从此开始
// 对于type-level的构造使用驼峰命名法,对于value-level的构造使用蛇形命名法.
fn main() {
    // 使用let声明变量,默认不可变
    let a = 0;
    // 可以使用类型推断,mut可变
    let mut b = 1;
    // 默认整形为i32,可以显示指定类型
    let c: i32 = 2;
    // 语句以;结尾,不可以省略
    // println!是宏调用,{}是占位符,可以使用多个占位符
    // 字符串使用双引号
    println!("a + b = {}", add(a, b));
}

fn basic_types() {
    // 基本类型
    // 数值类型
    // i8,i16,i32,i64,isize,u32,u64,usize,f32,f64,
    println!("{}", i8::max_value());
    println!("{}", u8::max_value());
    //debug 模式下溢出会panic,release模式下溢出会按照补码循环溢出的规则处理
    //避免浮点数上测试相等型，数学上未定义时需要小心
    assert!(0.1 + 0.2 == 0.3); //panic 不严谨
    (0.1_f64 + 0.2_f64).abs() < 0.001; //严谨
    // 只有同类型才能进行运算

    // 序列
    // 1..5  1,2,3,4  1..=5  1,2,3,4,5
            
    // 布尔类型
    // true false

    // 字符类型
    // unicode 字符 四个字节 ‘’ 表示

    // 单元类型
    // () 作为一个值来占位,但是不占用内存

    // 字符串类型
    // 字符串字面量和字符串切片
}

// 函数定义
fn add(a: i32, b: i32) -> i32 {
    // return 可以省略,表达式不加分号
    a + b
}

