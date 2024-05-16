fn main() {
    let x:i32 = 10;
    let y:i32 = 20;
    let z:f32 = 30.0;
    let is_male:bool = true;
    let is_over_18:bool = false;


    println!("x = {}, y = {}, z = {}" ,x,y,z);

    if is_male{
        println!("You are a male ")
    } else{
        println!("You are not a male")
    }

    if is_male && is_over_18{
        println!("You are an adult.")
    } else{
        println!("You aren't an adult")
    }

}


