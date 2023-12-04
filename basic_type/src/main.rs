fn main() {}

#[cfg(test)]
mod tests {
    // 移除某个部分让代码工作
    #[test]
    fn num1() {
        let x: i32 = 5;
        // let mut y:u32 = 5;
        let mut _y = 5;

        _y = x;

        let _z = 10; // 这里 z 的类型是?
    }

    // 填空
    #[test]
    fn num2() {
        // let v: u16 = 38_u8 as __;
        let _v: u16 = 38_u8 as u16;
    }

    // 修改 `assert_eq!` 让代码工作
    #[test]
    fn num3() {
        let x = 5;
        // assert_eq!("u32".to_string(), type_of(&x));
        assert_eq!("i32".to_string(), type_of(&x));
    }

    // 填空，让代码工作
    #[test]
    fn num4() {
        // assert_eq!(i8::MAX, __);
        // assert_eq!(u8::MAX, __);
        assert_eq!(i8::MAX, 127);
        assert_eq!(u8::MAX, 255);
    }


    // 解决代码中的错误和 `panic`
    #[test]
    fn num5() {
        let v1 = 251_u8 - 8;
        // let v2 = i8::checked_add(251, 8).unwrap();
        let v2 = v1.checked_add(8).unwrap();
        println!("{},{}", v1, v2);
    }


    // 修改 `assert!` 让代码工作
    #[test]
    fn num6() {
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        // assert!(v == 1597);
        assert_eq!(v, 1597);
    }


    // 将 ? 替换成你的答案
    #[test]
    fn float1() {
        // let x = 1_000.000_1; // ?
        let x = 1_000.000_1; // f64
        let y: f32 = 0.12; // f32
        let z = 0.01_f64; // f64
    }

    #[test]
    fn float2() {
        // 使用两种方法来让下面代码工作
        // assert!(0.1+0.2==0.3);
        // 第一种方法
        assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32);
        // 第二种方法
        assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);
    }

    #[test]
    // 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122
    fn range1() {
        let mut sum = 0;
        for i in -3..2 {
            sum += i
        }

        assert_eq!(sum, -5);

        for c in 97..=122 {
            println!("{}", c);
        }
    }

    // 填空
    use std::ops::{Range, RangeInclusive};

    #[test]
    fn range2() {
        assert_eq!((1..5), Range { start: 1, end: 5 });
        let a = RangeInclusive::new(1, 5);
        println!("{:?}", a);
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
    }


    // 填空，并解决错误
    #[test]
    fn num7() {
        // 整数加法
        assert_eq!(1u32 + 2, 3);

        // 整数减法
        assert_eq!(1i32 - 2, -1);
        assert!(1i8 - 2 == -1);

        assert!(3 * 50 == 150);

        assert!(9.6_f32 / 3.2 == 3.0); // error ! 修改它让代码工作

        assert!(24 % 5 == 4);

        // 逻辑与或非操作
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);

        // 位操作
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }

    // 修改2处 `assert_eq!` 让代码工作
    use std::mem::size_of_val;
    use std::{thread, time};

    #[test]
    fn char1() {
        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4);

        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);

        println!("Success!")
    }

    // 修改一行让代码正常打印
    #[test]
    fn char2() {
        let c1 = "中";
        print_char(c1);
    }

    fn print_char(c: &str) {
        println!("{}", c);
    }


    // 使成功打印
    #[test]
    fn bool1() {
        let _f: bool = false;

        let t = _f;
        if !t {
            println!("Success!")
        }
    }


    #[test]
    fn bool2() {
        let f = true;
        let t = !(!true || !true);
        assert_eq!(t, f);

        println!("Success!")
    }


    // 让代码工作，但不要修改 `implicitly_ret_unit` !
    #[test]
    fn unit1() {
        let _v: () = ();

        let v = (2, 3);
        assert_eq!(_v, implicitly_ret_unit());

        println!("Success!")
    }

    fn implicitly_ret_unit() {
        println!("I will return a ()")
    }

    // 不要使用下面的函数，它只用于演示！
    fn explicitly_ret_unit() -> () {
        println!("I will return a ()")
    }


    // 让代码工作：修改 `assert!` 中的 `4`
    #[test]
    fn unit2() {
        let unit: () = ();
        // 单元结构体(unit tuple)(())类型，它被保证为尺寸为 0，对齐量为 1
        assert!(std::mem::size_of_val(&unit) == 0);

        println!("Success!")
    }

    // 示例
    #[test]
    fn statements1() {
        let x = 5u32;

        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;

            // 下面表达式的值将被赋给 `y`
            x_cube + x_squared + x
        };

        let z = {
            // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
            2 * x;
        };

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }

    // 使用两种方法让代码工作起来
    #[test]
    fn statements2() {
        let v = {
            let mut x = 1;
            x += 2;
            x
        };

        assert_eq!(v, 3);
    }

    #[test]
    fn statements3() {
        let v = {
            let x = 3;
            x
        };

        assert!(v == 3);
    }


    #[test]
    fn statements4() {
        let s = sum(1, 2);
        assert_eq!(s, 3);
    }

    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn func1() {
        // 不要修改下面两行代码!
        let (x, y) = (1, 2);
        let s = sum2(x, y);

        assert_eq!(s, 3);
    }

    fn sum2(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn func2() {
        print();
    }

    // 使用另一个类型来替代 i32
    fn print() -> () {
        println!("hello,world");
    }

    // 用两种方法求解
    #[test]
    fn func3() {
        never_return();
    }

    fn never_return() -> ! {
        // 实现这个函数，不要修改函数签名!
        // 解1 panic!("I return nothing!")
        loop {
            println!("I return nothing!");
            thread::sleep(time::Duration::from_secs(1));
        }
    }


    // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }


    #[test]
    fn func4() {
        println!("Success!");
    }

    fn get_option(tp: u8) -> Option<i32> {
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        };

        // 这里与其返回一个 None，不如使用发散函数替代
        never_return_fn()
    }

    // 使用三种方法实现以下发散函数
    fn never_return_fn() -> ! {
        // panic!()
        // todo!()
        loop {
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    #[test]
    fn func5() {
        // 填空
        let b = true;

        let _v = match b {
            true => 1,
            // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
            false => {
                println!("Success!");
                panic!("we have no value for `false`, but we can panic")
            }
        };

        println!("Exercise Failed if printing out this line!");
    }

}

