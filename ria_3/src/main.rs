use std::ops::Add;
use std::time::Duration;
fn main() {
    let a = 10;
    let b = 20;

    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);

    let gad = add_with_generics(5, 2);
    println!("{}", gad);

    let durations = add_with_generics(Duration::new(5, 0), Duration::new(10, 0));

    println!("{:?} ", durations);

    let search_term = "picture";

    let quote = "\
        Every face, every shop, bedroom window, public-house, adn
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{} : {}", line_num, line);
        }
    }
    array_ex()
}

fn add_with_lifetimes<'a, 'b>(j: &'b i32, i: &'a i32) -> i32 {
    *i + *j
}

fn add_with_generics<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn array_ex() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays = [one, two, blank1, blank2];
    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        (0..a.len()).for_each(|i| {
            sum += a[i];
        });
        println!("\t({:?} = {})", a, sum);
    }
}
