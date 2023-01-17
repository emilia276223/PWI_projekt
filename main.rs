#![allow(unused)]
#![allow(non_snake_case)]

extern crate ncurses;//nie wiem jeszcze czy dziala
use ncurses::*;
use std::io;
use rand::Rng; // 0.8.5

const DUZE_KOLO_Z_DZIURKA: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', '_', '_', '_', '_', '_', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '.', '-', '\'', ' ', ' ', ' ', ' ', ' ', '\'', '-', '.', ' ', ' ', ' ', ' ',],
  [' ', ' ', '.', '\'', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\'', '.', ' ', ' ',],
  [' ', '/', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\\', ' ',],
  [';', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ';',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'o', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  [':', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ';',],
  [' ', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '/', ' ',],
  [' ', ' ', '\'', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '\'', ' ', ' ',],
  [' ', ' ', ' ', ' ', '\'', '-', '.', '_', '_', '_', '_', '_', '.', '-', '\'', ' ', ' ', ' ', ' ',],
];

const DUZE_KOLO: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', '_', '_', '_', '_', '_', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '.', '-', '\'', ' ', ' ', ' ', ' ', ' ', '\'', '-', '.', ' ', ' ', ' ', ' ',],
  [' ', ' ', '.', '\'', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\'', '.', ' ', ' ',],
  [' ', '/', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\\', ' ',],
  [';', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ';',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  [':', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',' ', ' ', ' ', ';',],
  [' ', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '/', ' '],
  [' ', ' ', '\'', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '\'', ' ', ' '],
  [' ', ' ', ' ', ' ', '\'', '-', '.', '_', '_', '_', '_', '_', '.', '-', '\'', ' ', ' ', ' ', ' ',],
];

const MALE_KOLO_Z_DZIURKA: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', '.', '-', '"', '"', '"', '-', '.', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '.', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '.', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', '/', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\\', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', 'o', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '/', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '\'', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '\'', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', '\'', '-', '-', '-', '-', '-', '\'', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

const MALE_KOLO: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', '.', '-', '"', '"', '"', '-', '.', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '.', '`', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '.', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', '/', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '\\', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '\\', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '/', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '\'', '.', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '.', '\'', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', '\'', '-', '-', '-', '-', '-', '\'', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

const DUZY_KWADRAT_Z_DZIURKA: [[char; 19]; 10] = [
  [' ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', ' ',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'o', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '|',],
];

const DUZY_KWADRAT: [[char; 19]; 10] = [
  [' ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', ' ',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|',],
  ['|', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '|',],
];

const MALY_KWADRAT_Z_DZIURKA: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', 'o', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',]
];

const MALY_KWADRAT: [[char; 19]; 10] = [
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', '|', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '|', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',],
  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',]
];

const MIEJSCA_NA_INDEKSY : [[i32;2];16] = [[0, 100], [0, 120], [0, 140], [0, 160],
[12, 100], [12, 120], [12, 140], [12, 160],
[24, 100], [24, 120], [24, 140], [24, 160],
[36, 100], [36, 120], [36, 140], [36, 160]
];

const POLA_NA_PLANSZY : [[i32;2];16] = [[0, 0], [0, 20], [0, 40], [0, 60],
[12, 0], [12, 20], [12, 40], [12, 60],
[24, 0], [24, 20], [24, 40], [24, 60],
[36, 0], [36, 20], [36, 40], [36, 60]
];

const NUMERY_ELEMENTOW : [i32;16] = [210, 105, 70, 35, 30, 15, 10, 5, 42, 21, 14, 7, 6, 3, 2, 1];

fn ustawienie_kolorow(){
	initscr();
	start_color();
	init_pair(1, COLOR_RED, COLOR_BLACK);
    init_pair(2, COLOR_BLUE, COLOR_BLACK);
    init_pair(3, COLOR_BLACK, COLOR_WHITE);
}

fn print_end(player: i32){
	// clear();
    mvaddstr(53,100, "GRA ZAKONCZONA");
    mvaddstr(53, 116, "WYGRAL GRACZ");
    mvaddch(53, 130, (player + '1' as i32 ) as u32);
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

fn check_if_end(tab: &[[i32;4];4], player: i32) -> bool{//true  - koniec
	
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
		// println!();
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

fn start() -> [[[i32;19];10];16]//sprawdzic czy zadziala
{
    let mut tab : [[[i32;19];10];16] = [[[0;19];10];16];
    for i in 0..10
    {
		for j in 0..19
        {
			tab[0][i][j] = DUZY_KWADRAT_Z_DZIURKA[i][j] as i32;
			tab[1][i][j] = DUZE_KOLO_Z_DZIURKA[i][j] as i32;
			tab[2][i][j] = MALY_KWADRAT_Z_DZIURKA[i][j] as i32;
			tab[3][i][j] = MALE_KOLO_Z_DZIURKA[i][j] as i32;
			tab[4][i][j] = DUZY_KWADRAT[i][j] as i32;
			tab[5][i][j] = DUZE_KOLO[i][j] as i32;
			tab[6][i][j] = MALY_KWADRAT[i][j] as i32;
			tab[7][i][j] = MALE_KOLO[i][j] as i32;
			tab[8][i][j] = DUZY_KWADRAT_Z_DZIURKA[i][j] as i32;
			tab[9][i][j] = DUZE_KOLO_Z_DZIURKA[i][j] as i32;
			tab[10][i][j] = MALY_KWADRAT_Z_DZIURKA[i][j] as i32;
			tab[11][i][j] = MALE_KOLO_Z_DZIURKA[i][j] as i32;
			tab[12][i][j] = DUZY_KWADRAT[i][j] as i32;
			tab[13][i][j] = DUZE_KOLO[i][j] as i32;
			tab[14][i][j] = MALY_KWADRAT[i][j] as i32;
			tab[15][i][j] = MALE_KOLO[i][j] as i32;
		}
	}
    return tab;
}

//zmienione
fn wstaw_element(n : usize, x : usize, ustawienie_w_tablicy: &mut [[i32;2];16], tablica_bot : &mut [[i32;4];4]){//n - numer elementu, x - numer pola
    //wstawiamy n- ty element w x - te miejsce
    
    //do wyswietlania
    ustawienie_w_tablicy[n][0] = POLA_NA_PLANSZY[x][0];
	ustawienie_w_tablicy[n][1] = POLA_NA_PLANSZY[x][1];
    
    //dla komputera
    tablica_bot[x / 4][x % 4] = NUMERY_ELEMENTOW[n];
}

//nic nie zmieniamy
fn draw_board(ustawienie_w_tablicy : &[[i32;2];16], tab: [[[i32;19];10];16]){
	clear();
	//napisanie pol
    let mut k2 = 97;
	for k in 0..16{
		mvaddch(MIEJSCA_NA_INDEKSY[k][0], MIEJSCA_NA_INDEKSY[k][1] + 10, /*liczba[k + 1]*/ (k + k2) as u32);
		mvaddch(POLA_NA_PLANSZY[k][0], POLA_NA_PLANSZY[k][1] + 10, /*liczba[k + 1]*/ (k + k2) as u32);
	}
	
	//na czerwono
	attron(COLOR_PAIR(1));
	for k in 0..8{
		for i in 0..10{
			for j in 0..19{
				mvaddch(ustawienie_w_tablicy[k][0] + (i + 1) as i32, ustawienie_w_tablicy[k][1] + (j + 1) as i32, tab[k][i][j] as u32);
			}
		}
	}
	attroff(COLOR_PAIR(1));

	//na niebiesko
	attron(COLOR_PAIR(2));
	for k in 8..16{
		for i in 0..10
		{
			for j in 0..19
			{
				mvaddch(ustawienie_w_tablicy[k][0] + (i + 1) as i32, ustawienie_w_tablicy[k][1] + (j + 1) as i32, tab[k][i][j] as u32);
			}
		}
	}
	attroff(COLOR_PAIR(2));
}

//zmienione
fn umiesc_element(n : usize, ustawienie_w_tablicy : &mut [[i32;2];16], czy_wpisane : &mut [i32;16], tab : [[[i32;19];10];16], tablica_bot : &mut [[i32;4];4]){//n - wybrany element
	//podswietlenie wybranego elementu
	attron(COLOR_PAIR(3));
	mvaddch(MIEJSCA_NA_INDEKSY[n][0], MIEJSCA_NA_INDEKSY[n][1] + 10, /*liczba[k + 1]*/ (n + 97) as u32);
	attroff(COLOR_PAIR(3));
	
	//wybranie miejsca
	mvaddstr(50,0,"Podaj, gdzie chcesz ustawic ten element");
	let mut a = 0;
    a = getch();
	let mut x = a - 97 as i32;
	if x >= 0 && x < 16 && czy_wpisane[x as usize] == 0
    {
		czy_wpisane[x as usize] = 1;
		wstaw_element(n,x as usize, ustawienie_w_tablicy, tablica_bot);//czy nie musi byc &mut
	}
	else{
		mvaddstr(51,0,"Tam juz jest wpisany inny element lub zle podane miejsce");
		umiesc_element(n, ustawienie_w_tablicy, czy_wpisane, tab, tablica_bot);
	}
    // ustawienie_w_tablicy[0][0] = 3;
	draw_board(&ustawienie_w_tablicy, tab);
}

//nic sie nie zmienia
fn wczytaj_element(czy_uzyty : &mut [i32;16]) -> i32{//wybranie przez gracza jaki chce element 
	//ustawienie jednego elementu
	let b = getch();
	//ogarnac potem zeby sie wyswietlalo
	let n = b - 97 as i32;
	if n >= 0 && n < 16 && czy_uzyty[n as usize] == 0{
		czy_uzyty[n as usize] = 1;
		return n as i32;
	}
	mvaddstr(52,0,"zle wybrany element");
	return wczytaj_element(czy_uzyty);
}

//bot wybiera gdzie wstawic element
fn bot(tablica_bot: &mut [[i32; 4]; 4], element: i32, ustawienie_w_tablicy: &mut [[i32;2];16], czy_wpisane: &mut [i32;16]){
    loop {
        let a = rand::thread_rng().gen_range(0..=15);
        // let b = rand::thread_rng().gen_range(0..=3);
        // if tab[a][b] != 0
        if czy_wpisane[a as usize] == 0 
        {
            wstaw_element(element as usize, a, ustawienie_w_tablicy, tablica_bot);
            czy_wpisane[a as usize] = 1;
            return;
        } 
    }
}

fn random_item(czy_uzyty : &mut [i32;16]) -> i32 { 
    loop{
        let a = rand::thread_rng().gen_range(0..=15);
        if czy_uzyty[a] == 0
        {
            czy_uzyty[a] = 1;
            return a as i32;
        }
    }
    
}

fn main()
{
    //ustawienia poczatkowe
    let mut czy_uzyty : [i32;16] = [0;16];
    let mut czy_wpisane : [i32;16] = [0;16];
    //gdzie w tablicy jest dany element
    let mut ustawienie_w_tablicy : [[i32;2];16] = MIEJSCA_NA_INDEKSY;

    //tablica z tym jak wygladaja elementy
    let mut tab : [[[i32;19];10];16] = [[[0;19];10];16];
    let mut tablica_bot : [[i32; 4]; 4] = [[0; 4]; 4];//odpowiedni numer
    tab = start();

	ustawienie_kolorow();
    // curs_set(1);//tum sie zajmiemy pozniej
	
    //wybranie czy gra z graczem, czy z botem
    let mut tryb_gry = -1;
    loop
    {
        if tryb_gry == 1 || tryb_gry == 2 {
            break;
        }
        mvaddstr(0,0,"Podaj, czy chcesz grac w dwie osoby (1) czy z botem (2)");
        tryb_gry = getch() - '0' as i32;
    }

    let mut n;//ktory element w tym momencie wstawiamy
    let mut gracz = 0;//czyja teraz kolej (gdy gra z botem to bot = 0, gacz = 1);

    
    if tryb_gry == 1 //z gaczem
    {
        //najpierw wybranie pierwszego elementu
        draw_board(&ustawienie_w_tablicy, tab);
        n = wczytaj_element(&mut czy_uzyty) as usize;
        refresh();

        gracz = 1;

        //dla kazdego kolejnego gracza: wstawienie i wybranie (i po drodze sprawdzenie czy koniec)
        loop
    	{
	    	draw_board(&ustawienie_w_tablicy, tab);	
		    refresh();
		    umiesc_element(n, &mut ustawienie_w_tablicy, &mut czy_wpisane, tab, &mut tablica_bot);
            draw_board(&ustawienie_w_tablicy, tab);
            if check_if_end(&tablica_bot, gracz) {break;}
	    	n = wczytaj_element(&mut czy_uzyty) as usize;
		    refresh();
            gracz = (gracz + 1) % 2;
    	}

        getch();
	    endwin();
        return;
    }

    else//gra z botem
    {
        //najpierw wybranie pierwszego elementu (to robi bot)
        draw_board(&ustawienie_w_tablicy, tab);
        n = random_item(&mut czy_uzyty) as usize;
        refresh();

        gracz = 1;
        //dla kazdego kolejnego gracza: wstawienie i wybranie (i po drodze sprawdzenie czy koniec)
        loop
    	{
	    	draw_board(&ustawienie_w_tablicy, tab);	
		    refresh();
            if gracz == 0
            {
                bot(&mut tablica_bot, n as i32, &mut ustawienie_w_tablicy, &mut czy_wpisane);
                draw_board(&ustawienie_w_tablicy, tab);
                if check_if_end(&tablica_bot, gracz) {break;}
                n = random_item(&mut czy_uzyty) as usize;
                draw_board(&ustawienie_w_tablicy, tab);
            }
            else
            {
                umiesc_element(n, &mut ustawienie_w_tablicy, &mut czy_wpisane, tab, &mut tablica_bot);
                draw_board(&ustawienie_w_tablicy, tab);
                if check_if_end(&tablica_bot, gracz) {break;}
	    	    n = wczytaj_element(&mut czy_uzyty) as usize;
                draw_board(&ustawienie_w_tablicy, tab);
		        refresh();
            }
            gracz = (gracz + 1) % 2;
    	}


	    getch();
	    endwin();
        return;
    }
}