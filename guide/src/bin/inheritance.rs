#![allow(unused)]

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Ant {
    name: String,
}

struct Spider {
    name: String,
}


//THINK ON TRAITS AS A SOME KIND OF INTERFACE
// the method can implemented or not
// if it is implemented
// the specific implementation of the struct
// can use it as default. check Ant or Spider impl
trait Animal {
    // fn talk(&self) -> String; // &self, allows the method to acces the data in the implemented struct
    fn talk(&self) -> String {
        String::from("Cant talk!!") // default implementation
    }
}

impl Animal for Dog {
    fn talk(&self) -> String {
        String::from("DOg  makes sounds -- from Dog impl")
    }
}

// define a DEFAULT IMPLE FOR THOSE THAT HAVE SAME CODE
impl Animal for Ant {}

impl Animal for Spider {}

// TRAITS AS PARAMETERS AND TRAITS BOUNDS

// here we define the trait bounds - a trait as param
fn make_animal_talk(animal: &impl Animal) {
    // implement Animal traits instead of an concrete implementation
    println!(
        "imprimimos lo correspondiente implementado en el trait --> {}",
        animal.talk()
    );
}
// USANDO GENERICS
fn make_generic_talk<T>(animal: &T)
where
    T: Animal,
{
    println!("imprimimos usando generics --> {}", animal.talk());
}


fn main() {
    let dog = Dog {
        name: "SmeDOG".to_string(),
    };

    let ant = Ant {
        name: "Ant NAME".to_string(),
    };

    let spider = Spider {
        name: "Spider NAME".to_string(),
    };

   // println!("{} -->> {}",dog.name, dog.talk());
   // println!("{} -->> {}",ant.name, ant.talk());
   // println!("{} -->> {}",spider.name, spider.talk());
    make_animal_talk(&dog);
    make_animal_talk(&ant);
   
    make_generic_talk(&spider);
    make_generic_talk(&dog);
}