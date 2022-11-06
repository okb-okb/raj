fn main() {
    let (k, x) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace().map(|i| i.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    if k*500 >= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
