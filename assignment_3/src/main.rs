use std::collections::HashMap;
use std::fmt::{write, Debug, Display, Formatter};
use std::iter::Enumerate;
use crate::Coin::{Euro1, Euro2, FiftyCents, TenCents, TwentyCents};
use crate::Item::{Chocolate, Coffee, Coke, Muffin, Tea};
use crate::sentence::Sentence;
use crate::test_sentence::magic_sentence;

//2
enum Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IP{
    IPv4([i8; 3]),
    IPv6([u16; 8])
}

struct Point {
    x: f64,
    y: f64,
    z: f64
}
//4
#[derive(Eq, Hash, PartialEq,Clone)]//Necessario se si vuole usare in una Hashmap
enum Item{
    Coke,
    Coffee,
    Tea,
    Chocolate,
    Muffin
}

enum Coin{
    FiftyCents,
    TwentyCents,
    TenCents,
    Euro1,
    Euro2
}

impl Coin {
    fn to_cents(&self) -> usize{
        match self {
            FiftyCents => {50}
            TwentyCents => {20}
            TenCents => {10}
            Euro1 => {100}
            Euro2 => {200}
        }
    }
}

struct VendingMachine {
    coins: usize,
    items: HashMap<Item,usize>
}

impl VendingMachine{
    fn new( hash_map: HashMap<Item,usize>) -> VendingMachine{
        VendingMachine {
            coins: 0,
            items: hash_map
        }
    }
    fn add_item(&mut self, item: Item, n: usize){
        let new_value = n + *self.items.get(&item).unwrap();
        self.items.insert(item, new_value);
    }

    fn insert_coin(&mut self, coin: Coin)-> Result<&'static str,&'static str>{
        match coin {
            FiftyCents => {self.coins += FiftyCents.to_cents();Ok("Monete inserite")}
            TwentyCents => {self.coins += TwentyCents.to_cents();Ok("Monete inserite")}
            TenCents => {self.coins += TenCents.to_cents();Ok("Monete inserite")}
            Euro1 => {self.coins += Euro1.to_cents();Ok("Monete inserite")}
            Euro2 => {self.coins += Euro2.to_cents();Ok("Monete inserite")}
        }
    }

    fn get_item_price(&self, item: &Item)-> usize{
        match item {
            Coke => {60}
            Coffee => {45}
            Tea => {50}
            Chocolate => {60}
             Muffin => {120}
        }
    }

    fn remove_item(&mut self, item: &Item) -> Result<usize, &'static str>{
        let mut quantity = *self.items.get(&item).unwrap();
        if quantity > 0 {
            quantity -=1;
            self.items.insert(item.clone(), quantity);
            return Ok(quantity)
        }
        Err("Il prodotto non è disponibile")

    }

    fn remove_and_buy(&mut self, item: Item) -> Result<usize, &'static str>{
        match self.remove_item(&item) {
            Ok(v) => {
                println!("Ci sono ancora {} prodotti di questo tipo disponibili.",v);
                let price = self.get_item_price(&item);
                if self.coins >= price {
                    let change = self.coins - price;
                    self.coins = 0;
                    return Ok(change)
                }
                Err("Non hai soldi a sufficienza")
            }
            Err(s) =>  Err(s)
        }
    }

    fn buy(&mut self, item: Item) -> Result<usize, &'static str>{
        match item {
            Coke => {
                match self.remove_and_buy(Coke){
                    Ok(v) => { Ok(v) }
                    Err(s) => { Err(s) }
                }
            }
            Coffee => {
                match self.remove_and_buy(Coffee){
                    Ok(v) => { Ok(v) }
                    Err(s) => { Err(s) }
                }
            }
            Tea => {
                match self.remove_and_buy(Tea){
                    Ok(v) => { Ok(v) }
                    Err(s) => { Err(s) }
                }
            }
            Chocolate => {
                match self.remove_and_buy(Chocolate){
                    Ok(v) => { Ok(v) }
                    Err(s) => { Err(s) }
                }
            }
            Muffin => {
                match self.remove_and_buy(Muffin){
                    Ok(v) => { Ok(v) }
                    Err(s) => { Err(s) }
                }
            }
        }
    }
}
#[derive(Debug)]
struct Date(u8,u8,u16);
#[derive(Debug)]
struct Hour(u8,u8);

impl Display for Date{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:02}/{:02}/{:04}",self.0,self.1,self.2)
    }
}
impl Display for Hour{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:02}:{:02}",self.0,self.1)
    }
}

#[derive(Debug)]
struct BoxShipping{
    name:String,
    barcode:String,
    shipment_date:Date,
    shipment_hour:Hour
}

impl Display for BoxShipping{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Name: {}\nBarcode: {}\nDate: {}\nHour: {}"
        ,self.name,self.barcode,self.shipment_date,self.shipment_hour)
    }
}

enum Month{
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}
enum Product{
    Book(String,String),
    Article(String),
    Magazine(usize,Month)

}

impl Product{
    fn to_string(&self) -> String{
        match self {
            Product::Book(a, pb) => {
                format!("Book. Author = {}, Publishing company = {}",a,pb)
            }
            Product::Article(o) => {
                format!("Article. Orchid = {}",o)
            }
            Product::Magazine(n, m) => {
                format!("Magazine. Number = {}, Month = {}",n,m)
            }
        }
    }
}

struct Library{
    shelf: Vec<Product>
}
impl Library {
    fn new(vec:Vec<Product>) -> Library{
        Library{
            shelf : vec
        }
    }

    fn add(&mut self,product: Product ){
        self.shelf.push(product)
    }
}

impl Display for Month{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Month::January => {write!(f,"January")}
            Month::February => {write!(f,"February")}
            Month::March => {write!(f,"March")}
            Month::April => {write!(f,"April")}
            Month::May => {write!(f,"May")}
            Month::June => {write!(f,"June")}
            Month::July => {write!(f,"July")}
            Month::August => {write!(f,"August")}
            Month::September => {write!(f,"September")}
            Month::October => {write!(f,"October")}
            Month::November => {write!(f,"November")}
            Month::December => {write!(f,"December")}
        }
    }
}

impl Display for Product{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Product::Book(a, pb) => {
                write!(f,"Book. Author = {}, Publishing company = {}.",a,pb)
            }
            Product::Article(o) => {
                write!(f,"Article. Orchid = {}.",o)
            }
            Product::Magazine(n, m) => {
                write!(f,"Magazine. Number = {}, Month = {}.",n,m)
            }
        }
    }
}

impl Display for Library{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for p in self.shelf.iter() {
            str.push_str(p.to_string().as_str());
            str.push_str("\n");
        }
        write!(f,"{}",str)
    }
}

mod point {
    pub struct Pointz{
        pub x:f32,
        pub y:f32
    }
    impl Pointz{
        pub fn new(x:f32,y:f32)->Pointz{
            Pointz{
                x,
                y
            }
        }
        fn distance(&self, point: &Pointz) ->f32{
            ((self.x -point.x).abs().powi(2) + (self.y -point.y).abs().powi(2)).sqrt()
        }
    }
}

mod line {
    use crate::point;

    pub struct Line{
        start: point::Pointz,
        end: point::Pointz,
        m:f32,
        q:f32
    }
    impl Line{
        pub fn new(start: point::Pointz, end: point::Pointz) -> Line{
            let m = (end.y - start.y)/(end.x - start.x);
            let q = end.y - start.y - m * (end.x - start.x);
            Line{ start, end, m, q }
        }
        pub fn contains(&self,p: &point::Pointz)-> Result<(),String>{
            if self.m * p.x + self.q - p.y == 0.0{
                return Ok(())
            }
            Err(String::from("Il punto non appartiene alla retta"))
        }
    }



}

mod test{
    use crate::point;
    use crate::line;
    pub fn test(){
        let start = point::Pointz::new(1.0,1.0);
        let end = point::Pointz::new(3.3,3.0);
        let point = point::Pointz::new(2.0,2.0);
        let line = line::Line::new(start,end);

        match line.contains(&point) {
            Ok(_) => println!("La linea contiene il punto"),
            Err(s) => println!("{}",s),
        }
    }
}

mod sentence{
    pub struct Sentence{
        pub words: Vec<String>
    }
    impl Sentence {
        pub fn new_default()->Self{
            Sentence{
                words: Vec::new()
            }
        }
        pub fn new(&mut self, str: &str){
            for s in str.split_whitespace(){
                self.words.push(String::from(s))
            }
        }

    }
}

impl Display for Sentence{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for s in self.words.iter(){
            str.push_str(s.as_str());
            str.push(' ');
        }
        write!(f,"{}",str)
    }
}

mod test_sentence{
    use std::collections::HashMap;
    use crate::sentence::Sentence;

    pub fn magic_sentence(hash_map: &HashMap<i32,Sentence>,i: i32, j:i32) -> Result<Sentence,&str>{
        if hash_map.contains_key(&i) && hash_map.contains_key(&j) {
            let s1 = hash_map.get(&i).unwrap();
            let s2 = hash_map.get(&j).unwrap();
            let mut str = String::new();

            for (i,s) in s1.words.iter().enumerate(){
                if *s == s2.words[i] {
                    str.push_str(&*s);
                    str.push(' ');
                }
            }
            let mut sentence = Sentence::new_default();
            sentence.new(str.as_str());
            if sentence.words.is_empty() {
                return Err("Nessuna parola in comune trovata")
            }
            return Ok(sentence)
        }
        Err("Indici non esistenti nell'hash map.")
    }
}
fn main() {

    let string = String::from("245719");
    if is_it_luhn(&string) { println!("Numero di carta valido") } else { println!("Numero di carta non valido") }


    let numberplate = String::from("FL44G8KZ");
    let mut hash_map: HashMap<String, String> = HashMap::new();
    hash_map.insert(String::from("EM886PK"), String::from("Federico Valsecchi"));
    hash_map.insert(String::from("FL448KZ"), String::from("Manuela Divina"));
    hash_map.insert(String::from("KG341HL"), String::from("Luca Brizzante"));
    match recognise_owner(&hash_map,&numberplate) {
        None => {println!("Targa non esistente")}
        Some(v) => {println!("Proprietario: {}",v)}
    }


    let mut items:HashMap<Item,usize> = HashMap::new();
    items.insert(Coke,0);
    items.insert(Coffee,10);
    items.insert(Tea,7);
    items.insert(Chocolate,2);
    items.insert(Muffin,15);
    let mut vending_machine = VendingMachine::new(items);
    vending_machine.add_item(Muffin,1);
    let coin = Euro2;
    let item = Coke;
    match vending_machine.insert_coin(coin) {
        Ok(s) => {println!("{}",s)}
        Err(s) => {println!("{}",s)}
    }
    match vending_machine.buy(item) {
        Ok(v) => {
            println!("Il tuo resto: {}",v);
        }
        Err(s) => {
            println!("{}",s);
        }
    }
    let date = Date(12,11,2001);
    let hour = Hour(0,45);

    let boxshipping = BoxShipping{name: String::from("Fumetto"),
        barcode: String::from("371284378432"),
        shipment_date:date,
        shipment_hour:hour};
    println!("{}",boxshipping);

    let mut vec:Vec<Product> = Vec::new();
    vec.push(Product::Book(String::from("Paul Walker"),String::from("Annapurna")));
    vec.push(Product::Article(String::from("Dark")));
    vec.push(Product::Magazine(12,Month::April));
    let mut library = Library::new(vec);
    let new_product = Product::Article(String::from("Comedy"));
    library.add(new_product);
    println!("{}",library);
    test::test();


    let str = "Ciao mi chiamo Gino ed ho 15 anni";
    let str2 =  "ds Ciao mi chiamo Paolo ed ho 23 anni";
    let mut sentence:Sentence = Sentence::new_default();
    let mut sentence2:Sentence = Sentence::new_default();
    sentence.new(str);
    sentence2.new(str2);
    let mut hash_map:HashMap<i32,Sentence> = HashMap::new();
    hash_map.insert(0,sentence);
    hash_map.insert(2,sentence2);

    match magic_sentence(&hash_map,0,1) {
        Ok(s) => {
            println!("{}",s)
        }
        Err(e) => {
            println!("{}",e)
        }
    }
}

//1
fn is_it_luhn(string: &String)-> bool{
    let mut str = string.clone();
    if string.len() > 1 {
        str = str.replace(' ',"").parse().unwrap();
        let mut odd_vec = Vec::new();
        let mut sum = 0;
        for (i,c) in str.chars().enumerate() {
            let mut n = c.to_digit(10).unwrap();
            if i % 2 == 1 && i != 0{
                n *=2;
                if n > 9 { n = n-9; }
                odd_vec.push(n);

            }else { odd_vec.push(n); }
            sum += odd_vec[i];
        }
        if sum%10 == 0 { return true }

    }
    false
}

//3
fn recognise_owner(hash_map: &HashMap<String,String>, numberplate: &String) -> Option<String> {
    hash_map.get(numberplate).cloned()
}