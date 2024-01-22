#![allow(dead_code)]

use core::cmp::Ordering; //Used for comparison of value sizes

pub enum Comp {
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct Person{
    name:String,
    age:u8
}

struct Unit;
//no variables or fields within but used for implementation in other bodies of code

struct Pair(i32,f32); // a tuple struct i.e used to hold multiple pairs

struct Point{
    x:f32,
    y:f32
}

struct Rectangle{         // here the struct Point is being used as a field in the struct Rectangle
    top_left: Point,
    bottom_right: Point
}


pub fn run() {
let person = Person{
    name: String::from("Ayomide"),
    age:19,
};

println!("{:#?}",person);

let _unit = Unit;

let point = Point{x:10.3,y:0.4};

println!("The point coordinates = ({},{})",point.x,point.y);

let bottom_right = Point{x:5.2,..point};

println!("Second point = ({},{})",bottom_right.x, bottom_right.y );

let Point {
     x:left_edge,
     y:top_edge 
    } = point;
     dbg!(&left_edge,&top_edge);

     let _rectangle = Rectangle{
        top_left:Point {
             x: left_edge,
             y: top_edge,
             },
             bottom_right,
     };

     let pair = Pair(1, 0.1);
     println!("pair contains {:?} and {:?}", pair.0, pair.1);

     let Pair(integer, decimal ) = pair;
     println!("pair contains {:?} and {:?}", integer, decimal);



}
trait Shape{
    fn new(length:i32,width:i32, name:&'static str) -> Self;
    fn area(&self) ->i32;
    fn set_length(&mut self, length:i32);
    fn get_length(&self) -> i32;
    fn set_width(&mut self, width:i32);
    fn get_width(&self) -> i32;
    fn set_name(&mut self, name:&'static str);
    fn get_name(&self) -> &str;

}

#[derive(Default, Debug, Clone)]
struct Rect{
    length:i32,
    width:i32,
    name: &'static str,
}

impl Rect{
    fn default() -> Self {
        Rect { length: 1,
             width: 1, 
            name: "rectangle1" 
        }
    }
}

impl Shape for Rect {
    fn new(length:i32, width:i32, name:&'static str) -> Self{
        Rect { length, 
            width, 
            name
         }
    }

    fn area(&self) -> i32{
        self.length * self.width

    }

    fn set_length(&mut self, length:i32) {
        self.length = length;
    }

    fn get_length(&self) -> i32 {
        self.length
    }

    fn set_width(&mut self, width:i32) {
        self.width = width;
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

}

impl PartialEq for Rect {
    fn eq(&self, other:&Self) -> bool{
        self.area() == other.area()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
        // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }

}

impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse:: <i32>().unwrap(),
            None => 0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect{length, width, name: &name}
    }
}
 pub fn run2(){
    let rectangle1 = Rect::default();

    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1,3,"Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!("result1 = {:?}", result1);

    let result2 =rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);
 }