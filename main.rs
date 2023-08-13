use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;


fn genwordlist(userlist: Vec<String>){

    let mut file = File::create("wordlist.txt").unwrap();


    for mut n in userlist {

        n = n.to_lowercase();
        let name:Vec<&str> = n.split(' ').collect();

        // ex. jdoe
        let mut first = name[0].chars().nth(0).unwrap().to_string();
        first.push_str(name[1]);
        writeln!(file, "{}", first).unwrap();
        //ex. johnd
        let mut second = name[1].chars().nth(0).unwrap().to_string();
        first = name[0].to_string();
        first.push_str(&second);
        writeln!(file, "{}", first).unwrap();
        //ex. djohn
        second = name[1].chars().nth(0).unwrap().to_string();
        second.push_str(name[0]);
        writeln!(file, "{}", second).unwrap();
        // ex. j.doe
        first = name[0].chars().nth(0).unwrap().to_string();
        first.push_str(".");
        first.push_str(name[1]);
        writeln!(file, "{}", first).unwrap();
        // ex. d.joe
        second = name[1].chars().nth(0).unwrap().to_string();
        second.push_str(".");
        second.push_str(name[0]);
        writeln!(file, "{}", second).unwrap();
        // ex. doe.j
        second = name[1].to_string();
        second.push_str(".");
        second.push_str(&name[0].chars().nth(0).unwrap().to_string());
        writeln!(file, "{}", second).unwrap();
        // john.doe
        first = name[0].to_string();
        first.push_str(".");
        first.push_str(name[1]);
        writeln!(file, "{}", first).unwrap(); 
        // john_doe
        first = name[0].to_string();
        first.push_str("_");
        first.push_str(name[1]);
        writeln!(file, "{}", first).unwrap(); 
        // ex. j_doe
        first = name[0].chars().nth(0).unwrap().to_string();
        first.push_str("_");
        first.push_str(name[1]);
        writeln!(file, "{}", first).unwrap();
        // ex. doe_j
        second = name[1].to_string();
        second.push_str("_");
        second.push_str(&name[0].chars().nth(0).unwrap().to_string());
        writeln!(file, "{}", second).unwrap();
    }

}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3{
        println!("[ + ] Please add a user.txt");
        return;
    }
    let txt_file = &args[2];
    // open file, read line by line and create a wordlist
    let f: Vec<String> = fs::read_to_string(txt_file)
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

    genwordlist(f);
    
}
