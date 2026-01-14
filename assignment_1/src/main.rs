use std::collections::HashMap;

const SPEED_LIGHT:i32 = 300_000_000;
type Matrix = ((i32, i32), (i32, i32));
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
    println!("Energia = {}", e_equals_mc_squared(m));

    let v = vec![3,1,5,6,12,543,0];
    println!("Maggiore = {}. Minore = {} ",max_min(&v).0, max_min(&v).1);

    let string = String::from("pellicanessa");
    println!("{} -> {}", string, lord_farquaad(&string));

    let hash_map = HashMap::from([(String::from("Pasta"),1.5),(String::from("Pesto"),2.5),(String::from("Posta"),6.5),(String::from("Pista"),0.5) ]);
    println!("{:?}",hash_map);
    let furniture = String::from("Pita");
    let furniture_value = get_value(&hash_map,&furniture);
    if furniture_value != -1.0 {
        println!("{} è presente nell'Hashmap e vale {}",furniture, furniture_value)
    }else { println!("{} non è presente", furniture) }

    let string = String::from("tomare");
    let string2= append(string.clone());
    println!("{} -> {}",string, string2);

    let number = 2;
    if is_armstrong(number){
        println!("{} è un numero di Armstrong.",number);
    }else{
        println!("{} non è un numero di Armostrong.",number);
    }

    let matrix:Matrix = ((1,2),(3,4));
    println!("{:?} -> {:?}", matrix, matrix_transposer(matrix));

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
fn e_equals_mc_squared(m:f32) -> f32{
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

//6
fn lord_farquaad(string: &String) -> String{
    let mut v:Vec<char> = string.chars().collect();
    for c in v.iter_mut() {
        if *c == 'e'{
            *c = '💥';
        }
    }
    let v:String = v.into_iter().collect();
    v
}

//7
fn get_value(hash_map: &HashMap<String,f32>, furniture: &String) -> f32{
    if hash_map.contains_key(furniture)
    { *hash_map.get_key_value(furniture).unwrap().1 }
    else { -1.0 }

}

//8
fn append(mut string:  String) -> String{
    string.push_str("foobar");
    string
}

//9
fn is_armstrong(num: i32) ->bool{
    let vec: Vec<char> = num.to_string().chars().collect();
    let mut sum:i32 = 0;
    for c in vec.iter(){
        sum += (c.to_digit(10).unwrap() as i32).pow(vec.len() as u32);
    }
    if sum == num { true } else { false}


}

//10
fn matrix_transposer(mut m:Matrix) -> Matrix{
    let tmp:i32 = m.0.1;
    m.0.1 = m.1.0;
    m.1.0 = tmp;
    m
}