
#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug, Clone, Default)]
enum Membershiptype {
    #[default]
    New,
    Causual,
    Loyal,
}



impl Customer {
    fn create(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name,
            username: None,
            membership: None,
            gender: None,
            country: None,
            age: None,
        }
    }
}


struct CustomerBuilder {
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype) -> &mut Self {
        self.membership = Some(membership);
        self
    }
    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}

fn main() {
    let new_user = Customer::create("Nouman".to_string()).build();

    let user_with_login = Customer::create("Joseph".to_string())
        .username("joe123".to_string())
        .build();

    let user_with_membership = Customer::create("Micheal".to_string())
        .username("micheal2000".to_string())
        .membership(Membershiptype::Loyal)
        .build();
}


//Notes Rust lack of overloading so the best option is to use
// the buolder patterns when we try to define a differents contructors