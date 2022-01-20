use std::io;
use std::error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Myerror {
    e: String
}

impl From<String> for Myerror {
    fn from(e: String) -> Myerror {
        Myerror { e }
    }
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn prova() -> Result<String, io::Error> {
    Err(io::ErrorKind::InvalidData)?
}

fn prova2() -> Result<(), Box<dyn error::Error>> {
    Err("hello error")?
}

fn prova3() -> Result<(), Myerror> {
    Err("altro errore".to_string())?
}

fn main() {
    println!("Hello, world!");
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    let d: Result<&str, &str> = Result::Ok("hello");
    let d = d.unwrap();
    dbg!(d);
    let d: Result<&str, &str> = Result::Err("Failed!");
    let d = d.unwrap_or_else(|e| {"ciao"});
    dbg!(d);
    dbg!(prova());
    dbg!(prova2());
    dbg!(prova3());
}
