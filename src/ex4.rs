pub trait TrafficLight<T> {
    fn get_time(&self) -> &T;
}

pub enum Light<T> {
    Red(T),
    Yellow(T),
    Green(T),
}

impl<T> TrafficLight<T> for Light<T> {
    fn get_time(&self) -> &T {
        match self {
            Light::Red(t) => t,
            Light::Yellow(t) => t,
            Light::Green(t) => t,
        }
    }
}

pub fn sum(list: &[u32]) -> Option<u32> {
    let is_panic = std::panic::catch_unwind(|| list.iter().sum::<u32>()).is_err();
    if is_panic {
        None
    } else {
        Some(list.iter().sum())
    }
}

// pub trait Area<T> {
//     fn area(&self) -> T;
// }

// pub fn show_area<T:std::fmt::Display>(s: &impl Area<T>) {
//     println!("area is: {}", s.area());
// }

// pub struct Circle<T> {
//     r: T,
//     pi: T,
// }

// impl<T> Circle<T> {
//     pub fn new(r: T, pi:T) -> Circle<T> {
//         Circle { r: r, pi: pi }
//     }
// }

// impl<T:Copy + std::ops::Mul<Output = T>> Area<T> for Circle<T> {
//     fn area(&self) -> T {
//         self.r * self.r * self.pi
//     }
// }

// pub struct Triangle<T> {
//     a: T,
//     h: T,
//     n: T,
// }

// impl<T> Triangle<T> {
//     pub fn new(a:T, h:T, n:T) -> Triangle<T> {
//         Triangle { a: a, h: h, n:n }
//     }
// }

// impl<T:Copy + std::ops::Mul<Output = T>> Area<T> for Triangle<T> {
//     fn area(&self) -> T {
//         self.a * self.h * self.n
//     }
// }

// pub struct Square<T> {
//     a: T,
// }

// impl<T> Square<T> {
//     pub fn new(a:T) ->Square<T> {
//         Square { a: a }
//     }
// }

// impl<T:Copy + std::ops::Mul<Output = T>> Area<T> for Square<T> {
//     fn area(&self) -> T {
//         self.a * self.a
//     }
// }

pub fn show_area(s: &impl Shape) {
    println!("{} area is: {}", s.name(),s.area());
}

pub trait Shape {
    fn name(&self) -> &str{
        let parts: Vec<&str> = std::any::type_name::<Self>().split("::").collect();
        parts[parts.len()-1]
    }

    fn area(&self) -> f64;
}

pub struct Circle{
    r:f64,
}

impl Circle{
    pub fn new(r:f64) -> Circle{
        Circle { r: r }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

pub struct Triangle{
    a:f64,
    h:f64,
}

impl Triangle{
    pub fn new(a:f64, h:f64) -> Triangle{
        Triangle { a: a, h: h }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.a * self.h
    }
}

pub struct Square{
    a:f64,
}

impl Square {
    pub fn new(a:f64) -> Square{
        Square { a: a }
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.a * self.a    
    }
}