fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
    let a_num = 4;
    make_and_drop();

    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    //m1 e m2 sono stati spostati nella funzione greet, quindi non possono essere più utilizzati nel main, essendo che sono stati liberati
    let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved



        let mut x: Box<i32> = Box::new(1);
        let a: i32 = *x;         // *x reads the heap value, so a = 1
        *x += 1;                 // *x on the left-side modifies the heap value,
        //     so x points to the value 2

        let r1: &Box<i32> = &x;  // r1 points to x on the stack
        let b: i32 = **r1;       // two dereferences get us to the heap value

        let r2: &i32 = &*x;      // r2 points to the heap value directly
        let c: i32 = *r2;    // so only one dereference is needed to read it

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}
fn make_and_drop() {
    let a_box = Box::new(5);//allocato nella heap con Box::new
}

