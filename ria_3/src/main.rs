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
    array_ex();
    veccy()
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

// Enabling context lines to be printed out with a <Vec<Vec<T>>>

fn veccy() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
        Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
 What do we seek
 through millions of pages?";
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{} : {} ", line_num, line);
        }
    }
}
