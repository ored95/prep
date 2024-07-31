use std::fmt::{self, Display};

struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.age)
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.chars().count() > 0 {     // same as !s.is_empty()
            let item: Vec<_> = s.split(',').collect();
            if item.len() == 2 {
                let name: String = item[0].to_string();
                let age: usize = match item[1].parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => 0,
                };
                if !name.is_empty() && age > 0 {
                    return Person {name, age};
                }
            }
        }
        return Person::default();
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{}", p1);
    println!("{}", p2);
}