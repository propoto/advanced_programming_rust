use std::collections::HashMap;
use std::fmt::{write, Debug, Display, Formatter};
use std::iter::Enumerate;
use crate::Coin::{Euro1, Euro2, FiftyCents, TenCents, TwentyCents};
use crate::Item::{Chocolate, Coffee, Coke, Muffin, Tea};

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
            Coin::FiftyCents => {self.coins += FiftyCents.to_cents();Ok("Monete inserite")}
            Coin::TwentyCents => {self.coins += TwentyCents.to_cents();Ok("Monete inserite")}
            Coin::TenCents => {self.coins += TenCents.to_cents();Ok("Monete inserite")}
            Coin::Euro1 => {self.coins += Euro1.to_cents();Ok("Monete inserite")}
            Coin::Euro2 => {self.coins += Euro2.to_cents();Ok("Monete inserite")}
        }
    }

    fn get_item_price(&self, item: &Item)-> usize{
        match item {
            Item::Coke => {60}
            Item::Coffee => {45}
            Item::Tea => {50}
            Item::Chocolate => {60}
            Item::Muffin => {120}
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