fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    struct Point {
        x: f64,
        y: f64,
    }

    // `Point` 的关联函数都放在下面的 `impl` 语句块中
    impl Point {
        // 关联函数的使用方法跟构造器非常类似
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 另外一个关联函数，有两个参数
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // 这是一个方法
        // `&self` 是 `self: &Self` 的语法糖
        // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // 使用点操作符可以访问 `self` 中的结构体字段
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;


            // `abs` 是一个 `f64` 类型的方法，会返回调用者的绝对值
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` 持有两个分配在堆上的整数
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 该方法会拿走调用者的所有权
        // `self` 是 `self: Self` 的语法糖
        fn destroy(self) {
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // `first` 和 `second` 在这里超出作用域并被释放
        }
    }

    #[test]
    fn example() {
        let rectangle = Rectangle {
            // 关联函数的调用不是通过点操作符，而是使用 `::`
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };

        // 方法才是通过点操作符调用
        // 注意，这里的方法需要的是 `&self` 但是我们并没有使用 `(&rectangle).perimeter()` 来调用，原因在于：
        // 编译器会帮我们自动取引用
        //  `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());

        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };


        // 错误！`rectangle` 是不可变的，但是这个方法要求一个可变的对象
        // rectangle.translate(1.0, 0.0);
        // TODO ^ 试着反注释此行，看看会发生什么

        // 可以！可变对象可以调用可变的方法
        square.translate(1.0, 1.0);

        let pair = Pair(Box::new(1), Box::new(2));

        pair.destroy();

        // Error! 上一个 `destroy` 调用拿走了 `pair` 的所有权
        //pair.destroy();
        // TODO ^ 试着反注释此行
    }

    struct Rectangle1 {
        width: u32,
        height: u32,
    }

    impl Rectangle1 {
        // 完成 area 方法，返回矩形 Rectangle 的面积
        fn area(&self) -> u32 {
            self.height * self.width
        }
    }

    /// 方法跟函数类似：都是使用 fn 声明，有参数和返回值。但是与函数不同的是，方法定义在结构体的上下文中(枚举、特征对象也可以定义方法)，而且方法的第一个参数一定是 self 或其变体 &self 、&mut self，self 代表了当前调用的结构体实例。
    #[test]
    fn func1() {
        let rect1 = Rectangle1 { width: 30, height: 50 };

        assert_eq!(rect1.area(), 1500);
    }

    // 只填空，不要删除任何代码行!
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }

    /// self 会拿走当前结构体实例(调用对象)的所有权，而 &self 却只会借用一个不可变引用，&mut self 会借用一个可变引用
    #[test]
    fn func2() {
        let light = TrafficLight {
            color: "red".to_owned(),
        };
        // 不要拿走 `light` 的所有权
        light.show_state();
        // 否则下面代码会报错
        println!("{:?}", light);
    }

    struct TrafficLight1 {
        color: String,
    }

    impl TrafficLight1 {
        // 使用 `Self` 填空
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }

        // 填空，不要使用 `Self` 或其变体
        pub fn change_state(self: &mut Self) {
            self.color = "green".to_string()
        }
    }

    #[test]
    fn func3() {}

    #[derive(Debug)]
    struct TrafficLight2 {
        color: String,
    }

    impl TrafficLight2 {
        // 1. 实现下面的关联函数 `new`,
        // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
        // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
        pub fn new() -> Self {
            TrafficLight2 {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

    #[test]
    fn func4() {
        let light = TrafficLight2::new();
        assert_eq!(light.get_state(), "red");
    }


    struct Rectangle2 {
        width: u32,
        height: u32,
    }

    // 使用多个 `impl` 语句块重写下面的代码
    impl Rectangle2 {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle2 {
        fn can_hold(&self, other: &Rectangle2) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn func5() {}


    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    // 为 TrafficLightColor 实现所需的方法
    impl TrafficLightColor {
        fn color(&self) -> String {
            match self {
                TrafficLightColor::Red => "red".to_string(),
                TrafficLightColor::Yellow => "yellow".to_string(),
                TrafficLightColor::Green => "green".to_string(),
            }
        }
    }

    #[test]
    fn func6() {
        let c = TrafficLightColor::Yellow;

        assert_eq!(c.color(), "yellow");

        println!("{:?}", c);
    }
}