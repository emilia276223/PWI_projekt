//quarto
fn place_on_board(mut tab: [[i32;4];4], element: i32) -> [[i32; 4]; 4]
{ 
	loop { //pętla w nieskończoność do podania niezajętego miejsca  
        let mut line = String::new(); //wczytywanie user inputu             
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: usize = parts[0].parse().unwrap();
        let b: usize = parts[1].parse().unwrap();
    } 
}

fn main(){
	let tab: [[i32;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
	//jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu 
}
