use std::fs;

#[allow(dead_code)]
mod interpretation;
use interpretation::*;


fn main() {
    let filename = "program.mk43";

    let contents = fs::read_to_string(filename)
        .expect("Could not read the file"); 

    let grid = DynGrid::from(contents);

    println!("{}", grid.to_string());

    interpret(grid.clone(), |_| "0".to_string(), |x| println!("{}", x));

    compile(grid);
}
