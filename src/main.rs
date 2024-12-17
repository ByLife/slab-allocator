fn main() {
    let tab = [1, 2, 3, 4, 5];
    
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    let stro = String::from("salut");
    
    let boite = Box::new(42);
    
    let mut str_test = String::with_capacity(20);
    str_test.push_str("test");
    
    println!("mon tab: {:?}", tab);
    println!("mon vec: {:?}", vec);
    println!("ma string lol: {}", stro);
    println!("ma boite chelou: {}", boite);
    println!("str de test: {} (capa: {})", str_test, str_test.capacity());
 }