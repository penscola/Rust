pub fn m2() {
    let number1 = 5;
    let number2 = 6;

    {
        let result = number1 * number2;
        println!("Result in the scope space = {}", result);
    }

    let result = number1 * number2 * 2;
    println!("The result in the global space = {}", result);
}