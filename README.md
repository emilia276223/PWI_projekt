# Quarto w Rust
Gra przypominająca "kółko i krzyżyk" w wrsji 4x4. Plansza składa się z 16 pól, gracze ustawiają na przemian po jednej z figur. Każda z nich posiada cztery charakterystyczne cechy. Wygrywa ten kto ustawi rząd z czterech figur posiadających przynajmniej jedną wspólną cechę.


## Wymagania
Żeby uruchomić projekt potrzebujesz mieć zainstalowane: Rust i Cargo

```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
    
## Uruchomienie lokalne

Sklonuj projekt

```bash
  git clone https://github.com/emilia276223/PWI_projekt.git
```

Przejdź do katalogu projektu

```bash
  cd PWI_projekt
```

Uruchom grę

```bash
  Cargo run
```




## Biblioteka ncurses

Żeby uruchomić projekt potrzebujesz mieć dodatkowo zainstalowaną bibliotekę ncurses

```bash
  sudo apt-get install libncurses5-dev libncursesw5-dev
```
