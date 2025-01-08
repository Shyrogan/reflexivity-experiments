// Vec représente un tableau en Rust
struct MyVec<T>(Vec<T>);

impl<T> ToString for MyVec<T> {
    fn to_string(&self) -> String {
        return String::from("Bonjour, je suis un Vec !");
    }
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_type() {
        // Code si l'on souhaite utiliser notre version de ToString
        let my_vec = MyVec(vec!["".to_string()]);
        assert_eq!(my_vec.to_string(), "Bonjour, je suis un Vec !".to_string());
    }
}
