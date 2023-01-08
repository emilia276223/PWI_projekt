// quarto
use rand::Rng; // 0.8.5

fn random_item() -> i32 { 
    let mut rng = rand::thread_rng();

	  let feature = [2,3,5,7]; // mod wlasnosci
	  let mut value = 1; // finalna wartosc wlasnosci
	  for i in 0..4 {
	      let toss: bool = rng.gen_bool(0.5); // losowanie true lub false z takim samym prawdopodobienstwem
//      println!("Wlasciwosc: {}", toss);
        if toss == true { // jesli true dodajemy elementowi wlasnosc
            value *= feature[i]
        }
	  }
	  return value
}

fn main() {
//  let tab: [[i32;4];4] = [[0;4];4];//utworzenie tablicy w ktorej bedzie zapisywany stan gry
//  jesli 0 to puste pole, w innym przypadkach odpowiednio 1*(2)*(3)*(5)*(7) zaleznie od cech elementu 
    println!("Wartosc: {}", random_item());
}
