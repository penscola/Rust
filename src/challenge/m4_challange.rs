pub fn vectors() {
    let mut v = vec![2,3,6,7,8,4];
    let mut v2 = vec![];

    for i in v {
        if i % 3 == 0 {
            println!("Great number: {}. I'am not going to delete this one.", i)
        } else {
            println!("Adding number that cannot be divided by 3: {}", i);
            v2.push(i);
        }
    }
}