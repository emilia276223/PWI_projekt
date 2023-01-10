//quarto
use std::io;

fn test(){
	let test1 = [[0,6,0,0],[0,3,0,0],[0,21,0,0],[0,210,0,0]]; //przykładowa tablica do testów
	println!("{:?}",place_on_board(test1, 12));
}

fn place_on_board(mut tab: [[i32;4];4], element: i32) -> [[i32; 4]; 4]
{ 
	loop { //pętla w nieskończoność do podania niezajętego miejsca  
        let mut line = String::new(); //wczytywanie user inputu             
        io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: usize = parts[0].parse().unwrap();
        let b: usize = parts[1].parse().unwrap();
		if tab[a][b]!=0{               
            println!("Wybrane miejsce jest zajęte!");
        }
        else { //znalezienie poprawnego miejsca
            tab[a][b]=element;
            return tab;
        }
    } 
}

fn main(){
	let tab: [[i32;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
	//jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu 
	test();
}
