// -------------------------------------------
// 			Basic File Handling
// -------------------------------------------
use std::fs::*;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

//    fn basic_file_handling() -> std::io::Result<()> {
//        let path_loc = r"~/Destop/my_text.txt";
//        let path = Path::new(path_loc);
//        let mut file = File::create(path)?;

//        //file.write(b"let's put this in the file")?;
//        file.write_all("let's put this in the file".as_bytes())?;

//        // let mut file = OpenOptions::new().append(true).open(path)?;
//        // file.write("\n www.includehelp.com\n".as_bytes())?;

//        // let str1 = "nouman";
//        // file.write(str1.as_bytes())?;

//        let some_vec = vec![1,2,3,4,5,6];
//        let str_from_vec = some_vec
//        .into_iter()
//        .map(|a| format!("{} \n", a.to_string()))
//        .collect::<String>();

//        file.write_all(str_from_vec.as_bytes())?;

//        // let (name, age) = ("Joseph", 40);
//        // let formatted_str = format!("I am {} and my name is {}", name, age);
//        // file.write(formatted_str.as_bytes())?;

//        // let mut file = File::open(path)?;
//        // let mut contents = String::new();
//        // file.read_to_string(&mut contents)?;
//        // println!("The file contains {:?}", contents);

//        let file = File::open(path);
//        let mut fileok = match file {
//         Ok(file) => file,
//         Err(error) => {println!("Error occurs...{} ", error);
//         Ok(()) ;
//                     }
//        };

//        let file_buffer = BufReader::new(file);
//        for lines in file_buffer.lines(){
//            println!("{:?}", lines?);
//        }
//        Ok(())

//    }
fn main() {
    let path_loc = r"/Users/vplechuc/Desktop/my_text.txt";
    let path = Path::new(path_loc);
    let mut file = File::create(path);

    file.expect("Error writing file")
        .write_all(b"let's put this in the file");
    //file("let's put this in the file".as_bytes());

    let file = File::open(path);
    let fileok = match file {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let file_buffer = BufReader::new(fileok);
    for line in file_buffer.lines() {
        let msg = line.unwrap();
        println!("{:?}", msg);
    }
}
