
use std::error::Error;
use std::io::{self, Read};
use std::fs::{self, File};

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn find_value(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Секретное число в пределах от 1 до 100, получено {}", value);

        }

        Guess { value }

    }

        pub fn value(&self) -> i32 {
            self.value
        }
    }



 fn main() //-> Result<(), Box<dyn Error>>// 
 {
    // let v = vec![1,2,3];
    // // let f = File::open("hello.txt").unwrap_or_else(|error| {
    // //     if error.kind() == ErrorKind::NotFound {
    // //         File::create("hello1.txt").unwrap_or_else(|error| {
    // //             panic!("Проблема с созданием файла: {:?}", error);
    // //         })
    // //     } else {
    // //         panic!("Проблема с открытием файла: {:?}", error);
    // //     }
    // // });
    // // let f = match f {
    // //     Ok(file) => file,
    // //     Err(error) => match error.kind() {
    // //         ErrorKind::NotFound => match File::create("hello.txt") {
    // //             Ok(fc) => fc,
    // //             Err(e) => panic!("Проблема с созданием файла: {:?}", e),
    // //         },
    // //         other_error => panic!("Проблема с открытием файла: {:?}", other_error),
    // //     }
    // let f = File::open("hello.txt").expect("Не удалось открыть файл hello.txt");                             
    // let mut f = File::open("hello.txt")?;
    // Ok(())
    
}
