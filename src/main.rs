mod ex3;

fn main() {
    let mut list = vec![1,5,22,3,11,9];
    ex3::bubble(&mut list);
    println!("{:?}",list);

    let mut list = vec!['c','C','D','a','d','A'];
    ex3::bubble(&mut list);
    println!("{:?}",list);
}