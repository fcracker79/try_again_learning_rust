struct Pippo {
    field: String
}

fn main() {
    let mut p = Pippo {field: String::from("hello")};
    p.field = String::from("hi");
    p = Pippo {field: String::from("hi there")};

    let p = Pippo {field: String::from("hello")};
    // Once a variable is NOT mutable, I cannot change either its value nor any part of it.
    // p.field = String::from("hi");
    // p = Pippo {field: String::from("hi there")};
}