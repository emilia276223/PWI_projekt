//quarto

fn print_end(player: i8){
	println!("Wygral gracz {}", player + 1);
}

fn check_if_end(tab: &[[i8;4];4], player: i8) -> bool{
	
	//sprawdzam czy sa wolne miejsca
	let free = 0;//co jest jesli miejsce jest wolne
	let mut sum = 0;
	for i in 0..4{
		for j in 0..4{
			if tab[i][j] == free{//
				sum += 1;//licze sume wolnych miejsc
			}
		}
	}
	
	if sum == 0{//brak wolnych miejsc
		print_end(player);
		return true;
	}
	
	//skoro sa jeszcze wolnie miejsca sprawdzam czy nie ma 4 takich samych w rzedzie / kolumnie / na skosach
	
	let feature = [2,3,5,7];//mod wlasciwosci
	let mut count = 0;
	let mut free_elements = 0;
	
	//sprawdzam czy sa 4 w kolumnie
	
	for f in feature {//dla kazdej wlasciwosci
		for i in 0..4 { //dla kazdej kolumny
			//sprawdzenie wlasnosci dla wszystkich elementow
			for j in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w kolumnie
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
					print_end(player);
					return true;
				}
			}
		}
		count = 0;
		free_elements = 0;
	}
	
	//4 w rzędzie
	
	for f in feature {//dla kazdej wlasciwosci
		for j in 0..4 { //dla kazdego rzedu
			//sprawdzenie wlasnosci dla wszystkich elementow
			for i in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w rzedzie
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
					print_end(player);
					return true;
				}
			}
		}
		count = 0;
		free_elements = 0;
	}
	
	false
}

fn main(){
	let tab: [[i8;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
	//jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu 
	println!("{}", check_if_end(&tab, 0));
}
