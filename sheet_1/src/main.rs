const SPEED_LIGHT: i32 = 300_000_000;

fn main() {
    //1. Write a function string_reverse that takes a &str as
    // input and returns it, reversed as a String

    let s = "babbonatale";
    let mut g =  String::from("cazzone");
    println!("{}", s);
    println!("{}", string_reverse(s));
    string_reverse_mut(&mut g);
    println!("{}",g);

    //2. Write a function bigger that takes two i32
    // and returns the bigger number ( i32 ) without using
    // another function call and additional variables

    let a = 20;
    let b = 15;
    let biggest = bigger(a, b);
    println!("Il numero maggiore è: {}", biggest);
    println!("{}", a); //non c'entra un cazzo con l'es., ma qua posso stampare a perché i32 è Copy, quindi non sposto l'ownership nell funzione, ma solo una copia di a bit per bit

    //3. Write a function multiply that takes an i32,
    // a f32 and a f64 and returns the multiplication of the
    // three of them as a f64 value;

    let n1: i32 = 12;
    let f2: f32 = 2.43;
    let f3: f64 = 7.588;

    println!(
        "Il risultato di {} * {} * {} è: {}",
        n1, f2, f3, multiply(n1, f2, f3)
    );

    //4. Write a function e_equals_mc_squared that takes
    // as input a f32 representing the mass, and that
    // uses a globally-defined constant containing
    // the value of the speed of light in a vacuum (expressed in
    // m/s). The function outputs the energy
    // equivalent to the mass input;

    let mass: f32 = 555.32;
    println!("L'energia ottenuta data dalla massa m = {} equivale a: {}",
    mass,e_equals_mc_squared(mass));



    //5.Given a vector of i32 , create a function max_min
    // that returns the maximum and the minimum value
    // inside that vector;

    let v = vec![3,1,43,77,55,2,11];
    let max_and_min = max_min(v);
    println!("Il valore massimo di v è: {}, mentre il minore è: {}.",
    max_and_min.0, max_and_min.1);

    //6.Write a function lord_farquaad that takes a String
    // and outputs another String in which every
    // character 'e' is substituted by the character '💥';

    let string = String::from("ebeereeeete");
    println!("{}",lord_farquaad(string));


}
//1
fn string_reverse(str: &str) -> String {
    let mut string = String::new();
    for c in str.chars().rev() {
        string.push(c);
    }
    string
}
//1.1, inutile ma istruttivo
fn string_reverse_mut(str: &mut String){
    let mut chars: Vec<char> = str.chars().collect();
    let lenght = chars.len();
    for i in 0.. lenght/2  {
        chars.swap(i,lenght-i-1);
    }
    *str = chars.into_iter().collect();
}
//2
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
//3
fn multiply(a: i32, b: f32, c: f64) -> f64 {
    a as f64 * b as f64 * c
}
//4
fn e_equals_mc_squared(m: f32) -> f32{
    m * SPEED_LIGHT as f32 * SPEED_LIGHT as f32
}
//5
fn max_min(v: Vec<i32>) -> (i32, i32) {
    let mut v1 = v.clone();
    v1.sort();
    (*v1.last().unwrap(), *v1.first().unwrap())
    // metto l'asterisco perché devo deferenziarlo,
    // v.first mi restituisce un Option<&i32>,
    // con .unwrap ottengo &i32, e con * ottengo i32
    //grazie a *, ottengo il valore puntato da &i32, *&i32
}
    //6
    fn lord_farquaad(s: String) -> String{
        let mut chars: Vec<char> = s.chars().collect();
        for c in chars.iter_mut() {

            if *c == 'e' {
                *c = '💥';
            }
        }

        let sf: String = chars.into_iter().collect();
        sf

    }



