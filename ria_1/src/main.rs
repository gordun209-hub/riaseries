fn main() {
    greet_world();
    // VCS parser example
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data
        ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().is_empty() {
            // skip header row and lines with only whitespace
            continue;
        }
        // Vec<_> tels rust to infer types
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        // debug stuff
        if cfg!(debug_assertions) {
            eprintln!("debug : {:?} -> {:?}", record, fields);
        }
        // first filed is name
        let name = fields[0];
        // parse second field to get length
        // parse() method returns Ok(T) or Err(E) if error,
        // skip Errors and parse only succesful (Ok) cases
        if let Ok(length) = fields[1].parse::<u8>() {
            println!("{}, {}cm", name, length);
        }
    }
    // #[derive(Debug)]
    // enum Cereal {
    //     Barley,
    //     Millet,
    //     Rice,
    //     Rye,
    //     Splet,
    //     Wheat,
    // }

    // let mut grains: Vec<Cereal> = vec![];
    // grains.push(Cereal::Rye);
    // error bec we droppin.
    // drop(grains);
    // println!("{:?}", grains);
}
fn greet_world() {
    println!("Hello, world!");

    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];

    for region in regions.iter() {
        println!("{}", &region)
    }
}
