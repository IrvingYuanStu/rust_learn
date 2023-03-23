fn greet_world() {
    let chinese = "你好，世界！";
    let english = "hello world";
    let regions = [chinese, english];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

fn main() {
    greet_world();

    let cats_data = "
    name, length(cm)
    wangcai, 80
    laifu, 75
    Invalid, data
    ";

    let records = cats_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}的长度是, {}cm", name, length)
        }

    }
}
