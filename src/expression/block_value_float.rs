//1.In rust blocks are expressions too, which mean that it can be used as value in assignments.
//2.If the last expression of the block ends with a semicolon, the return value will be ().

fn main() {
    let x = 10.0;

    let y = {
        //variable shadowing & variable binding
        let x = 20.0;
        let tmp = 13.0;

        //last expression in block is used as value in block variable
        //If the last expression of the block ends with a semicolon, the return value will be ().
        x + tmp
    };

    let z = {
        //variable shadowing & binding
        let y = 40.0;
        let tmp = 15.0;

        //last expression in block is used as value in block variable
        //If the last expression of the block ends with a semicolon, the return value will be ().
        y + tmp
    };

    let sum = x + y + z;
    println!("x:{},y:{},z:{},sum:{}", x, y, z, sum);
}
