// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                                       // str
    string("red".to_string());                                  // String (.to_string())
    string(String::from("hi"));                                 // String
    string("rust is fun!".to_owned());                          // String (.to_owned())
    string_slice("nice weather".into());                        // str (.into())
    string(format!("Interpolation {}", "Station"));             // String (format!)
    string_slice(&String::from("abc")[0..1]);                   // str (slice array)
    string_slice("  hello there ".trim());                      // str (.trim())
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String (to_string())
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());            // String (to_lowercase())
}
