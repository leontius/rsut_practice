fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use std::ops::{Range, RangeInclusive};

    #[test]
    fn arr() {
        fn create_arr(n: i32) {
            // 数组的类型是 [T; Length]，就如你所看到的，数组的长度是类型签名的一部分，因此数组的长度必须在编译期就已知，例如你不能使用以下方式来声明一个数组:
            // let arr = [1; n];
        }
    }


    #[test]
    fn arr1() {
        // 使用合适的类型填空
        let arr: [i32; 5] = [1, 2, 3, 4, 5];

        // 修改以下代码，让它顺利运行
        assert!(arr.len() == 5);
    }


    #[test]
    fn arr2() {
        // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
        let arr0 = [1, 2, 3];
        let arr: [_; 3] = ['a', 'b', 'c'];

        // 填空
        // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
        // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
        assert!(std::mem::size_of_val(&arr) == 12);
    }


    // 数组中的所有元素可以一起初始化为同一个值
    #[test]
    fn arr3() {
        // 填空
        let list: [i32; 100] = [1; 100];

        assert!(list[0] == 1);
        assert!(list.len() == 100);
    }


    #[test]
    fn arr4() {
        // 修复错误
        let _arr = [1, 2, 3];
        println!("{:?}", _arr);
    }

    #[test]
    fn arr5() {
        let arr = ['a', 'b', 'c'];

        let ele = arr[0]; // 只修改此行来让代码工作

        assert!(ele == 'a');
    }


    // 修复代码中的错误
    #[test]
    fn arr6() {
        let names = [String::from("Sunfei"), "Sunface".to_string()];

        // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
        let name0 = names.get(0).unwrap();

        // 但是下标索引就存在越界的风险了
        let _name1 = &names[0];
    }


    // 修复代码中的错误，不要新增代码行!
    #[test]
    fn slice1() {
        // 这里, [i32] 和 str 都是切片类型，但是直接使用它们会造成编译错误，如下代码所示。为了解决，你需要使用切片的引用： &[i32]，&str。
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..1];

        let s2: &str = "hello, world";
    }

    /// 一个切片引用占用了2个字大小的内存空间( 从现在开始，为了简洁性考虑，如无特殊原因，我们统一使用切片来特指切片引用 )。
    /// 该切片的第一个字是指向数据的指针，第二个字是切片的长度。字的大小取决于处理器架构，例如在 x86-64 上，字的大小是 64 位也就是 8 个字节，那么一个切片引用就是 16 个字节大小。
    /// 切片( 引用 )可以用来借用数组的某个连续的部分，对应的签名是 &[T]，大家可以与数组的签名对比下 [T; Length]。
    #[test]
    fn slice2() {
        let arr: [char; 3] = ['中', '国', '人'];

        let slice = &arr[..1];

        // 修改数字 `8` 让代码工作
        // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
        let x = std::mem::size_of_val(&slice);
        assert!(x == 16);
    }


    #[test]
    fn slice3() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        // 填空让代码工作起来
        let slice = &arr[1..=3];
        assert_eq!(slice, &[2, 3, 4]);
    }


    #[test]
    fn slice4() {
        let s = String::from("hello");

        let slice1 = &s[0..2];
        // 填空，不要再使用 0..2
        let slice2 = &s[..2];

        assert_eq!(slice1, slice2);
    }

    #[test]
    fn slice5() {
        let s = "你好，世界";
        // 修改以下代码行，让代码工作起来
        let slice = &s[0..3];

        assert!(slice == "你");
    }


    // 修复所有错误
    #[test]
    fn slice6() {
        let mut s = String::from("hello world");

        // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
        // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
        let ch = first_character(&s);

        println!("the first character is: {}", ch);
        s.clear(); // error!
    }

    fn first_character(s: &str) -> &str {
        &s[..1]
    }

    #[test]
    fn tuple1() {
        let _t0: (u8, i16) = (0, -1);
        // 元组的成员还可以是一个元组
        let _t1: (u8, (i16, u32)) = (0, (-1, 1));
        // 填空让代码工作
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    }


    // 修改合适的地方，让代码工作
    #[test]
    fn tuple2() {
        let t = ("i", "am", "sunface");
        assert_eq!(t.2, "sunface");
    }


    // 修复代码错误
    #[test]
    fn tuple3() {
        // 超过12个元素的元组无法被打印。
        let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
        println!("too long tuple: {:?}", too_long_tuple);
    }


    #[test]
    fn tuple4() {
        let tup = (1, 6.4, "hello");

        // 填空
        let (x, z, y) = tup;

        assert_eq!(x, 1);
        assert_eq!(y, "hello");
        assert_eq!(z, 6.4);
    }

    #[test]
    fn tuple5() {
        let (x, y, z);

        // 填空
        (y, z, x) = (1, 2, 3);

        assert_eq!(x, 3);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
    }


    #[test]
    fn tuple6() {
        // 填空，需要稍微计算下
        let (x, y) = sum_multiply((2, 3));

        assert_eq!(x, 5);
        assert_eq!(y, 6);
    }

    fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
        (nums.0 + nums.1, nums.0 * nums.1)
    }


    // fix the error
    struct Person {
        name: String,
        age: u8,
        hobby: String,
    }

    #[test]
    fn struct1() {
        let age = 30;
        let p = Person {
            name: String::from("sunface"),
            age,
            hobby: String::from("hoppy"),
        };
    }


    struct Unit;

    trait SomeTrait {
        // ...定义一些行为
    }

    // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
    // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
    impl SomeTrait for Unit {}

    #[test]
    fn struct2() {
        let u = Unit;
        do_something_with_unit(u);
    }

    // 填空，让代码工作
    fn do_something_with_unit(u: Unit) {}


    // 填空并修复错误
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32);

    // 元组结构体看起来跟元组很像，但是它拥有一个结构体的名称，该名称可以赋予它一定的意义。由于它并不关心内部数据到底是什么名称，因此此时元组结构体就非常适合。
    #[test]
    fn struct3() {
        let v = Color(0, 127, 255);
        check_color(v);
    }

    fn check_color(p: Color) {
        let Color(x, y, z) = p;
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(z, 255);
    }


    // 填空并修复错误，不要增加或移除代码行
    struct Person1 {
        name: String,
        age: u8,
    }

    // 你可以在实例化一个结构体时将它整体标记为可变的，但是 Rust 不允许我们将结构体的某个字段专门指定为可变的.
    #[test]
    fn struct4() {
        let age = 18;
        let mut p = Person1 {
            name: String::from("sunface"),
            age,
        };

        // how can you believe sunface is only 18?
        p.age = 30;

        // 填空
        p.name = String::from("sunfei");
    }


    // 填空
    #[test]
    fn struct5() {
        build_person("caocao".to_string(), 18);
    }

    fn build_person(name: String, age: u8) -> Person1 {
        Person1 {
            age,
            name,
        }
    }


    // 填空，让代码工作
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 你可以使用结构体更新语法基于一个结构体实例来构造另一个
    #[test]
    fn struct6() {
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);
    }

    fn set_email(u: User) -> User {
        User {
            active: u.active,
            username: u.username,
            email: String::from("contact@im.dev"),
            sign_in_count: u.sign_in_count,
        }
    }


    // 填空，让代码工作
    #[derive(Debug, Clone)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    #[test]
    fn struct7() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
            height: 50,
        };

        dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

        println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
    }

    #[test]
    fn struct_example() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
        // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
        let Person { name, ref age } = person;

        println!("The person's age is {}", age);

        println!("The person's name is {}", name);

        // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
        //println!("The person struct is {:?}", person);

        // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
        println!("The person's age from person struct is {}", person.age);
    }


    // 修复错误
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }

    #[test]
    fn struct8() {
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string(),
        };

        let _name = f.name;

        // 只能修改这一行
        println!("{}, {}", _name, f.data);
    }


    // 修复错误
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Number1 {
        Zero = 0,
        One,
        Two,
    }

    // C语言风格的枚举定义
    enum Number2 {
        Zero = 0.0 as isize,
        One = 1.0 as isize,
        Two = 2.0 as isize,
    }

    #[test]
    fn enum1() {
        // 通过 `as` 可以将枚举值强转为整数类型
        assert_eq!(Number::One as i32, Number1::One as i32);
        assert_eq!(Number1::One as i32, Number2::One as i32);
    }


    // 填空
    #[derive(Debug, Clone)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn enum2() {
        let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
        let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
    }


    // 仅填空并修复错误
    #[test]
    fn enum3() {
        // let msg = Message::Move { x: 1, y: 2 };
        let msg = Message::Move { x: 1, y: 1 };

        if let Message::Move { x: a, y: b } = msg {
            assert_eq!(a, b);
        } else {
            panic!("不要让这行代码运行！");
        }
    }


    // 填空，并修复错误
    #[test]
    fn enum4() {
        let msgs: [Message; 3] = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];

        for msg in msgs {
            show_message(msg)
        }
    }

    fn show_message(msg: Message) {
        println!("{:?}", msg);
    }


    // 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
    #[test]
    fn enum5() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        if let n = six {
            println!("{:#?}", n)
        }

        panic!("不要让这行代码运行！");
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1)
        }
    }


    // 填空，让代码运行
    use crate::tests::List::*;

    enum List {
        // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
        Cons(u32, Box<List>),
        // Nil: 链表中的最后一个节点，用于说明链表的结束
        Nil,
    }

    // 为枚举实现一些方法
    impl List {
        // 创建空的链表
        fn new() -> List {
            // 因为没有节点，所以直接返回 Nil 节点
            // 枚举成员 Nil 的类型是 List
            Nil
        }

        // 在老的链表前面新增一个节点，并返回新的链表
        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        // 返回链表的长度
        fn len(&self) -> u32 {
            match *self {
                // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
                Cons(head, ref tail) => 1 + tail.len(),
                // 空链表的长度为 0
                Nil => 0
            }
        }

        // 返回链表的字符串表现形式，用于打印输出
        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    // 递归生成字符串
                    format!("{}, {}", head, tail.stringify())
                }
                Nil => {
                    format!("Nil")
                }
            }
        }
    }


    #[test]
    fn enum6() {
        // 创建一个新的链表(也是空的)
        let mut list = List::new();

        // 添加一些元素
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        // 打印列表的当前状态
        println!("链表的长度是: {}", list.len());
        println!("{}", list.stringify());
    }
}