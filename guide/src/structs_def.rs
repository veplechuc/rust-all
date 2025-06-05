// a simple file with some structs definitions

#[derive(Debug, Clone)]
pub struct Algo {
    pub var1: u32,
    pub var2: u32,
}

// Adding #[derive(Clone, Debug)] applies those traits to the specific struct
#[derive(Clone, Debug)]
pub struct Otra {
    pub var11: u32,
    pub var12: u32,
}

// /// Returns the "default value" for a type.

// here we overwrite the Trait Default for the strct Algo

impl Default for Algo {
    fn default() -> Self {
        Algo { var1: 99, var2: 99 }
    }
}
