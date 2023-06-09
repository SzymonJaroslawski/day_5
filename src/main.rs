use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let stos_1 = File::open("./src/stos_1").unwrap();
    let stos_2 = File::open("./src/stos_2").unwrap();
    let stos_3 = File::open("./src/stos_3").unwrap();
    let instrukcje = File::open("./src/instructions").unwrap();

    let kontenery_stos_1 = vec!();
    let kontenery_stos_2 = vec!();
    let kontenery_stos_3 = vec!();
    let instruckje_stos = vec!();

    read_lines_to_stos(kontenery_stos_1, kontenery_stos_2, kontenery_stos_3, &instruckje_stos, stos_1, stos_2, stos_3, instrukcje);

    for lines in instruckje_stos.iter()  {
        
    }
}

fn find_instruction(instrukcje_stos: Vec<String>) -> Vec<i32> {
    let mut instrukcje: Vec<i32> = vec!();
    
    for lines in instrukcje_stos.iter() {
        let words: Vec<_> = lines.split(" ").collect();
        
        let instrukcja_move = words[1].to_string();
        let instrukcja_from = words[3].to_string();
        let instrukcja_to = words[5].to_string();

        instrukcje.push(instrukcja_move.parse::<i32>().unwrap());
        instrukcje.push(instrukcja_from.parse::<i32>().unwrap());
        instrukcje.push(instrukcja_to.parse::<i32>().unwrap());
    }

    instrukcje
}

fn read_lines_to_stos(mut stos1: Vec<String>, mut stos2: Vec<String>, mut stos3: Vec<String>, mut stos4: Vec<String>, f1: File, f2: File, f3: File, f4: File) {
    let stos_1_reader = BufReader::new(f1);
    let stos_2_reader = BufReader::new(f2);
    let stos_3_reader = BufReader::new(f3);

    let instrukcje_reader = BufReader::new(f4);

    for lines in stos_1_reader.lines() {
        let line = lines.unwrap();

        stos1.push(line);
    }

    for lines in stos_2_reader.lines() {
        let line = lines.unwrap();

        stos2.push(line);
    }

    for lines in stos_3_reader.lines() {
        let line = lines.unwrap();

        stos3.push(line);
    }

    for lines in instrukcje_reader.lines() {
        let line = lines.unwrap();
        
        stos4.push(line);
    }

    println!("{:?}", stos1);
    println!("{:?}", stos2);
    println!("{:?}", stos3);
    println!("{:?}", stos4);
}