use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    grid: Vec<Vec<char>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let mut grid = vec![vec!['.'; 10]; 10];
        let word = "rust";

        for (i, letter) in word.chars().enumerate() {
            grid[0][i] = letter;
        }

        Game { grid }
    }

    pub fn get_grid(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}