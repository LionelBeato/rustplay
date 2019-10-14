

fn main() {
    //input Fahrenheit value; note the decimal notation
    let y:f32 = 75.0;
    let x:f32 = converter(y);
    println!("{}", x);
}
 
 //function that converts from fahrenheit to celcius
 //note how the number type is declared and how the return type is annotated
fn converter(temp:f32) -> f32 {
    let c = (temp-32.0)*(5.0/9.0);
    //for functions that have explicit return values, simply stating the value below without a semicolon resolves
    c
}