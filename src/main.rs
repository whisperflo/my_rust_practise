// use std::result;
// use std::io;

// use std::arch::x86_64;

fn _greet_world() {
    let southern_germany = "Grüß Gott Welt";
    let chinese = "世界你好";
    let english = "World,hello";
    let regions = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", region);
    }
}

fn _add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式->Rust 函数默认返回最后一个表达式的值（不带分号）
}

fn _plus_five(x: i32) -> i32 {
    x + 5
}

fn _plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

struct _Struct {
    a: i32,
    b: i32,
    e: i32,
}
fn _change(some_string: &mut String) {
    some_string.push_str(", world!")
}
fn _greet(name: &str) {
    println!("Hello, {}!", name);
}
fn _say_hello(s: &str) {
    println!("{}", s);
}
#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// fn _value_in_cents(coin:Coin) ->u8{
//     match coin{
//         Coin::Penny => 1,
//         Coin::Nickel =>5,
//         Coin::Dime =>10,
//         Coin::Quarter(state) =>{
//             println!("Quarter from state: {:?}",state);
//             25
//         },
//     }
// }
#[allow(dead_code)]
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}
//  #[derive(Clone,Debug)]
#[allow(dead_code)]
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageAt {
    Hello { id: i32 },
}

// use num::complex::Complex; 有理数和复数
fn main() {
    // match 模式匹配中的变量遮蔽
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched,y = {}", y),
        _ => println!("Default case,x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // 单分支多模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过..=匹配值的范围
    // Rust的范围模式使用 ..= 语法，表示一个闭区间，包含起始值和结束值
    // 序列只允许数字或字符类型，原因：它们可连续匹配，而其他类型（如布尔值、枚举等）则不适用范围模式
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构结构体
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({},{})", x, y),
    }

    // 解构枚举
    // let msg = Message::ChangeColor(0, 160, 255);
    let m1 = Message::Quit;
    match m1 {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message:{}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 解构数组(不定长数组)
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
        println!("The first element of the array is: {}", x);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
        println!("The last element of the array is: {}", y);
    }

    let arr: &[u16] = &[];
    assert!(matches!(arr, [..])); // matches!宏支持数组模式
    assert!(!matches!(arr, [x, ..])); // matches!宏支持空数组模式

    // 匹配守卫
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定
    let msg = MessageAt::Hello { id: 5 };
    match msg {
        MessageAt::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range:{}", id_variable);
        }
        MessageAt::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageAt::Hello { id } => {
            println!("Found some other id :{}", id)
        }
    }

    // @前绑定后解构(Rust 1.56新增)
    // 使用@还可以在绑定新变量的同时，对目标进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x:{},y:{}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is :{} in {:?}", y, p);
    } else {
        println!("x was not 10");
    }

    // @新特性(Rust 1.53新增)
    
    // // matches!宏
    // let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];

    // // 现在要对v中的元素进行过滤，只保留类型是MyEnum::Foo的元素
    // let filtered: Vec<_> = v.iter().filter(|x: &&MyEnum| matches!(x, MyEnum::Foo)).collect();
    // dbg!(filtered);

    // v.retain(|x| matches!(x,MyEnum::Bar));
    // 下述两行代码报错 因为MyEnum 结构体未实现clone方法 加上#[derive(Clone)]即可
    // let mut new_vec = v.clone();
    // new_vec.retain(|x| matches!(x,MyEnum::Foo));
    // dbg!(new_vec);

    // let foo = 'f';
    // assert!(matches!(foo, 'a'..='z' | 'A'..='Z')); // matches!宏支持范围模式
    // println!("{}", matches!(foo, 'a'..='z' | 'A'..='Z'));

    // // enumerate用法
    // let v = vec!['a','b','c'];
    // for(index,value) in v.iter().enumerate(){
    //     println!("index:{},value:{}",index,value);
    // }

    //
    // dbg!(v);
    // // 模式绑定
    // let actions = [
    //     Action::Say("Hello Rust".to_string()),
    //     Action::MoveTo(1, 2),
    //     Action::ChangeColorRGB(255, 255, 0),
    // ];

    // for action in &actions{
    //     match action{
    //         Action::Say(s) => println!("Say:{}",s),
    //         Action::MoveTo(x, y) => println!("MoveTo:({}, {})", x, y),
    //         Action::ChangeColorRGB(r, g, _) => {
    //             println!("Change Color into '(r:{}, g:{}, b:0)', 'b has been ignored'", r, g);
    //         }
    //     }
    // }

    // let a = [1,2,3,4,5];
    // std::array::from_fn每次都会重新分配一块新的堆内存
    // 根据一个函数，生成一个数组
    // std::array::from_fn(|index| {
    // 根据 index 生成每个元素
    // })
    // let array :[String;8] = std::array::from_fn(|_i| String::from("rust is good"));
    // println!("{:#?}",array);
    // let a :[i32;5] = [1,2,3,4,5];
    // // 从索引 1 开始
    // // 到索引 3 结束（但不包含 3）
    // let slice :&[i32] = &a[1..3]; // 切片(Rust的切片区间是 左闭右开：[start..end))
    // assert_eq!(slice,&[2,3]);
    // println!("{:#?}",slice);

    // // 编译器自动推导one的类型
    // let one  = [1,2,3];

    // // 显示类型标注
    // let two :[u8;3] = [1,2,3];
    // let blank1 = [0;3];
    // let blank2:[u8;3] = [0;3];

    // // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8;3];
    // let arrays:[[u8;3];4] = [one,two,blank1,blank2];

    // // 借用arrays的元素用作循环中
    // for a in &arrays{
    //     print!("{:?}",a);
    //     // 将a变成一个迭代器，用作循环
    //     // 也可以用for n in a {} 来进行循环
    //     for n in a.iter(){
    //         print!("\t{} + 10 = {}",n,n+10);
    //     }
    //     let mut sum = 0;
    //     // 0..a.len是一个Rust的语法糖，其实就等于一个数组，元素从0，1，2一直增加到a.len-1
    //     for i in 0..a.len(){
    //         sum += a[i];
    //     }
    //     println!("\t({:?} = {})",a,sum);
    // }
    // let b  = [3;5];
    // println!("{:#?}",b);

    // println!("Please enter an array inex.");
    // let mut index = String::new();

    // // 读取控制台的输入
    // io::stdin().read_line(&mut index).expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Index entered was not a number");

    // let element_a = a[index];
    // let element_b = b[index];

    // println!("The value of the element at index {} is {} ",index,element_a);
    // println!("The value of the element at index {} is {} ",index,element_b);

    // let string_replace = String::from("I like rust.Learning rust is my favourite!");
    // let new_string_replacen = string_replace.replacen("rust","RUST",1);
    // let new_string_replace = string_replace.replace("rust", "Rust");
    // // dbg!会移动非Copy类型的值
    // // dbg：打印一个值到标准错误（stderr），并返回这个值(会显示值，所在文件和行号)
    // // 返回新的字符串
    // dbg!(&new_string_replace);
    // dbg!(&new_string_replacen);

    // // 直接操作原来的字符串
    // let mut string_replace_range = String::from("I lile rust!");
    // string_replace_range.replace_range(7..8, "R");
    // dbg!(string_replace_range);
    // // println!("{}",new_string_replace);
    // println!("-----------------------");
    // // pop --删除并返回字符串的最后一个字符
    // let mut  string_pop = String::from("rust pop 中文!");
    // let p1 = string_pop.pop();
    // let p2 = string_pop.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(string_pop);
    // // remove --删除并返回字符串中指定位置的字符
    // let mut string_remove = String::from("测试remove方法");
    // println!("string_remove 占 {} 个字节",std::mem::size_of_val(string_remove.as_str()));

    // // 删除第一个汉字
    // string_remove.remove(0);
    // dbg!(&string_remove);
    // // 删除第二个汉字
    // string_remove.remove(3);
    // dbg!(string_remove);

    // println!("-----------------------");
    // // truncate --删除字符串中从指定位置开始到结尾的全部字符
    // let mut string_truncate = String::from("测试truncate");
    // string_truncate.truncate(3);
    // dbg!(string_truncate);
    // // let mut s = String::from("Hello Rust!");
    // // s.insert(5,',');
    // // println!("插入字符 insert() -> {}",s);
    // // s.insert_str(6, "I like");
    // // println!("插入字符串 insert_str() -> {}",s);

    // // +/+= 连接字符串
    // let sn1 = String::from("Hello");
    // let sn2 = String::from(" Rust!");
    // let sn3 = sn1 + &sn2; // s1 被移动到 s3，s1 不再有效
    // println!("连接字符串 + -> {}",sn3);
    // let mut sn4 = String::from("Hello");
    // let sn5 = String::from(" Rust!");
    // sn4 += &sn5; // s4 被修改，s5 不受影响
    // println!("连接字符串 += -> {}",sn4);
    // println!("sn5 -> {}",sn5);

    // println!("-----------------------");

    // // 使用format!宏连接字符串
    // let fn1 = String::from("Hello");
    // let fn2 = String::from("Rust");
    // let fn3 = format!("{} {}!", fn1, fn2); // format!宏不会移动任何参数(不存在move)，所有参数在 format! 调用后仍然有效
    // println!("连接字符串 format! -> {}", fn3);
    // println!("fn1 -> {}", fn1);
    // println!("fn2 -> {}", fn2);

    // println!("-----------------------");
    // // 元组
    // let tup = (500, 6.4, "hello");
    // let (x, y, z) = tup; // 解构元组
    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);
    // println!("The value of z is: {}", z);
    // println!("The value of tup.0 is: {}", tup.0);
    // println!("The value of tup.1 is: {}", tup.1);
    // println!("The value of tup.2 is: {}", tup.2);
    // // 如果元组中的元素类型相同，可以使用数组来存储它们
    // let tup = (1,2,3);
    // let arr = [tup.0, tup.1, tup.2];
    // for x in arr.iter() {
    //     println!("x = {}", x);
    // }

    // let s = String::from("hello");
    // say_hello(&s);
    // say_hello(s.as_str());

    // let mut s = String::from("World");
    // let mut s = String::from("hello,world!");
    // s.replace_range(0..5, "HELLO");
    // println!("{}", s);
    // let s1 = &s[0..5];
    // let s2: &str = &s[6..12];
    // println!("{}, {}", s1, s2);

    // let mut s = String::from("helllo");
    // {
    //     let r1 = &mut s;
    //     println!("{}", r1);
    // }
    // let r2 = &mut s;
    // change(r2);
    // println!("{}", r2);
    // println!("s:{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1; // s1 被移动到 s2，s1 不再有效
    // println!("{},world!", s1); // 这行代码会报错，因为

    // let forty_twos = [42.0,42f32,42.0_f32];
    // println!("{:.2}",forty_twos[0]);

    // 序列
    // for i in 'a'..'z' {
    //     println!("i = {}", i);
    // }

    // 有理数和复数
    // let a = Complex{ re:2.1,im: -1.2};
    // let b = Complex::new(11.1, 22.2);
    // let reslt = a + b;
    // println!("{} + {}i", reslt.re, reslt.im);

    // let x = 13.14_f32;
    // let x = (x * 10.0).round() / 10.0;
    // println!("{}", x);

    // let c = 'z';
    // let z = 'ℤ';
    // let g = '国';
    // let heart_eyed_cat = '😻';
    // println!("c = {}, z = {}, g = {}, heart_eyed_cat = {}", c, z, g, heart_eyed_cat);

    // let result = add_with_extra(1, 3);
    // println!("result = {}", result);

    // let abc:(f32,f32,f32) = (0.1,0.2,0.3);
    // let xyz:(f64,f64,f64) = (0.1,0.2,0.3);
    // println!("abc (f32)");
    // println!(" 0.1 + 0.2 :{:x}", (abc.0 + abc.1).to_bits());
    // println!("       0.3:{:x}", (abc.2).to_bits());
    // println!();

    // println!("xyz (f64)");
    // println!(" 0.1 + 0.2 :{:x}", (xyz.0 + xyz.1).to_bits());
    // println!("       0.3:{:x}", (xyz.2).to_bits());
    // println!();
    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // let x = (-42.0_f32).sqrt();
    // match x.is_nan() {
    //     true => println!("NaN"),
    //     false => println!("{}", x),
    // }

    // if x.is_nan(){
    //     println!("NaN");
    // }
    // else
    // {
    //     println!("{}", x);
    // }

    // 循环控制
    // for i in 1..= 5{
    //     println!("i = {}", i);
    // }
    // let a = [4, 3, 2, 1];
    // // `.iter()` 方法把 `a` 数组变成一个迭代器
    // for (i, v) in a.iter().enumerate() {
    //     println!("第{}个元素是{}", i + 1, v);
    // }
    // // 模式匹配 match
    // let dire = Direction::South;
    // match dire{
    //     Direction::East => println!("East"),
    //     Direction::North | Direction::South => println!("North or South"),
    //     _ => println!("West"),
    // };
}
