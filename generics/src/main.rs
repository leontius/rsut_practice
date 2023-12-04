fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]

    // 填空
    struct A;

    // 具体的类型 `A`.
    struct S(A);

    // 具体的类型 `S`.
    struct SGen<T>(T); // 泛型 `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    #[test]
    fn generic1() {
        // 使用非泛型函数
        reg_fn(S(A {}));          // 具体的类型
        gen_spec_t(SGen { 0: A });      // 隐式地指定类型参数 `A`.
        gen_spec_i32(SGen { 0: 1_i32 });    // 隐式地指定类型参数 `i32`.

        // 显式地指定类型参数 `char`
        generic::<char>(SGen('c'));

        // 隐式地指定类型参数 `char`.
        generic(SGen { 0: 'a' });
    }


    // 实现下面的泛型函数 sum
    fn sum<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
        b + a
    }

    #[test]
    fn generic2() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));
    }

    struct Point<T> {
        x: T,
        y: T,
    }

    // 实现一个结构体 Point 让代码工作
    #[test]
    fn generic3() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }


    // 修改以下结构体让代码工作
    struct Point1<T> {
        x: T,
        y: String,
    }

    #[test]
    fn generic4() {
        // 不要修改这行代码！
        let p = Point1 { x: 5, y: "hello".to_string() };
    }


    // 为 Val 增加泛型参数，不要修改 `main` 中的代码
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    #[test]
    fn generic5() {
        let x = Val { val: 3.0 };
        let y = Val { val: "hello".to_string() };
        println!("{}, {}", x.value(), y.value());
    }

    struct Point2<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point2<T, U> {
        // 实现 mixup，不要修改其它代码
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    #[test]
    fn generic6() {
        let p1 = Point2 { x: 5, y: 10 };
        let p2 = Point2 { x: "Hello", y: '中' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');
    }


    // 修复错误，让代码工作
    struct Point3<T> {
        x: T,
        y: T,
    }

    impl Point3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    #[test]
    fn generic7() {
        let p = Point3 { x: 5.0, y: 10.0 };
        println!("{}", p.distance_from_origin());
    }


    // 目前，const 泛型参数只能使用以下形式的实参:
    // 1.一个单独的 const 泛型参数
    // 2.一个字面量 (i.e. 整数, 布尔值或字符).
    // 3.一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
    fn foo<const N: usize>() {}

    fn bar<T, const M: usize>() {
        foo::<M>(); // ok: 符合第一种
        foo::<2021>(); // ok: 符合第二种
        foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // ok: 符合第三种

        // foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
        // foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T

        let _: [u8; M]; // ok: 符合第一种
        // let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
    }

    #[test]
    fn cons_example1() {}


    // 修复错误
    struct Array<T, const N: usize> {
        data: [T; N],
    }

    // <T, const N: usize> 是结构体类型的一部分，和数组类型一样，这意味着长度不同会导致类型不同： Array<i32, 3> 和 Array<i32, 4> 是不同的类型
    #[test]
    fn cons1() {
        let arrays = [
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 3],
            },
            Array {
                data: [1, 2, 3]
            }
        ];
    }


    // 填空
    fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }

    #[test]
    fn cons2() {
        let arr = [1, 2, 3];
        print_array(arr);

        let arr = ["hello", "world"];
        print_array(arr);
    }

    // fn check_size<T>(val: T)
    //     where
    //         Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    // {
    //     //...
    // }

    // 修复 main 函数中的错误
    // 有时我们希望能限制一个变量占用内存的大小，例如在嵌入式环境中，此时 const 泛型参数的第三种形式 const 表达式 就非常适合.
    #[test]
    fn cons3() {
        // check_size([0u8; 767]);
        // check_size([0i32; 191]);
        // check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
        // check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
        // check_size(['中'; 191]); // A char takes 4 bytes in Rust
    }

    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}


    struct Sheep {
        naked: bool,
        name: String,
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // 特征 Trait 可以告诉编译器一个特定的类型所具有的、且能跟其它类型共享的特性。
    // 我们可以使用特征通过抽象的方式来定义这种共享行为，还可以使用特征约束来限定一个泛型类型必须要具有某个特定的行为。
    trait Animal {
        // 关联函数签名；`Self` 指代实现者的类型
        // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
        fn new(name: String) -> Self;

        // 方法签名
        fn name(&self) -> String;

        fn noise(&self) -> String;

        // 方法还能提供默认的定义实现
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Animal for Sheep {
        // `Self` 被替换成具体的实现者类型： `Sheep`
        fn new(name: String) -> Sheep {
            Sheep { name: name, naked: false }
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn noise(&self) -> String {
            if self.is_naked() {
                "baaaaah?".to_string()
            } else {
                "baaaaah!".to_string()
            }
        }

        // 默认的特征方法可以被重写
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    #[test]
    fn traits_example() {
        // 这里的类型注释时必须的
        let mut dolly: Sheep = Animal::new("Dolly".to_string());
        // TODO ^ 尝试去除类型注释，看看会发生什么

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }


    // 完成两个 `impl` 语句块
    // 不要修改 `main` 中的代码
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}

    impl Hello for Student {
        fn say_hi(&self) -> String {
            "hi".to_string()
        }

        fn say_something(&self) -> String {
            "I'm a good student".to_string()
        }
    }

    struct Teacher {}

    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            "Hi, I'm your new teacher".to_string()
        }

        fn say_something(&self) -> String {
            "I'm not a bad teacher".to_string()
        }
    }

    #[test]
    fn traits1() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!")
    }


    // `Centimeters`, 一个元组结构体，可以被比较大小
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, 一个元组结构体可以被打印
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // 添加一些属性让代码工作
    // 不要修改其它代码！
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct Seconds(i32);

    /// 我们可以使用 #[derive] 属性来派生一些特征，对于这些特征编译器会自动进行默认实现，对于日常代码开发而言，这是非常方便的，例如大家经常用到的 Debug 特征，就是直接通过派生来获取默认实现，而无需我们手动去完成这个工作。
    #[test]
    fn traits2() {
        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = _one_second == _one_second;
        let _this_is_true = _one_second > _one_second;

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp =
            if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };

        println!("One foot is {} than one meter.", cmp);
    }


    // 实现 fn multiply 方法
    // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
    // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
    fn multiply<T: std::ops::Mul<Output=T>>(a: T, b: T) -> T {
        a * b
    }

    #[test]
    fn traits3() {
        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));

        println!("Success!")
    }


    // 修复错误，不要修改 `main` 中的代码!

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Foo;

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Bar;

    #[derive(Debug, PartialEq, PartialOrd)]
    struct FooBar;

    #[derive(Debug, PartialEq, PartialOrd)]
    struct BarFoo;

    // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
    impl std::ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl std::ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }

    #[test]
    fn traits4() {
        // 不要修改下面代码
        // 你需要为 FooBar 派生一些特征来让代码工作
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!")
    }


    // 实现 `fn summary`
    // 修复错误且不要移除任何代码行
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    // 除了使用具体类型来作为函数参数，我们还能通过 impl Trait 的方式来指定实现了该特征的参数：该参数能接受的类型必须要实现指定的特征。
    #[test]
    fn traits5() {
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(&post);
        summary(&weibo);

        println!("{:?}", post);
        println!("{:?}", weibo);
    }

    // 在下面实现 `fn summary` 函数
    fn summary(x: &impl Summary) {
        let _ = x.summarize();
    }


    struct Sheep1 {}

    struct Cow {}

    trait Animal1 {
        fn noise(&self) -> String;
    }

    impl Animal1 for Sheep1 {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal1 for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
    // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
    fn random_animal(random_number: f64) -> Box<dyn Animal1> {
        if random_number < 0.5 {
            Box::new(Sheep1 {})
        } else {
            Box::new(Cow {})
        }
    }

    // 我们还可以在函数的返回值中使用 impl Trait 语法。然后只有在返回值是同一个类型时，才能这么使用，如果返回值是不同的类型，你可能更需要特征对象。
    #[test]
    fn trait6() {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!("You've randomly chosen an animal, and it says {}", animal.noise());
    }

    #[test]
    fn trait7() {
        assert_eq!(_sum(1, 2), 3);
        assert_eq!(_sum2(1, 2), 3);
    }

    // 通过两种方法使用特征约束来实现 `fn sum`
    fn _sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
        x + y
    }

    // impl Trait 语法非常直观简洁，但它实际上是特征约束的语法糖。
    // 当使用泛型参数时，我们往往需要为该参数指定特定的行为，这种指定方式就是通过特征约束来实现的。
    fn _sum2<S>(x: S, y: S) -> S
        where S: std::ops::Add<Output=S> {
        x + y
    }

    // 修复代码中的错误
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    struct Unit(i32);

    #[test]
    fn trait8() {
        let pair = Pair{
            x: Unit(1),
            y: Unit(3)
        };

        pair.cmp_display();
    }


    // 填空
    fn example1() {
        // `T: Trait` 是最常使用的方式
        // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }

        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        println!("v = {}, arg = {}", v, arg);
                        self.value = Some(v);
                        v
                    },
                }
            }
        }

        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(10), 11);
        assert_eq!(cacher.value(15), 11);
    }


    fn example2() {
        // 还可以使用 `where` 来约束 T
        struct Cacher<T>
            where T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
            where T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    },
                }
            }
        }

        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 21);
    }



    #[test]
    fn trait9() {
        example1();
        example2();

        println!("Success!")
    }

}