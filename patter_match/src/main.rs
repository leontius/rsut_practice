fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    #[test]
    fn match1() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => { // 在这里匹配 South 或 North
                println!("South or North");
            }
            _ => println!("West"),
        };
    }


    #[test]
    fn match2() {
        let boolean = true;

        // 使用 match 表达式填空，并满足以下条件
        //
        // boolean = true => binary = 1
        // boolean = false => binary = 0
        let binary = match boolean {
            true => {
                1
            }
            false => {
                0
            }
        };

        assert_eq!(binary, 1);
    }


    // 填空
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    #[test]
    fn match3() {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0)
        ];

        for msg in msgs {
            show_message(msg)
        }
    }

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => { // 这里匹配 Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants")
        }
    }


    #[test]
    fn match4() {
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

        // 使用 `matches` 填空
        for ab in alphabets {
            assert!(matches!(ab, alphabets));
        }
    }


    enum MyEnum {
        Foo,
        Bar,
    }

    #[test]
    fn match5() {
        let mut count = 0;

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in v {
            if matches!(e, MyEnum::Foo) { // 修复错误，只能修改本行代码
                count += 1;
            }
        }

        assert_eq!(count, 2);
    }


    #[test]
    fn match6() {
        let o = Some(7);

        // 移除整个 `match` 语句块，使用 `if let` 替代
        // match o {
        //     Some(i) => {
        //         println!("This is a really long string and `{:?}`", i);
        //     }
        //     _ => {}
        // };

        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
        }
    }

    enum Foo1 {
        Bar(i32)
    }

    #[test]
    fn match7() {
        let a = Foo1::Bar(1);

        if let Foo1::Bar(i) = a {
            println!("foobar 持有的值是: {}", i);
        }
    }


    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    #[test]
    fn match8() {
        let a = Foo::Qux(10);

        // 移除以下代码，使用 `match` 代替
        // if let Foo::Bar = a {
        //     println!("match foo::bar")
        // } else if let Foo::Baz = a {
        //     println!("match foo::baz")
        // } else {
        //     println!("match others")
        // }

        match a {
            Foo::Bar => {
                println!("match foo::bar")
            }
            Foo::Baz => {
                println!("match foo::baz")
            }
            _ => {}
        }
    }


    // 就地修复错误
    #[test]
    fn match9() {
        let age = Some(30);
        if let Some(a) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
            assert_eq!(Some(a), Some(30));
        } // 新的 `age` 变量在这里超出作用域

        match age {
            // `match` 也能实现变量遮蔽
            Some(age) => println!("age 是一个新的变量，它的值是 {}", age),
            _ => ()
        }
    }


    #[test]
    fn patter1() {
        match_number(3);
    }

    fn match_number(n: i32) {
        match n {
            // 匹配一个单独的值
            1 => println!("One!"),
            // 使用 `|` 填空，不要使用 `..` 或 `..=`
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            // 匹配一个闭区间的数值序列
            6..=10 => {
                println!("match 6 -> 10")
            }
            _ => {
                println!("match 11 -> +infinite")
            }
        }
    }


    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn patter02() {
        // 填空，让 p 匹配第二个分支
        let p = Point { x: 3, y: 10 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // 第二个分支
            Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }


    // 修复错误
    enum Message1 {
        Hello { id: i32 },
    }

    #[test]
    fn patter03() {
        let msg = Message1::Hello { id: 5 };

        match msg {
            Message1::Hello {
                id: id @ (3..=7),
            } => println!("id 值的范围在 [3, 7] 之间: {}", id),
            Message1::Hello { id: newid @ (10 | 11 | 12 ) } => {
                println!("id 值的范围在 [10, 12] 之间: {}", newid)
            }
            Message1::Hello { id } => println!("Found some other id: {}", id),
        }
    }


    // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
    // 填空让代码工作，必须使用 `split`
    #[test]
    fn patter04() {
        let num = Some(4);
        let split = 5;
        match num {
            Some(x) if x < 5 => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => (),
        }
    }


    // 填空，让代码工作
    #[test]
    fn patter05() {
        let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

        match numbers {
            (first, .., last) => {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
            }
            _ => {}
        }
    }


    // 修复错误，尽量少地修改代码
    // 不要移除任何代码行
    // 使用模式 &mut V 去匹配一个可变引用时，你需要格外小心，因为匹配出来的 V 是一个值，而不是可变引用
    #[test]
    fn patter06() {
        let mut v = String::from("hello,");
        let r = &mut v;

        match r {
            value => value.push_str(" world!")
        }
    }
}