extern crate core;

use std::env::var;
use std::f64::consts::PI;
use std::u128::MAX;
fn number_type() {
    let f = 2.0;
    let m: f32 = 3.3;
    let b: i64 = 77;
    let a = 10;
    // let _n = 32;
    // let _y: u32 = 12;
    println!("{}_{}_{}_{}", f, m, a, b)
}

/// # this is a new func
/// 111
/// # hello
/// ```
/// a = b + c
/// ```
fn tup() {
    let a: [i32; 3] = [1, 3, 12];
    let second = a[1];
    let mut x = [1, 2, 3, 4];
    x[0] = 4;
    print!("{}", second)
}

fn new_func(a: i32, b: i32) -> i32 {
    let c = a + b;
    println!("{0}", c);
    return c;
}

fn lop_demo() {
    let a = [1, 2, 3, 4, 5];
    let mut i = 0;
    loop {
        if a[i] > 3 {
            break;
        }
        println!("{}", a[i]);
        i += 1;
    }
}

fn demo1() -> String {
    let name = String::from("Rose");
    return name;
}


fn demo2() {
    // let mut s = String::from("hello");
    let sm = String::from("none");
    let s2 = &sm;
    let s1 = &sm;
    // s2.push_str(" world");
    println!("{}_{}", s1, s2);
}


fn reference() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str(" world");
    println!("{}", s2);
}

fn str_slice() {
    let s1 = String::from("hello_world");
    let s2 = &s1[..5];
    let s3 = &s1[6..];
    let s4 = &s1[..7];
    // s1.push_str("_game");
    let s5 = &s1[..];
    println!("s2: {}, s3: {}", s2, s4);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a1 = &a[..3];
    for i in a1.iter() {
        println!("{}", i);
    }
}

fn struct_demo() {
    // struct definition
    struct Student {
        name: String,
        gender: bool,
        grade: u32
    }

    let jack = Student {
        name: String::from("Jack"),
        gender: true,
        grade: 12
    };

    let rose = Student {
        name: String::from("Jack"),
        ..jack
    };

    println!("{}__{}", jack.name, rose.grade);
}

fn tuple_struct() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 10.0);

    println!("{}, {}", black.0, origin.1)
}


fn nested_struct() {
    #[derive(Debug)]
    struct BodyData {
        height: f64,
        weight: f64
    }

    #[derive(Debug)]
    struct SeatPosition(i32, i32);

    #[derive(Debug)]
    struct Stu {
        name: String,
        age: i32,
        body_data: BodyData,
        seat_pos: SeatPosition
    }

    let rose = Stu {
        name: String::from("Rose"),
        age: 18,
        body_data: BodyData{height: 175.30, weight: 55.2},
        seat_pos: SeatPosition(5, 2)
    };


    impl Stu {
        fn stu_basic_info(&self){
            println!("{}的年龄是{}, 身高{}cm, 体重{}kg", self.name, self.age, self.body_data.height, self.body_data.weight);
        }

        fn is_older_than(&self, stu: Stu) -> bool {
            if self.age > stu.age {
                true
            } else {
                false
            }
        }


        fn create(name: String, age: i32, height: f64, weight: f64, x_pos: i32, y_pos: i32) -> Stu {
            Stu {
                name,
                age,
                body_data: BodyData{height, weight},
                seat_pos: SeatPosition(x_pos, y_pos),
            }
        }
    }
    // println!("{}\n{}\n{:#?}\n{:#?}", rose.name, rose.age, rose.body_data, rose.seat_pos)
    // print!("{}", rose.stu_basic_info());
    // rose.stu_basic_info();
    // let jack = Stu {
    //     name: String::from("Jack"),
    //     age: 23,
    //     body_data: BodyData{height: 187.3, weight: 77.3},
    //     seat_pos: SeatPosition(4, 2)
    // };
    let jack = Stu::create(String::from("Jack"), 23, 188.3, 78.3, 4, 2);
    println!("{:#?}", jack);
    jack.stu_basic_info();
    println!("{}", jack.is_older_than(rose));
}

struct Dog {
    name: String,
    age: i8
}

fn my_dog() {
    let wang_cai = Dog {
        name: String::from("wangcai"),
        age: 3
    };

    let s = &(wang_cai.name);
    let s1 = wang_cai.name.clone();
    println!("s:{},  s1: {}", s, s1);
    println!("{}", wang_cai.name);
}

fn enum_demo() {
    #[derive(Debug)]
    enum Book {
        Papery(u32),
        Electronic(String)
    }

    #[derive(Debug)]
    enum NewBook {
        Papery {index: u32},
        Electronic {url: String}
    }

    fn my_book() {
        let book = Book::Papery(1005);
        println!("{:?}", book);
    }

    fn new_book() {
        let book = NewBook::Papery {index: 103};
        let ebook = NewBook::Electronic {url: String::from("baidu.com")};
        match ebook {
            NewBook::Papery { index } => {
                println!("Papery book {}", index);
            }
            NewBook::Electronic { url } => {
                println!("Electronic book {}", url);
            }
        }
        // println!("{:?}", book)
    }

    // my_book();
    new_book();

}

fn option_demo() {
    // Option 枚举，可以枚举值为空的对象
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }
    let opt = Option::Some("hello");
    let opts: Option<&str> = Option::None;
    match opts {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    let t = Some(64);
    let tt = None;
    match tt {
        Some(64) => println!("Yes"),
        _ => println!("No")
    }
}

fn if_let_demo() {
    fn demo() {
        println!("yes, zero")
    }
    let i = 1;
    match i {0 => println!("zero"), _ => println!("not zero") }
    match i {
        0 => {
            demo()
        }
        _ => {
            println!("not zero")
        }
    }

    enum Book {
        Papery(u32), Electronic(String)
    }
    let my_book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = my_book {  // 属性为 tuple 类型的，需要临时指定一个名称
        println!("papery book")
    } else {
        println!("electronic book")
    }

}

use std::io;
use std::io::Read;
use std::fs::File;
use std::fs::read_to_string;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_demo() {
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file")
                },
                _ => { println!("Cannot read the file") }
            }
        }
    }
}


fn variables() {
    let x = 5;
    assert_eq!(x, 3);
    // println!("{}", a);
    println!("{}", x);
    let x: isize = 10086;
    let a = 10.33;
    // let b: i32 = 10.33 * 100; // error

}

fn get_sum() -> i32 {
    let a = 5;
    let b = 3;
    return a + b;
}

fn ownership() {
    let s1 = "hello";
    let s2 = s1;
    println!("s1: {}, s2: {}", s1, s2);
    let mut s3 = String::from("hello");
    s3.push_str(s1);
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
}

fn dereference() {
    let s = String::from("world");
    let s2 = &s;
    println!("{}__{}", s, s2);
    assert_eq!(s, *s2);
}

fn ref_func() {
    // 解引用函数
    let s1 = String::from("hello world");

    fn str_length(s: &String) -> usize {
        s.len()
    }

    let l = str_length(&s1);

    println!("Length of \"{}\" is {}.", s1, l)
}

fn no_ref_func() {
    let s1 = String::from("hello world");

    fn str_length(s: String) -> (usize, String) {
        (s.len(), s)
    }

    let res = str_length(s1);
    let s1 = res.1;
    let len = res.0;
    println!("Length of \"{}\" is {}.", s1, len);
}

fn action_scope() {
    let mut s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    // print!("{}{}", s2, s3);
    // print!("{}{}", s2, s3);
    let s4 = &mut s1;
    // print!("{}{}{}", s2, s3, s4);
}

fn s_slice() {
    let mut s = String::from("hello world");
    fn first_char(word: &String) -> &str {
        &word[..1]
    }
    let s1 = first_char(&s);
    s.clear(); // 此处使用了可变借用
    // println!("{}", s1); // 此处使用了不可变借用，因此编译不通过
}

fn str_operation() {
    // push
    let mut s = String::from("hello");
    s.push_str(" rust");
    s.push('!');

    // insert
    s.insert_str(0, "hi ");
    s.insert(0, '@');

    // replace
    let s1 = s.replace("hello", "HELLO");
    let mut s2 = s1.replacen(" ", "_", 3);
    s2.replace_range(0..5, "Yes");

    let s3 = "hello";
    let s4 = s3.replace("h", "H");
    println!("{}", s4);

    // delete
    dbg!(s4);
}

fn tuple_demo() {
    let s1 = String::from("hello");

    fn str_len(s: String) -> (String, usize) {
        let ls = s.len();
        (s, ls)
    }
    // fn str_l(s: String) -> usize {
    //     let ls = s.len();
    //     ls
    // }
    let res = str_len(s1);
    let s2 = res.0;
    let len_s1 = res.1;
    println!("{}_{}", s2, len_s1);
}

fn struct_emp() {
    // struct example
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u32
    }

    let mut user1 = User {
        username: String::from("Rose"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1
    };

    user1.username = String::from("Jack");

    fn build_user(email: String, username: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1
        }
    }

    let mut user2 = build_user(String::from("another@example.com"),
                              String::from("Julia"));
    let user3 = User {
        username: String::from("Bob"),
        ..user2 // 所有权转移，user2 的 email 将不再可用, 其他属性仍可用
    };
    user2.sign_in_count = 3;

    // tuple struct
    struct WeightHeight(f64, f64);

    // unit-like struct
    // struct AlwaysEqual;
    // let subject = AlwaysEqual;
    // impl SomeTrait for AlwaysEqual {
    //     println!("hello");
    // }
    // let ae = AlwaysEqual;
    let user4 = dbg!(user3);
    print!("{:#?}", user4)
}

fn enum_exm() {
    enum DiceNumber {
        // 骰子数字
        One, Two, Three, Four, Five, Six
    }

    enum PokerSuit {
        // 扑克牌花色
        Clubs,  // 梅花
        Spades,  // 黑桃
        Diamonds,  // 方块
        Hearts  // 红桃
    }

    let club = PokerSuit::Clubs;
    println!("{}", club);

    // option 枚举
    let a = Some(5);
    let b = Some(String::from("hello"));
    let c: Option<i32> = None;
}

fn main() {
    enum_exm();
    // struct_emp();
    // tuple_demo();
    // str_operation();
    // s_slice();
    // action_scope();
    // no_ref_func();
    // ref_func();
    // dereference();
    // ownership()
    // variables();
    // read_demo();
    // panic!("error occured");
    // println!("{}", PI);
    // if_let_demo();
    // option_demo();  // option 枚举
    // enum_demo();  // enum 枚举, match 的使用
    // my_dog();
    // nested_struct();
    // tuple_struct();
    // struct_demo();
    // array_slice();
    // str_slice();
    // reference();
    // demo2();
    // let name = demo1();
    // println!("{}", name);
    // let a = 100;
    // println!("{}", a)
    // let b = a + 33;
    // println!("{}", b);
    // lop_demo();
    // let s1 = String::from("Rosemary");
    // println!("s1 : {}, s1 : {:p}", s1, &s1);
    // let s2 = s1;
    // print!("s2 : {}, s2 : {:p}", s2, &s2);
    // number_type();
    // tup();
    // let c = new_func(3, 7);
    // print!("new_func: c is {}", c);
    // let y: i32 = {
    //     let x = 1;
    //     x + 100
    // };
    // println!("{}", y)
    
}
