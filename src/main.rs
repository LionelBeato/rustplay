fn main() {
    let x:f32 = converter(75.0);
    println!("{}", x);
}
 
 //function that converts from fahrenheit to celcius

fn converter(temp:f32) -> f32 {
    let c = (temp-32.0)*(5.0/9.0);
    c
}