use std::fs;
fn main() {
    let mut total = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines(){

        //extract game number (only do this if we need it

        let mut game_number_section = line.split(":");
        let mut sections = game_number_section.next().unwrap().split(" ");
        _ = sections.next();
        let game_number = sections.next().unwrap().parse::<i32>().unwrap();
        
        if parse_all_games(game_number_section.next().unwrap()){

            total += game_number;
        }
    }
    
    println!("{total}");
}

fn parse_all_games(line : &str) -> bool {
    for game in line.split(";") {
        for colour in game.split(","){
            let mut sections = colour.split(" ");
            _=sections.next();
            let x = sections.next().unwrap();
            println!("{x}");
            let quantity = x.parse::<i32>().unwrap();
            match sections.next().unwrap()
            {
                "red" => if quantity > 12 {return false;} ,
                "green" =>  if quantity > 13 {return false;} ,
                "blue" =>  if quantity > 14 {return false;} ,
                _ => (),
            }
        }
    }
    return true;
}
