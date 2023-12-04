fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn owner1() {
        // 使用尽可能多的方法来通过编译
        // let x = String::from("hello, world");
        // 解1 let y = &x;
        // 解2 let y = x.as_str();
        // 解3 let y = x.clone();
        let x = &String::from("hello, world");
        let y = x;
        println!("{},{}", x, y);
    }

    // 不要修改下面的代码
    #[test]
    fn owner2() {
        let s1 = String::from("hello, world");
        let s2 = take_ownership(s1);

        println!("{}", s2);
    }

    // 只能修改下面的代码!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        s
    }

    #[test]
    fn owner3() {
        let s = give_ownership();
        println!("{}", s);
    }

    // 只能修改下面的代码!
    fn give_ownership() -> String {
        let s = String::from("hello, world");
        // convert String to Vec
        // 将 String 转换成 Vec 类型
        let _s = s.clone().into_bytes();
        s
    }

    // 修复错误，不要删除任何代码行
    #[test]
    fn owner4() {
        let s = String::from("hello, world");

        print_str(s.clone());

        println!("{}", s);
    }

    fn print_str(s: String) {
        println!("{}", s)
    }

    // 不要使用 clone，使用 copy 的方式替代
    #[test]
    fn owner5() {
        // let x = (1, 2, (), "hello".to_string());
        // let y = x.clone();
        let x = (1, 2, (), "hello");
        let y = x;
        println!("{:?}, {:?}", x, y);
    }

    #[test]
    fn mut1() {
        let s = String::from("hello, ");

        // 只修改下面这行代码 !
        let mut s1 = s;

        s1.push_str("world")
    }

    #[test]
    fn mut2() {
        let x = Box::new(5);

        let mut y = Box::new(0);      // 完成该行代码，不要修改其它行！

        *y = 4;

        assert_eq!(*x, 5);
    }

    // 示例
    #[test]
    fn move1() {
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

    #[test]
    fn move2() {
        let t = (String::from("hello"), String::from("world"));

        let _s = t.0;

        // 仅修改下面这行代码，且不要使用 `_s`
        println!("{:?}", t.1);
    }

    #[test]
    fn move3() {
        let t = (String::from("hello"), String::from("world"));

        // 填空，不要修改其它代码
        let (ref s1, ref s2) = t;

        println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    }

}
