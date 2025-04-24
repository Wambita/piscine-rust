#[derive(Debug, Clone, PartialEq)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

impl<'a> Person<'a> {
	pub fn new(name: &'a str) -> Person<'a> {
            Person {
            name,
            age: 0,
        }       
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_person() {
        let person = Person::new("Leo");
        assert_eq!(person.name, "Leo");
        assert_eq!(person.age, 0);
    }
    #[test]
    fn test_person_age() {
        let mut person = Person::new("Leo");
        person.age = 25;
        assert_eq!(person.age, 25);
    }
}
