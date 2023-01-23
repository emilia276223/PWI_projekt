#![allow(unused)]
#![allow(non_snake_case)]
extern crate ncurses;//nie wiem jeszcze czy dziala
use ncurses::*;
use std::io;
use rand::Rng; // 0.8.5

const BOK_PLANSZY: usize = 4;
const ROZMIAR_PLANSZY: usize = 16;
const ZNAKI_POZIOM: usize = 19;
const ZNAKI_PION: usize = 10;
const ZNAK_A: i32 = 97; // 'ASCII a'
const GRA_Z_GRACZEM: i32 = 1;
const GRA_Z_BOTEM: i32 = 2;

const DUZE_KOLO_Z_DZIURKA: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const DUZE_KOLO: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const MALE_KOLO_Z_DZIURKA: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const MALE_KOLO: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const DUZY_KWADRAT_Z_DZIURKA: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const DUZY_KWADRAT: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const MALY_KWADRAT_Z_DZIURKA: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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

const MALY_KWADRAT: [[char; ZNAKI_POZIOM]; ZNAKI_PION] = [
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


const MIEJSCA_NA_INDEKSY : [[i32;2]; ROZMIAR_PLANSZY] = [
	[0, 100], [0, 120], [0, 140], [0, 160],
	[12, 100], [12, 120], [12, 140], [12, 160],
	[24, 100], [24, 120], [24, 140], [24, 160],
	[36, 100], [36, 120], [36, 140], [36, 160]
];

const POLA_NA_PLANSZY : [[i32;2]; ROZMIAR_PLANSZY] = [
	[0, 0], [0, 20], [0, 40], [0, 60],
	[12, 0], [12, 20], [12, 40], [12, 60],
	[24, 0], [24, 20], [24, 40], [24, 60],
	[36, 0], [36, 20], [36, 40], [36, 60]
];

const NUMERY_ELEMENTOW : [i32; ROZMIAR_PLANSZY] = [210, 105, 70, 35, 30, 15, 10, 5, 42, 21, 14, 7, 6, 3, 2, 1];

fn ustawienie_kolorow() {
	initscr();
	start_color();
	curs_set(CURSOR_VISIBILITY :: CURSOR_INVISIBLE);
	noecho();
	init_pair(1, COLOR_RED, COLOR_BLACK);
  init_pair(2, COLOR_BLUE, COLOR_BLACK);
  init_pair(3, COLOR_BLACK, COLOR_WHITE);
}

fn print_end(player: i32) {
	// clear();
  mvaddstr(53,100, "GRA ZAKONCZONA");
  if(player == -1){
	mvaddstr(53, 116, "REMIS");
	return;
  } 
  else{
  mvaddstr(53, 116, "WYGRAL GRACZ");
  mvaddch(53, 130, (player + '1' as i32 ) as u32);
  }
}

fn check_if_end_test() {
	//wczytanie tablicy do tesu
	let test1 = [[0,6,0,0],[0,3,0,0],[0,21,0,0],[0,210,0,0]];//4 w jednej kolumnie
	println!("test 1: {}", check_if_end(&test1));// gdy gracz = 0

	let test2 = [[5,10,1,7],[0,30,0,0],[0,105,0,0],[0,0,0,14]];//4 w jednym rzedzie
	println!("test2: {}", check_if_end(&test2));// gdy gracz = 0

	let test3 = [[15,5,35,6],[105,42,2,14],[1,0,0,0],[30,0,0,0]];//brak
	println!("test 3: {}", check_if_end(&test3));// gdy gracz = 0

	let test4 = [[0,1,0,70],[0,0,105,0],[0,30,2,15],[5,0,0,6]];//4 na skosie "/"
	println!("test 4: {}", check_if_end(&test4));// gdy gracz = 0

	let test5 = [[10,105,0,0],[21,30,2,5],[0,0,6,0],[0,0,0,70]];//4 na skosie "\"
	println!("test 5: {}", check_if_end(&test5));// gdy gracz = 0
}

fn check_if_end(tab: &[[i32; BOK_PLANSZY]; BOK_PLANSZY]) -> bool {//true  - koniec

	//sprawdzam czy sa wolne miejsca
	let mut sumaWolnych = 0;
	for rzad in 0..BOK_PLANSZY{
		for kolumna in 0..BOK_PLANSZY{
			if tab[rzad][kolumna] == 0 {//
				sumaWolnych += 1;//licze sume wolnych miejsc
			}
		}
	}

	if sumaWolnych == 0 {//brak wolnych miejsc
		return true;
	}

	//skoro sa jeszcze wolnie miejsca sprawdzam czy nie ma 4 takich samych w rzedzie / kolumnie / na skosach

	let typy = [2, 3, 5, 7];//mod wlasciwosci
	let mut licznik = 0;
	let mut wolneElementy = 0;

	//sprawdzam czy sa 4 w rzedzie

	for typ in typy {//dla kazdej wlasciwosci
		for r in 0..BOK_PLANSZY { //dla kazdego rzedu
			//sprawdzenie wlasnosci dla wszystkich elementow
			for k in 0..BOK_PLANSZY {
				if tab[r][k] % typ == 0{//jesli jest spelnione 
					licznik += 1;
				}
				if tab[r][k] == 0{//jesli jest puste
					wolneElementy += 1;
				}
			}
			if wolneElementy == 0 {//jesli wszystkie uzupelnione
				if licznik == 0 || licznik == BOK_PLANSZY {//jesli wszystkie takie same
					return true;
				}
			}
			licznik = 0;
			wolneElementy = 0;
		}
	}

	//4 w kolumnie

	for typ in typy {//dla kazdej wlasciwosci
		for k in 0..BOK_PLANSZY { //dla kazdej kolumny
			//sprawdzenie wlasnosci dla wszystkich elementow
			for r in 0..BOK_PLANSZY {
				if tab[r][k] % typ == 0 {//jesli jest spelnione 
					licznik += 1;
				}
				if tab[r][k] == 0 {//jesli jest puste
					wolneElementy += 1;
				}
			}
			if wolneElementy == 0 {//jesli wszystkie uzupelnione
				if licznik == 0 || licznik == 4 {//jesli wszystkie takie same
					return true;
				}
			}
		licznik = 0;
		wolneElementy = 0;
		}
	}

	//sprawdzenie dla skosu "\"
	for typ in typy {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for p in 0..BOK_PLANSZY{
			if tab[p][p] % typ == 0 {//jesli ma wlasnosc f
				licznik += 1
			}
			if tab[p][p] == 0 {//jesli jest pusty
				wolneElementy += 1
			}
		}
		if wolneElementy == 0 {//jesli wszystkie uzupelnione
			if licznik == 0 || licznik == BOK_PLANSZY {//jesli wszystkie takie same
				return true;
			}
		}
		licznik = 0;
		wolneElementy = 0;
	}

	//sprawdzenie dla skosu "/"
	for typ in typy {//dla kazdej wlasciwosci
		//dla kazdej wartosci na skosie
		for p in 0..BOK_PLANSZY{
			if tab[p][BOK_PLANSZY - 1 - p] % typ == 0 {//jesli ma wlasnosc f
				licznik += 1
			}
			if tab[p][BOK_PLANSZY - 1 - p] == 0 {//jesli jest pusty
				wolneElementy += 1
			}
		}
		if wolneElementy == 0{//jesli wszystkie uzupelnione
			if licznik == 0 || licznik == BOK_PLANSZY {//jesli wszystkie takie same
				return true;
			}
		}
		licznik = 0;
		wolneElementy = 0;
	}	

	return false;
}

fn start() -> [[[i32; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY]//sprawdzic czy zadziala
{
  let mut tab : [[[i32; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY] = [[[0; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY];
  for i in 0..ZNAKI_PION {
		for j in 0..ZNAKI_POZIOM {
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
fn wstaw_element(n : usize, x : usize, ustawienie_w_tablicy: &mut [[i32;2]; ROZMIAR_PLANSZY],
	tablica_bot : &mut [[i32; BOK_PLANSZY]; BOK_PLANSZY]) {
	// n - numer elementu, x - numer pola; wstawiamy n- ty element w x - te miejsce

  //do wyswietlania
  ustawienie_w_tablicy[n][0] = POLA_NA_PLANSZY[x][0];
	ustawienie_w_tablicy[n][1] = POLA_NA_PLANSZY[x][1];

  //dla komputera
  tablica_bot[x / BOK_PLANSZY][x % BOK_PLANSZY] = NUMERY_ELEMENTOW[n];
}

//nic nie zmieniamy
fn draw_board(ustawienie_w_tablicy : &[[i32;2]; ROZMIAR_PLANSZY], tab: [[[i32; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY]) {
	erase();
	//napisanie pol
	for k in 0..ROZMIAR_PLANSZY {
		mvaddch(MIEJSCA_NA_INDEKSY[k][0], MIEJSCA_NA_INDEKSY[k][1] + (ZNAKI_PION as i32), /*liczba[k + 1]*/ (k + ZNAK_A as usize) as u32);
		mvaddch(POLA_NA_PLANSZY[k][0], POLA_NA_PLANSZY[k][1] + (ZNAKI_PION as i32), /*liczba[k + 1]*/ (k + ZNAK_A as usize) as u32);
	}

	//na czerwono
	attron(COLOR_PAIR(1));
	for k in 0..8 {
		for i in 0..ZNAKI_PION {
			for j in 0..ZNAKI_POZIOM {
				mvaddch(ustawienie_w_tablicy[k][0] + (i + 1) as i32, ustawienie_w_tablicy[k][1] + (j + 1) as i32, tab[k][i][j] as u32);
			}
		}
	}
	attroff(COLOR_PAIR(1));

	//na niebiesko
	attron(COLOR_PAIR(2));
	for k in 8..16 {
		for i in 0..ZNAKI_PION
		{
			for j in 0..ZNAKI_POZIOM
			{
				mvaddch(ustawienie_w_tablicy[k][0] + (i + 1) as i32, ustawienie_w_tablicy[k][1] + (j + 1) as i32, tab[k][i][j] as u32);
			}
		}
	}
	attroff(COLOR_PAIR(2));
}

//zmienione
fn umiesc_element(n : usize, ustawienie_w_tablicy : &mut [[i32;2]; ROZMIAR_PLANSZY], czy_wpisane : &mut [i32; ROZMIAR_PLANSZY],
	tab : [[[i32; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY], tablica_bot : &mut [[i32; BOK_PLANSZY]; BOK_PLANSZY], player: i32) {//n - wybrany element
	//podswietlenie wybranego elementu
	attron(COLOR_PAIR(3));
	mvaddch(MIEJSCA_NA_INDEKSY[n][0], MIEJSCA_NA_INDEKSY[n][1] + (ZNAKI_PION as i32), /*liczba[k + 1]*/ (n + ZNAK_A as usize) as u32);
	attroff(COLOR_PAIR(3));

	//wybranie miejsca
	mvaddstr(49, 0, "Runda gracza ");
  	mvaddch(49, 13, (player + '1' as i32 ) as u32);
	mvaddstr(50,0,"Podaj, gdzie chcesz ustawic ten element");
	let mut znak = 0;
	znak = getch();
	let mut x = znak - ZNAK_A as i32;
	if x >= 0 && x < ROZMIAR_PLANSZY as i32 && czy_wpisane[x as usize] == 0
    {
		czy_wpisane[x as usize] = 1;
		wstaw_element(n,x as usize, ustawienie_w_tablicy, tablica_bot);//czy nie musi byc &mut
	}
	else{
		mvaddstr(51,0,"Tam juz jest wpisany inny element lub zle podane miejsce");
		umiesc_element(n, ustawienie_w_tablicy, czy_wpisane, tab, tablica_bot, player);
	}
    // ustawienie_w_tablicy[0][0] = 3;
	draw_board(&ustawienie_w_tablicy, tab);
}

//nic sie nie zmienia
fn wczytaj_element(czy_uzyty : &mut [i32; ROZMIAR_PLANSZY], player: i32) -> i32 {//wybranie przez gracza jaki chce element 
	//ustawienie jednego elementu
	mvaddstr(49, 0, "Runda gracza ");
  	mvaddch(49, 13, (player + '1' as i32 ) as u32);
	mvaddstr(50,0,"Podaj jaki element chcesz wybrac");
	let znak = getch();
	//ogarnac potem zeby sie wyswietlalo
	let n = znak - ZNAK_A as i32;
	if n >= 0 && n < ROZMIAR_PLANSZY as i32 && czy_uzyty[n as usize] == 0{
		czy_uzyty[n as usize] = 1;
		return n as i32;
	}
	mvaddstr(52,0,"zle wybrany element");
	return wczytaj_element(czy_uzyty, player);
}

//bot wybiera gdzie wstawic element
fn bot(tablica_bot: &mut [[i32; BOK_PLANSZY]; BOK_PLANSZY], element: i32, ustawienie_w_tablicy: &mut [[i32;2]; ROZMIAR_PLANSZY], czy_wpisane: &mut [i32; ROZMIAR_PLANSZY]) {
	// sprawdzenie czy bot ma ruch konczacy
	let mut a: i32 = -1;
	let mut tablica_test : [[i32; BOK_PLANSZY]; BOK_PLANSZY] = [[0; BOK_PLANSZY]; BOK_PLANSZY];

	for i in 0..ROZMIAR_PLANSZY {
		tablica_test[i/4][i%4] = tablica_bot[i/4][i%4];
	}

	for i in 0..ROZMIAR_PLANSZY {
		if czy_wpisane[i as usize] == 0 {
			tablica_test[i/4][i%4] = element;
			if (check_if_end(&tablica_test)) {
				a = i as i32;
				break;
			} else {
				tablica_test[i/4][i%4] = 0;
			}
		}
	}
	// w przeciwnym wypadku losujemy pole
	if a == -1 {
		loop {
  	  a = rand::thread_rng().gen_range(0..=(ROZMIAR_PLANSZY as i32)-1);
    	if czy_wpisane[a as usize] == 0 {
      	break;
    	} 
  	}
	}
	wstaw_element(element as usize, a as usize, ustawienie_w_tablicy, tablica_bot);
	czy_wpisane[a as usize] = 1;
}

fn random_item(czy_uzyty : &mut [i32; ROZMIAR_PLANSZY]) -> i32 { 
  loop{
    let a = rand::thread_rng().gen_range(0..=ROZMIAR_PLANSZY-1);
    if czy_uzyty[a] == 0
    {
      czy_uzyty[a] = 1;
      return a as i32;
    }
  }
}

fn main() {
  //ustawienia poczatkowe
  let mut czy_uzyty : [i32; ROZMIAR_PLANSZY] = [0; ROZMIAR_PLANSZY];
  let mut czy_wpisane : [i32; ROZMIAR_PLANSZY] = [0; ROZMIAR_PLANSZY];
  //gdzie w tablicy jest dany element
  let mut ustawienie_w_tablicy : [[i32;2]; ROZMIAR_PLANSZY] = MIEJSCA_NA_INDEKSY;

  //tablica z tym jak wygladaja elementy
  let mut tab : [[[i32; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY] = [[[0; ZNAKI_POZIOM]; ZNAKI_PION]; ROZMIAR_PLANSZY];
  let mut tablica_bot : [[i32; BOK_PLANSZY]; BOK_PLANSZY] = [[0; BOK_PLANSZY]; BOK_PLANSZY];//odpowiedni numer
  tab = start();

	ustawienie_kolorow();
  // curs_set(1);//tum sie zajmiemy pozniej

  //wybranie czy gra z graczem, czy z botem
  let mut tryb_gry = -1;
  loop
  {
    if tryb_gry == GRA_Z_GRACZEM || tryb_gry == GRA_Z_BOTEM {
      break;
    }
    mvaddstr(0,0,"Podaj, czy chcesz grac w dwie osoby (1) czy z botem (2)");
    tryb_gry = getch() - '0' as i32;
  }

  let mut n;//ktory element w tym momencie wstawiamy
  let mut gracz = 0;//czyja teraz kolej (gdy gra z botem to bot = 0, gacz = 1);

  if tryb_gry == GRA_Z_GRACZEM //z gaczem
  {
    //najpierw wybranie pierwszego elementu
    draw_board(&ustawienie_w_tablicy, tab);
    n = wczytaj_element(&mut czy_uzyty, gracz) as usize;
    refresh();

    gracz = 1;

    //dla kazdego kolejnego gracza: wstawienie i wybranie (i po drodze sprawdzenie czy koniec)
    loop
  	{
	   	draw_board(&ustawienie_w_tablicy, tab);	
	    refresh();
	    umiesc_element(n, &mut ustawienie_w_tablicy, &mut czy_wpisane, tab, &mut tablica_bot, gracz);
       draw_board(&ustawienie_w_tablicy, tab);
      if check_if_end(&tablica_bot) {
		let mut sumaWolnych = 0;
		for rzad in 0..BOK_PLANSZY
		{
			for kolumna in 0..BOK_PLANSZY
			{
				if tablica_bot[rzad][kolumna] == 0 {//
					sumaWolnych += 1;//licze sume wolnych miejsc
				}
			}
		}
	
		if sumaWolnych == 0 {//brak wolnych miejsc
			print_end(-1);
		}
		else {print_end(gracz);}
		break;
			}
	   	n = wczytaj_element(&mut czy_uzyty, gracz) as usize;
	    refresh();
      gracz = (gracz + 1) % 2;
  	}

	let mut z : i32;
	z = getch();
    while(z != '\n' as i32) {z = getch();}
	
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
        if check_if_end(&tablica_bot) {
			let mut sumaWolnych = 0;
			for rzad in 0..BOK_PLANSZY
			{
				for kolumna in 0..BOK_PLANSZY
				{
					if tablica_bot[rzad][kolumna] == 0 {//
						sumaWolnych += 1;//licze sume wolnych miejsc
					}
				}
			}
		
			if sumaWolnych == 0 {//brak wolnych miejsc
				print_end(-1);
			}
			else {print_end(gracz);}
			break;
				}
        n = random_item(&mut czy_uzyty) as usize;
        draw_board(&ustawienie_w_tablicy, tab);
      }
      else
      {
        umiesc_element(n, &mut ustawienie_w_tablicy, &mut czy_wpisane, tab, &mut tablica_bot, gracz);
        draw_board(&ustawienie_w_tablicy, tab);
        if check_if_end(&tablica_bot) {
			let mut sumaWolnych = 0;
			for rzad in 0..BOK_PLANSZY
			{
				for kolumna in 0..BOK_PLANSZY
				{
					if tablica_bot[rzad][kolumna] == 0 {//
						sumaWolnych += 1;//licze sume wolnych miejsc
					}
				}
			}
		
			if sumaWolnych == 0 {//brak wolnych miejsc
				print_end(-1);
			}
			else {print_end(gracz);}
			break;
		}
  	    n = wczytaj_element(&mut czy_uzyty, gracz) as usize;
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
