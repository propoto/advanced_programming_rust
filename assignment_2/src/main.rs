use std::slice::Iter;
use std::collections::HashMap;

#[derive(Debug)]
enum DoubleType {
    T1(i32),
    T2(String)
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div
}
enum Expression {
    Number(i32),
    Operation(Box<Expression>,Box<Expression>,Operation)
}

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

    sub_slice(&vec![1,2,3,4,5,6],&vec![2,3,4,5]);

    let mut vec = vec![4, 2, 5, 1];
    println!("{:?} Max: {}",vec, max(&vec));
    swap(&mut vec);
    println!("{:?}",vec);
    if is_sorted(&vec){
        println!("vec è ordinato in ordine crescente.");
    }else { println!("vec non è ordinato in ordine crescente."); }
    let mut string_vec = vec![String::from("Culo"),String::from("Cazzo"),String::from("Pirlo")];
    let string = String::from("CIAOOO");
    insert_if_longer(&mut string_vec,string);
    println!("{:?}",string_vec);

    let vec = vec![1,2,3];
    let iter = vec.iter();
    let vec_from_iter = build_vector(iter);
    println!("{:?}",vec_from_iter);

    let mut vec = vec![65,2,4,7,3,93,1,3,6];
    pancake_sort(&mut vec);
    println!("{:?}",vec);

    let s1 = &[1,4,7,9];
    let s2 = &[3,5,8,12];
    println!("{:?}",merge(s1,s2));

    //9
    let vec: Vec<DoubleType> = vec![DoubleType::T1(2),DoubleType::T2(String::from("idiota"))];
    println!("{:?}",vec);

    let exp:Expression = Expression::Operation(Box::from(Expression::Number(22222)),Box::from(Expression::Number(111111110)),Operation::Mul);
    let res = evaluate_expression(exp);
    match res {
        Ok(v) => println!("{}",v),
        Err(e)=>println!("{}",e)
    }
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
        if vec1[i] == *vec2.first().unwrap() && !(i+vec2.len()>vec1.len()){
            let slice = &vec1[i..i+vec2.len()];
            if slice == vec2 {
                is_equal = true;
                break
            }
        }
    }
    if is_equal { println!("{:?}",vec2) } else { println!("Not found.") }
}

//5
fn max(vec: &Vec<i32>)->i32{
    let mut max = vec[0];
    let mut max_so_far = vec[0];
    for i in 1.. vec.len(){
        if max < vec[i] {
            max = vec[i];
            if max > max_so_far { max_so_far = max; }
        }
    }
    max_so_far
}
fn swap(vec: &mut Vec<i32>){
    let last = vec.len()-1;
    let tmp = *vec.first().unwrap();
    vec[0] = vec[last];
    vec[last] = tmp;

}

fn is_sorted(vec: &Vec<i32>)->bool{
    let mut increasing = true;
    for i in 0..vec.len()-1 {
        if vec[i] > vec[i+1] { increasing = false;break }
    }
    increasing
}

fn insert_if_longer(vec: &mut Vec<String>,string: String){
    if string.len() > 10 {
        vec.push(string);
    }
}

//6
fn build_vector(iter: Iter<i32>)-> Vec<&i32>{
    let mut vec: Vec<&i32> = Vec::new();
    for n in iter{
        vec.push(&n);
    }
    vec
}

//7
fn pancake_sort(vec: &mut Vec<i32>){
    let mut final_index = vec.len() - 1;
    while final_index > 0 {
        let max = max_index(vec,final_index);
        if max != final_index {
            let slice1 = &mut vec[0..=max];
            slice1.reverse();
            vec[0..=final_index].reverse();
        }
        final_index-=1;
    }

}

fn max_index(vec: &Vec<i32>,final_index:usize)->usize{
    let mut max = vec[0];
    let mut max_so_far = vec[0];
    let mut index = 0;
    for i in 1..=final_index{
        if max < vec[i] {
            max = vec[i];
            if max > max_so_far { max_so_far = max; index = i; }
        }
    }
    index
}

//8
fn merge(slice1: &[i32], slice2: &[i32]) -> Vec<i32>{
    let mut vec:Vec<i32> = [slice1,slice2].concat();
    vec.sort();
    vec
}

//10
fn evaluate_expression(exp:Expression)->Result<i32,String>{
    match exp {
        Expression::Number(a) => { Ok(a)}
        Expression::Operation(a, b, op) => {
            let left_value = evaluate_expression(*a);
            let right_value = evaluate_expression(*b);
            match op {
                Operation::Add => {
                    let r = left_value?.checked_add(right_value?);
                    match r {
                        None => {Err(String::from("Overflow"))}
                        Some(v) => {Ok(v)}
                    }
                }
                Operation::Sub => {
                    let r = left_value?.checked_sub(right_value?);
                    match r {
                        None => {Err(String::from("Overflow"))}
                        Some(v) => {Ok(v)}
                    }
                }
                Operation::Mul => {
                    let r = left_value?.checked_mul(right_value?);
                    match r {
                        None => {Err(String::from("Overflow"))}
                        Some(v) => {Ok(v)}
                    }
                }
                Operation::Div => {
                    let r = left_value?.checked_div(right_value?);
                    match r {
                        None => {Err(String::from("Diviso per 0"))}
                        Some(v) => {Ok(v)}
                    }
                }
            }
        }
    }
}
