use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f =File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn main() {
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
}
