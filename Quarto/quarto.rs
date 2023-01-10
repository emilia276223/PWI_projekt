#![allow(unused)]
#![allow(non_snake_case)]

use rand::Rng;
use std::io;

//quarto

fn bot(mut tab: [[i32; 4]; 4], element: i32) -> [[i32; 4]; 4] {
    loop {
        let a = rand::thread_rng().gen_range(0..=3);
        let b = rand::thread_rng().gen_range(0..=3);
        if tab[a][b] != 0 {
            continue;
        } else {
            tab[a][b] = element; //tu wstaw element
            return tab;
        }
    }
}

fn main() {
    let tab: [[i32; 4]; 4] = [[0; 4]; 4]; //utworzenie tablicy w ktorej bedzie zapisywany stan gry
                                          //jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu
}
