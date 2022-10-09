fn main() {
    let x = 10;

    // but has access here
    {
        let y = 5;

        println!("x is {}, y is {}.", x, y);
        //isolated
    }
    //println!("x is {}, y is {}.", x, y);
    // can't find y outside of the codeblock
}
