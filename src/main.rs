#[allow(warnings)]

fn main() {
    // let condition = false;
    // let some_number = 5;
    // let string_number = "5";
    // let names = ["Eric", "Nancy", "Lily"];
    // let number = string_number.parse::<i32>().unwrap();

    // integer_types::integer_types::hello();

    // integer
    // let number: u8 = 255;

    // float point
    // let f1: f32 = 1.0;
    // let f2: f32 = 176.98;
    // let f3: f32 = -1e4;
    // let f4: f32 = 1.;
    // let f5 = 32f32; // 32.0

    // bool
    let is_ok = true;
    if is_ok {
        println!("Good!");
    } else {
        println!("Sorry~");
    }

    // char
    // let c1 = 'a';
    // let c2 = '\'';
    // let c3 = '🐮';
    // let c4 = '\n';

    // tuple
    // let point = ('A', 32, 34);
    // println!("Ponit {}: {}, {}", point.0, point.1, point.2);

    // let text = "Hello world!";
    // 在index为 5 的位置进行切分，head 是 0-4，tail 是5-结尾
    // let (head, tail) = text.split_at(5);
    // println!("{}, {}", head, tail);

    // let a = ("hello, rust",);

    // unit type
    fn greet() -> () {
        println!("Hello!");
    }
    greet();

    // reference
    // let x = 10;
    // let r = &x; // &i32

    // let name: String = String::from("Eric"); // String
    // let reference = &name; // &String

    // say_hello(&name); // equal to: say_hello(reference)
    // say_hello(reference);
    // println!("{} is cool!", name);

    // arrays
    // let names = ["Alice", "Eric", "Nancy"];
    // for name in &names {
    //     println!("{}", name);
    // }

    // let numbers = [3, 1, 2, 4];
    // let zeros = [0; 1024];
    // let zeros2 = [0u8; 1024];

    // vectors 动态数组
    // let mut numbers = vec![1, 4, 3, 5];
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(3);

    for number in &numbers {
        println!("{}", number);
    }
}

// fn say_hello(name: &String) {
//     println!("hello, {}", name);
// }
