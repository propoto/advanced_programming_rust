use std::io;

fn main() {
    let number = 2;
    println!("{}", number);
    if number != 6 {
        println!("Questo numero non è 6");
    } else {
        println!("Questo numero è 6");
    }

    let c: bool = true;
    let x: i32 = if c { 3 + 4 } else { 1 };
    println!("{}", x);

    //il fatto che non sia mut non importa finché si tratta della prima assegnazione
    let k;
    k = 3;
    println!("{}", k);
    //anche se non è mut, non glie è mai stato assegnato un valore.
    let g;
    if c {
        g = 3;
    } else {
        g = 8;
    }
    println!("{}", g);

    /*    loop {
           println!("gay");
       }

    */

    let mut counter = 0;
    //loop può anche restituire un valore se messo dopo il break, quindi può assegnare valori
    let result = loop {
        counter += 1;

        if counter == 10 {
            //col break è opzionale mettere il ; perché il codice dopo non viene mai eseguito
            break counter * 2
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    //label data al loop, ora è possibile selezionare quale loop interrompere con il break
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                //di base il break interrompe il loop più interno
                //in questo caso rompe il loop di nome counting_up, quello + esterno
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //si può looppare un array ma non conviene con il while
    //perché se modifico la grandezza dell'array, devo modificare anche il range del while
    //molto meglio ed efficace è usare il for loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];
    //passa per tutti gli elementi di a, senza confrontare indici
    for element in a {
        println!("the value is: {element}");
    }

    //in generale si preferisce uitlizzare i for, più sicuri.
    //anche per cose in cui il while di base va bene
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


    //CONVERTITORE TEMPERATURA
    /*let mut temperature=String::new();

    let mut misura = String::new();

        loop {

            temperature.clear();//necessario ad ogni inizio iterazione per evitare che le stringhe si accumulino
            misura.clear();
            println!("Inserire la temperatura: ");

            io::stdin().read_line(&mut temperature).expect("Error");//read line APPENDE le stringhe, non le sovrascrive. quindi prima di ogni iterazione è FONDAMENTALE svuotare la stringa

            let temperature: u32 = match temperature.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

            println!("Inserire l'unità di misura valida della temperatura inserita, fahreneit o celsius: ");

                io::stdin().read_line(&mut misura).expect("Error");

                if misura.trim() == "fahreneit" {
                    println!("In celsius equivale a {}", (temperature - 32) * 5 / 9);
                }else if misura.trim() == "celsius" {
                    println!("In fahreneit equivale a {}", (temperature * 9 / 5) + 32);
                }else {
                println ! ("Unità di misura non valida!");
                continue
                }
                break
            }

     */

    //FIBONACCI

    //2 variabili, number otterrà il valore di input. per comodità
    let mut input = String::new();
    let number: i32;
    loop {
        input.clear();
        println!("Dimmi un numero n > 0: ");
        io::stdin().read_line(&mut input).expect("Error");
        match input.trim().parse::<i32>() {//parse::<i32>() per convertire in numero
            //copro sempre tutti quanti i casi
            Ok(n) if n >=1 => {
                number = n;
                break
            }
            _ => continue
        };
    }
    println!("L'elemento numero {number} della serie di Fibonacci è: {}",
    if number == 1 { 0 }
    else if number == 2 { 1 }
    else {fibonacci (number)})

}

fn fibonacci (n: i32) -> i32 {
    return if n <= 3 {
        1
    } else {
        //fibonacci n-1 equivale all'accumulo di 1, che verranno fuori da f(n-1-1) + f(n-1-2) e così via
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
