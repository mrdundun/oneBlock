mod ex3;
mod ex4;
use crate::ex4::TrafficLight;

fn main() {
    // ex3
    println!("\n\nex3:----------------------------------");
    let mut list = vec![1,5,22,3,11,9];
    ex3::bubble(&mut list);
    println!("{:?}",list);

    let mut list = vec!['c','C','D','a','d','A'];
    ex3::bubble(&mut list);
    println!("{:?}",list);

    // ex4
    println!("\n\nex4:----------------------------------");
    println!("ex4:红绿灯");
    let red = ex4::Light::Red("20s");
    let yellow = ex4::Light::Yellow("4s");
    let green = ex4::Light::Green("20s");
    println!("red: {}",red.get_time());
    println!("yellow: {}",yellow.get_time());
    println!("green: {}",green.get_time());
    
    println!("\nex4:整数集合求和");
    let list:[u32;4] = [28,39,1,8];
    println!("sum: {:?}\n",ex4::sum(&list));
    let list_overflow:[u32;4] = [u32::MAX,39,1,8];
    println!("sum: {:?}",ex4::sum(&list_overflow));

    println!("\nex4:求面积");
    // let c = ex4::Circle::new(3.0,std::f64::consts::PI);
    // let t = ex4::Triangle::new(5.0,8.0,0.5);
    // let s = ex4::Square::new(5.0);
    // ex4::show_area(&c);
    // ex4::show_area(&t);
    // ex4::show_area(&s);

    let c = ex4::Circle::new(3.0);
    let t = ex4::Triangle::new(5.0,8.0);
    let s = ex4::Square::new(5.0);
    ex4::show_area(&c);
    ex4::show_area(&t);
    ex4::show_area(&s);
}