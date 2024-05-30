fn main() {
    let mut v = vec![1,2,3];

    v.push(2);
    let _third = &v[2];
    match v.get(2){
        Some(third) => println!("the second item is {third}"),
        None => println!("none")
    }

    for i in &mut v {
        *i += 10
    }

    println!("{:?}", v);

    let mut number = vec![1.2, 3.5, 1.5];
    number.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(number, vec![1.2, 1.5, 3.5]);
}
