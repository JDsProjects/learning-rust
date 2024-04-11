fn main(){
    
    let empty_name;
    empty_name = "LOL";
    
    println!("{}", empty_name);
    // so a {} is needed for printing variables????
    
    let test: &str = "test";
    let test : String = test.to_uppercase();
    println!("{}", test);
    // neat
    
    let test: String = test.to_lowercase();
    println!("{}", test);
    // lowercase function and then print.
}
