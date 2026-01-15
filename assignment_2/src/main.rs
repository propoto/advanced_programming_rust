use std::collections::HashMap;

fn main() {
    let mut vec = create_vec();
    modify_odd(&mut vec);
    println!("{:?}",vec);

    let string = String::from("paaoooolaiiiiiii");
    println!("{:?}",count_characters(string));


    let slice = &[1,2,3,4,5,6,7];
    let value = 0;
    match split_at_value(slice,value) {
        Some((a,b)) => println!("{:?} --- {:?}",a,b),
        None => println!("Valore non trovato")
    }

    sub_slice(&vec![1,2,3,4,5,6],&vec![4,2,3,4,5]);//out of range,sistemare

}

//1
fn modify_odd(slice: &mut [i32]){
    for n in slice.iter_mut(){
        if *n % 2 != 0{
            *n = 0;
        }
    }
}

fn create_vec()->Vec<i32>{
    let mut vec:Vec<i32> = Vec::new();
    for i in 0..=100{
        vec.push(i);
    }
    vec
}

//2
fn count_characters(string: String) -> HashMap<char,u32>{
    let vec:Vec<char> = string.chars().collect();
    let mut hashmap:HashMap<char,u32> = HashMap::new();
    for c in vec.iter() {
        if !hashmap.contains_key(&c){
            hashmap.insert(*c,1);
        }else{
            hashmap.insert(*c,hashmap.get_key_value(c).unwrap().1+1);
        }
    }
    hashmap
}

//3
fn split_at_value(slice: &[i32],value: i32)->Option<(&[i32],&[i32])> {
    for i in 0..slice.len() {
        if slice[i] == value {
            return Some((&slice[0..=i],&slice[i+1..slice.len()]))
        }
    }
    None
}

//4
fn sub_slice(vec1: &Vec<i32>, vec2: &Vec<i32>){
    let mut is_equal = false;
    for i in 0..vec1.len(){
        if vec1[i] == *vec2.first().unwrap(){
            let slice = &vec1[i..i+vec2.len()];
            if slice == vec2 {
                is_equal = true;
                break
            }
        }
    }
    if is_equal { println!("{:?}",vec2) } else { println!("Not found.") }
}