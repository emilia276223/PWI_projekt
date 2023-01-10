//quarto
//use std::io;

fn print_end(player: i8){
	println!("Wygral gracz {}", player + 1);
}

fn check_if_end_test(){
	//wczytanie tablicy do tesu
	let test1 = [[0,6,0,0],[0,3,0,0],[0,21,0,0],[0,210,0,0]];//4 w jednej kolumnie
	println!("test 1: {}", check_if_end(&test1, 0));// gdy gracz = 0
	
	let test2 = [[5,10,1,7],[0,30,0,0],[0,105,0,0],[0,0,0,14]];//4 w jednym rzedzie
	println!("test2: {}", check_if_end(&test2, 0));// gdy gracz = 0
	
	let test3 = [[15,5,35,6],[105,42,2,14],[1,0,0,0],[30,0,0,0]];//brak
	println!("test 3: {}", check_if_end(&test3, 0));// gdy gracz = 0
	
	let test4 = [[0,1,0,70],[0,0,105,0],[0,30,2,15],[5,0,0,6]];//4 na skosie "/"
	println!("test 4: {}", check_if_end(&test4, 0));// gdy gracz = 0
	
	let test5 = [[10,105,0,0],[21,30,2,5],[0,0,6,0],[0,0,0,70]];//4 na skosie "\"
	println!("test 5: {}", check_if_end(&test5, 0));// gdy gracz = 0
}

fn check_if_end(tab: &[[i32;4];4], player: i8) -> bool{//true - gra toczy 
	
	//sprawdzam czy sa wolne miejsca
	let free = 0;//co jest jesli miejsce jest wolne
	let mut sum = 0;
	for i in 0..4{
		for j in 0..4{
//			print!("{} ", tab[i][j]);
			if tab[i][j] == free{//
				sum += 1;//licze sume wolnych miejsc
			}
		}
//		println!();
	}
	
	if sum == 0{//brak wolnych miejsc
		print_end(player);
		return true;
	}
	
	//skoro sa jeszcze wolnie miejsca sprawdzam czy nie ma 4 takich samych w rzedzie / kolumnie / na skosach
	
	let feature = [2,3,5,7];//mod wlasciwosci
	let mut count = 0;
	let mut free_elements = 0;
	
	//sprawdzam czy sa 4 w rzedzie
	
	for f in feature {//dla kazdej wlasciwosci
//		println!("f = {f}");
		for i in 0..4 { //dla kazdego rzedu
			//sprawdzenie wlasnosci dla wszystkich elementow
			for j in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w tym rzedzie
//			println!("rzad == {i}, count = {count}, free_elements == {free_elements}");
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
//					println!("rzad {}", i + 1);
					print_end(player);
					return true;
				}
			}
			count = 0;
			free_elements = 0;
		}
	}
	
	//4 w kolumnie
	
	for f in feature {//dla kazdej wlasciwosci
//		println!("f = {f}");
		for j in 0..4 { //dla kazdej kolumny
			//sprawdzenie wlasnosci dla wszystkich elementow
			for i in 0..4 {
				if tab[i][j] % f == 0{//jesli jest spelnione 
					count += 1;
				}
				if tab[i][j] == 0{//jesli jest puste
					free_elements += 1;
				}
			}
			
			//sprawdzenie czy sa 4 w tej kolumnie
//			println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
			if free_elements == 0{//jesli wszystkie uzupelnione
				if count == 0 || count == 4{//jesli wszystkie takie same
					//koniec gry
//					println!("kolumna {}", j + 1);
					print_end(player);
					return true;
				}
			}
		count = 0;
		free_elements = 0;
		}
	}
	
	//sprawdzenie dla skosu "\"
	for f in feature {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for i in 0..4{
			if tab[i][i] % f == 0 {//jesli ma wlasnosc f
				count += 1
			}
			if tab[i][i] == 0 {//jesli jest pusty
				free_elements += 1
			}
		}
//		println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
		if free_elements == 0{//jesli wszystkie uzupelnione
			if count == 0 || count == 4{//jesli wszystkie takie same
			//koniec gry
//				println!("skos lewy");
				print_end(player);
				return true;
			}
		}
		count = 0;
		free_elements = 0;
	}
	
	//sprawdzenie dla skosu "/"
		
	for f in feature {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for i in 0..4{
			if tab[i][3 - i] % f == 0 {//jesli ma wlasnosc f
				count += 1
			}
			if tab[i][3 - i] == 0 {//jesli jest pusty
				free_elements += 1
			}
		}
//		println!("kolumna == {j}, count = {count}, free_elements == {free_elements}");
		if free_elements == 0{//jesli wszystkie uzupelnione
			if count == 0 || count == 4{//jesli wszystkie takie same
			//koniec gry
//				println!("skos prawy");
				print_end(player);
				return true;
			}
		}
		count = 0;
		free_elements = 0;
	}	
	
	return false;
}

fn main(){
	let tab: [[i32;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
}
