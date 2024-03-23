#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    ///////////////  1.内存排布true        //////////////////////////
    let f1 = File{
        name: String::from("Hi, I am LEVI_104"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    ///////////////  2.dbg! true        //////////////////////////
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}