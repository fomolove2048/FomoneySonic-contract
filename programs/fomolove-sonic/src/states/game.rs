use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[account]
pub struct GameAccount {
    pub nft_mint: Pubkey,
    pub board: [[u16; 4]; 4],
    pub score: u32,
    pub top_tile: u16, // Updated to u16
}

impl GameAccount {
    pub const INIT_SPACE: usize = 8 + 32 + 4 * 4 * 2 + 4 + 2; // Updated size calculation

    fn slide_and_merge(row: &mut [u16; 4]) -> (u32, u16) {
        let mut new_row = [0u16; 4];
        let mut pos = 0;
        for &tile in row.iter() {
            if tile != 0 {
                new_row[pos] = tile;
                pos += 1;
            }
        }

        let mut score_increment = 0;
        let mut max_tile = 0;
        for i in 0..3 {
            if new_row[i] == new_row[i + 1] && new_row[i] != 0 {
                if let Some(new_tile) = new_row[i].checked_mul(2) {
                    new_row[i] = new_tile;
                    new_row[i + 1] = 0;
                    score_increment += new_tile as u32;

                    if new_tile > max_tile {
                        max_tile = new_tile;
                    }
                } else {
                    return (score_increment, max_tile);
                }
            }
        }

        let mut final_row = [0u16; 4];
        pos = 0;
        for &tile in new_row.iter() {
            if tile != 0 {
                final_row[pos] = tile;
                pos += 1;
            }
        }

        *row = final_row;

        (score_increment, max_tile)
    }

    pub fn move_left(&mut self) {
        let mut total_score_increment = 0;
        let mut max_tile = 0;
        for row in self.board.iter_mut() {
            let (score_increment, row_max_tile) = Self::slide_and_merge(row);
            total_score_increment += score_increment;
            if row_max_tile > max_tile {
                max_tile = row_max_tile;
            }
        }
        self.score += total_score_increment;
        if max_tile > self.top_tile {
            self.top_tile = max_tile;
        }
    }

    pub fn move_right(&mut self) {
        for row in self.board.iter_mut() {
            row.reverse();
            let (score_increment, max_tile) = Self::slide_and_merge(row);
            self.score += score_increment;
            row.reverse();
            if max_tile > self.top_tile {
                self.top_tile = max_tile;
            }
        }
    }

    pub fn move_up(&mut self) {
        for col in 0..4 {
            let mut column = [0u16; 4];
            for row in 0..4 {
                column[row] = self.board[row][col];
            }
            let (score_increment, max_tile) = Self::slide_and_merge(&mut column);
            self.score += score_increment;
            for row in 0..4 {
                self.board[row][col] = column[row];
            }
            if max_tile > self.top_tile {
                self.top_tile = max_tile;
            }
        }
    }

    pub fn move_down(&mut self) {
        for col in 0..4 {
            let mut column = [0u16; 4];
            for row in 0..4 {
                column[row] = self.board[row][col];
            }
            column.reverse();
            let (score_increment, max_tile) = Self::slide_and_merge(&mut column);
            self.score += score_increment;
            column.reverse();
            for row in 0..4 {
                self.board[row][col] = column[row];
            }
            if max_tile > self.top_tile {
                self.top_tile = max_tile;
            }
        }
    }

    pub fn add_new_tile(&mut self) -> Result<()> {
        let mut empty_tiles = vec![];

        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    empty_tiles.push((i, j));
                }
            }
        }

        if empty_tiles.is_empty() {
            return Ok(());
        }

        // Convert the board to a vector of u8 for hashing
        let board_as_u8: Vec<u8> = self
            .board
            .iter()
            .flat_map(|row| row.iter().flat_map(|&val| val.to_le_bytes()))
            .collect();

        let rand_index = anchor_lang::solana_program::keccak::hash(&board_as_u8).0[0] as usize
            % empty_tiles.len();
        let (x, y) = empty_tiles[rand_index];

        let new_tile =
            if anchor_lang::solana_program::keccak::hash(&[rand_index as u8]).0[0] % 10 == 0 {
                4
            } else {
                2
            };
        self.board[x][y] = new_tile;

        if new_tile > self.top_tile {
            self.top_tile = new_tile;
        }

        Ok(())
    }

    pub fn is_game_over(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    return false;
                }
                if j < 3 && self.board[i][j] == self.board[i][j + 1] {
                    return false;
                }
                if i < 3 && self.board[i][j] == self.board[i + 1][j] {
                    return false;
                }
            }
        }
        true
    }
}
