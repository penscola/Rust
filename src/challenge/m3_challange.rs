pub fn divisibility() {
    let check_this_number = 8;
    let result_variables;

    if check_this_number % 2 == 0{
        result_variables="yes";
    } else {
        result_variables= "no";
    }

    match result_variables{
        "yes" => println!("number {} can be divided by 2", check_this_number),
        "no" => println!("this number {} cannot be divided by 2", check_this_number),
        _ => println!("Did you provide the number"),
    }
}