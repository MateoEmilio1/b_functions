#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;


    {
    let area = area_of(width, height);
    println!("Area is {}", area);

    }

    println!("Volume is {}", volume(width, height, depth));

}

fn area_of(x: i32, y:i32) -> i32 {


    return x*y;
}

fn volume(x: i32, y:i32, z: i32) -> i32 {

    //devuelvo de forma "tail expression" like a Rust programmer would do 
    x*y*z
}