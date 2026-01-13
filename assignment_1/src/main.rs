const SPEED_LIGHT:i32 = 300_000_000;

fn main() {
    let str = "ciao";
    println!("{} -> {}",str, string_reverse(str));

    let a = 7;
    let b = 5;
    println!("Il numero più grande è: {}", bigger(a,b));

    let a = 9;
    let b = 3.4;
    let c = 1.1234;
    println!("Risultato della moltiplicazione: {}", multiply(a,b,c));

    let m:f32 = 15.6;
    println!("Energia = {}", e_equals_mc_sqaured(m));

    let v = vec![3,1,5,6,12,543,0];
    println!("Maggiore = {}. Minore = {} ",max_min(&v).0, max_min(&v).1)
}

//1
fn string_reverse(str: &str) -> String {
    let string = str.chars().rev();
    string.collect()
}

//2
fn bigger(a:i32,b:i32) -> i32{
    if a >= b {a} else { b }
}

//3
fn multiply(a:i32, b:f32, c:f64) -> f64{
    a as f64 *b as f64 * c
}

//4
fn e_equals_mc_sqaured(m:f32) -> f32{
    m * SPEED_LIGHT as f32 * SPEED_LIGHT as f32
}

//5
fn max_min(v:&Vec<i32>) -> (i32,i32){
    let mut max = v[0];
    let mut max_so_far = v[0];
    let mut min = v[0];
    let mut min_so_far = v[0];
    for i in 1..v.len() {
        if v[i] > max {
            max = v[i];
            if max > max_so_far { max_so_far = max; }
        }
        if v[i] < min {
            min = v[i];
            if min < min_so_far { min_so_far = min; }
        }
    }

    (max_so_far,min_so_far)
}