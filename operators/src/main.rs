macro_rules! bool_ops {
    (if_not_true $expr:expr => { $($body:tt)* }!) => {
        if !$expr {
            $($body)*
        }
    };
    (if_not_false $expr:expr => { $($body:tt)* }!) => {
        if $expr {
            $($body)*
        }
    };
    (repeat_until { $($body:tt)* } when ($cond:expr)!) => {
        loop {
            $($body)*
            if $cond { break; }
        }
    };
}

fn condition() -> bool {
    static mut COUNTER: i32 = 0;
    unsafe {
        COUNTER += 1;
        COUNTER >= 3
    }
}

fn main() {
    let x = 3;

    bool_ops! {
        if_not_true (x > 5) => {
            println!("x n'est pas supérieur à 5");
        }!
    }

    let y = true;
    bool_ops! {
        if_not_false y => {
            println!("y est true");
        }!
    }

    println!("Début de la boucle");
    bool_ops! {
        repeat_until {
            println!("itération");
        } when (condition())!
    }
    println!("Fin de la boucle après 3 itérations");
}
