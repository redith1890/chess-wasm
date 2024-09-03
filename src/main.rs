
use std::panic;

use macroquad::prelude::*;
use macroquad::window::Conf;

pub struct App {
    turn: Turn,
    grid: Grid,
    selected_piece: Option<(usize, usize)>,
    log: Vec<([usize;2], [usize;2])>
}
impl App {
    pub fn new() -> Self {
        App {
            turn: Turn::White,
            grid: Grid::new(),
            selected_piece: None,
            log: Vec::new()
        }
    }

    pub fn move_piece(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        cell_size: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        let col = ((mouse_x - offset_x) / cell_size) as usize;
        let row = 7 - ((mouse_y - offset_y) / cell_size) as usize;

        if col < 8 && row < 8 {
            match self.selected_piece {
                None => {
                    if self.grid.cells[col][row].piece.is_some() {
                        self.selected_piece = Some((col, row));
                    }
                }
                Some((from_col, from_row)) => {
                    if self.is_move_castling([from_col, from_row], [col, row]) {
                        // Mover el rey
                        let king = self.grid.cells[from_col][from_row].piece.take();
                        self.grid.cells[col][row].piece = king;
    
                        // Mover la torre
                        let (rook_from_col, rook_to_col) = if col == 6 {
                            (7, 5) // Enroque corto
                        } else {
                            (0, 2) // Enroque largo
                        };
                        let rook = self.grid.cells[rook_from_col][row].piece.take();
                        self.grid.cells[rook_to_col][row].piece = rook;
    
                        // Registrar el movimiento del rey
                        self.add_log(([from_col, from_row], [col, row]));
                        // Registrar el movimiento de la torre
                        self.add_log(([rook_from_col, row], [rook_to_col, row]));
                    } 
                        else if self.grid.is_move_legal([from_col, from_row], [col, row]) {
                        let piece = self.grid.cells[from_col][from_row].piece.take();
                        self.grid.cells[col][row].piece = piece;
                        self.selected_piece = None;
                        self.add_log(([from_col, from_row], [col, row]));
                    }
                    
                    self.selected_piece = None;
                }
            }
        }
    }
    pub fn add_log(&mut self, position: ([usize;2],[usize;2])){
        self.log.push(position);
    }
    
    

    pub fn draw(
        &self,
        textures: &std::collections::HashMap<Piece, Texture2D>,
        cell_size: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        for i in 0..=7 {
            for j in 0..=7 {
                let x = offset_x + cell_size * i as f32;
                let y = offset_y + cell_size * j as f32;

                if (i + j) % 2 == 0 {
                    draw_rectangle(x, y, cell_size, cell_size, WHITE);
                } else {
                    draw_rectangle(x, y, cell_size, cell_size, GRAY);
                }
            }
        }

        for i in 0..=7 {
            for j in 0..=7 {
                let cell = &self.grid.cells[i][j];
                if let Some(piece) = cell.piece {
                    if let Some(texture) = textures.get(&piece) {
                        let x = offset_x + cell_size * i as f32;
                        let y = offset_y + cell_size * (7 - j) as f32;
                        draw_texture_ex(
                            texture,
                            x,
                            y,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(cell_size, cell_size)),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }

        if let Some((col, row)) = self.selected_piece {
            let x = offset_x + cell_size * col as f32;
            let y = offset_y + cell_size * (7 - row) as f32;
            draw_rectangle_lines(x, y, cell_size, cell_size, 3.0, YELLOW);
        }
    }
    pub fn has_king_moved(&self, color: ChessColor) -> bool {
        let king_start_row = if color == ChessColor::White { 0 } else { 7 };
        for &(from, _) in &self.log {
            if from == [4, king_start_row] {
                return true;
            }
        }
        false
    }
    pub fn has_rook_moved(&self, color: ChessColor, is_kingside: bool) -> bool {
        let rook_start_col = if is_kingside { 7 } else { 0 };
        let rook_start_row = if color == ChessColor::White { 0 } else { 7 };
    
        for &(from, _) in &self.log {
            if from == [rook_start_col, rook_start_row] {
                return true;
            }
        }
        false
    }
    pub fn is_move_castling(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        if let Some(piece) = self.grid.find_cell(from_position).piece {
            if piece.type_of_piece == TypePiece::King {
                let (king_start_col, king_end_col, rook_start_col) = match piece.color {
                    ChessColor::White => (4, if to_position[0] == 6 { 6 } else { 2 }, if to_position[0] == 6 { 7 } else { 0 }),
                    ChessColor::Black => (4, if to_position[0] == 6 { 6 } else { 2 }, if to_position[0] == 6 { 7 } else { 0 }),
                };
                let king_row = if piece.color == ChessColor::White { 0 } else { 7 };
    
                if self.has_king_moved(piece.color) {
                    return false;
                }
    
                if self.has_rook_moved(piece.color, to_position[0] == 6) {
                    return false;
                }
    
                if !self.grid.is_path_clear([king_start_col, king_row], [rook_start_col, king_row]) {
                    return false;
                }
    
                let attacking_color = piece.color.opposite();
                let step: i32 = if king_end_col > king_start_col { 1 } else { -1 };
                for col in (std::cmp::min(king_start_col, king_end_col)..=std::cmp::max(king_start_col, king_end_col)).step_by(step.abs() as usize) {
                    if self.grid.is_square_attacked([col, king_row], attacking_color) {
                        return false;
                    }
                }
    
                return true;
            }
        }
        false
    }
}
pub enum Turn {
    White,
    Black,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypePiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChessColor {
    White,
    Black,
}
impl ChessColor {
    pub fn opposite(&self) -> ChessColor {
        match self {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    type_of_piece: TypePiece,
    color: ChessColor,
}
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    position: [usize; 2],
    piece: Option<Piece>,
}

#[derive(Debug, Clone)]
pub struct Grid {
    cells: [[Cell; 8]; 8],
}

impl Grid {
    // pub fn is_move_castling(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
    //     if self.find_cell(from_position).piece.is_some(){
    //         if self.find_cell(from_position).piece.unwrap().type_of_piece == TypePiece::King{
    //             match self.find_cell(from_position).piece.unwrap().color {
    //                 ChessColor::Black => if to_position == [6,7] || to_position == [2,7]{return true;}  else{return false;}
    //                 ChessColor::White => if to_position == [6,0] || to_position == [2,0]{return true;}  else{return false;}
    //             }
    //         }
    //         else {
    //             false
    //         }
    //     }
    //     else {
    //         false
    //     }
    // }
    
    pub fn is_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        if {
            match self.find_cell(from_position).piece.unwrap().type_of_piece {
                TypePiece::Pawn => self.is_pawn_move_legal(from_position, to_position),
                TypePiece::Bishop => self.is_bishop_move_legal(from_position, to_position),
                TypePiece::Rook => self.is_rook_move_legal(from_position, to_position),
                TypePiece::Queen => {
                    self.is_bishop_move_legal(from_position, to_position)
                        || self.is_rook_move_legal(from_position, to_position)
                }
                TypePiece::King => self.is_king_move_legal(from_position, to_position),
                TypePiece::Knight => self.is_knight_move_legal(from_position, to_position),
            }
        } && !self.does_move_put_king_in_check(from_position, to_position)
        {
            return true;
        }
        false
    }
    pub fn does_move_put_king_in_check(
        &self,
        from_position: [usize; 2],
        to_position: [usize; 2],
    ) -> bool {
        let mut temp_board = self.clone();

        let piece = temp_board.cells[from_position[0]][from_position[1]]
            .piece
            .take();
        temp_board.cells[to_position[0]][to_position[1]].piece = piece;

        let king_position = temp_board.find_king_position(
            temp_board.cells[to_position[0]][to_position[1]]
                .piece
                .unwrap()
                .color,
        );

        temp_board.is_square_attacked(king_position, self.find_cell(king_position).piece.unwrap().color)
    }
    pub fn is_square_attacked(&self, position: [usize; 2], attacking_color: ChessColor) -> bool {
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.cells[col][row].piece {
                    if piece.color == attacking_color && self.is_move_legal([col, row], position) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn find_king_position(&self, color: ChessColor) -> [usize; 2] {
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = &self.cells[col][row].piece {
                    if piece.type_of_piece == TypePiece::King && piece.color == color {
                        return [col, row];
                    }
                }
            }
        }
        panic!("King not found on the board!");
    }
    pub fn is_knight_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        let row_diff = (to_position[1] as isize - from_position[1] as isize).abs();
        let col_diff = (to_position[0] as isize - from_position[0] as isize).abs();
        if col_diff == 2 || row_diff == 2 {
            if col_diff == 1 || row_diff == 1 {
                if self.find_cell(to_position).piece.is_none() {
                    return true;
                }
                if self.find_cell(from_position).piece.unwrap().color
                    != self.find_cell(to_position).piece.unwrap().color
                {
                    return true;
                }
            }
        }
        false
    }
    pub fn is_king_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        let row_diff = to_position[1] as isize - from_position[1] as isize;
        let col_diff = to_position[0] as isize - from_position[0] as isize;
        if col_diff.abs() <= 1 && row_diff.abs() <= 1 {
            if self.find_cell(to_position).piece.is_none() {
                return true;
            } else {
                if self.find_cell(from_position).piece.unwrap().color
                    != self.find_cell(to_position).piece.unwrap().color
                {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
    pub fn is_path_clear(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        let mut row = from_position[1] as isize;
        let mut col = from_position[0] as isize;
        let row_diff = to_position[1] as isize - row;
        let col_diff = to_position[0] as isize - col;

        let row_step = if row_diff == 0 {
            0
        } else {
            row_diff / row_diff.abs()
        };
        let col_step = if col_diff == 0 {
            0
        } else {
            col_diff / col_diff.abs()
        };

        row += row_step;
        col += col_step;

        while row != to_position[1] as isize || col != to_position[0] as isize {
            if self.find_cell([col as usize, row as usize]).piece.is_some() {
                return false;
            }
            row += row_step;
            col += col_step;
        }

        true
    }
    pub fn is_rook_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        if (from_position[0] == to_position[0] || from_position[1] == to_position[1])
            && self.is_path_clear(from_position, to_position)
        {
            if self.find_cell(to_position).piece.is_none() {
                return true;
            } else {
                if self.find_cell(from_position).piece.unwrap().color
                    != self.find_cell(to_position).piece.unwrap().color
                {
                    return true;
                }
            }
        }

        false
    }
    pub fn is_bishop_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        let row_diff = to_position[1] as isize - from_position[1] as isize;
        let col_diff = to_position[0] as isize - from_position[0] as isize;
        if (row_diff.abs() == col_diff.abs()) && self.is_path_clear(from_position, to_position) {
            if self.find_cell(to_position).piece.is_none() {
                return true;
            } else {
                if self.find_cell(from_position).piece.unwrap().color
                    != self.find_cell(to_position).piece.unwrap().color
                {
                    return true;
                } else {
                    return false;
                }
            }
        } else {
            false
        }
    }
    pub fn is_pawn_move_legal(&self, from_position: [usize; 2], to_position: [usize; 2]) -> bool {
        let direction = if self.find_cell(from_position).piece.unwrap().color == ChessColor::White {
            1
        } else {
            -1
        };

        let row_diff = (to_position[1] as isize - from_position[1] as isize) * direction;
        let col_diff = (to_position[0] as isize - from_position[0] as isize).abs();

        match (row_diff, col_diff) {
            (1, 0) => {
                self.find_cell(to_position).piece.is_none()
                    && self.is_path_clear(from_position, to_position)
            } // Simple movement
            (2, 0) if from_position[1] == 1 || from_position[1] == 6 => {
                self.find_cell(to_position).piece.is_none()
                    && self.is_path_clear(from_position, to_position)
            } // Double initial movement
            (1, 1) => {
                self.find_cell(to_position).piece.is_some()
                    && self.find_cell(to_position).piece.unwrap().color
                        != self.find_cell(from_position).piece.unwrap().color
            } // Diagonal capture
            _ => false,
        }
    }

    pub fn full_positions(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                self.cells[i][j].position = [i, j];
            }
        }
    }
    pub fn find_cell(&self, position: [usize; 2]) -> &Cell {
        let [column, row] = position;

        if column < 8 && row < 8 {
            &self.cells[column][row]
        } else {
            panic!("Cell dont exist!");
        }
    }
    pub fn find_cell_mut(&mut self, position: [usize; 2]) -> &mut Cell {
        let [column, row] = position;

        if column < 8 && row < 8 {
            &mut self.cells[column][row]
        } else {
            panic!("Cell dont exist!");
        }
    }

    pub fn initialize_pieces(&mut self) {
        let white_pieces = [
            (TypePiece::Rook, [0, 0]),
            (TypePiece::Knight, [1, 0]),
            (TypePiece::Bishop, [2, 0]),
            (TypePiece::Queen, [3, 0]),
            (TypePiece::King, [4, 0]),
            (TypePiece::Bishop, [5, 0]),
            (TypePiece::Knight, [6, 0]),
            (TypePiece::Rook, [7, 0]),
        ];

        for &(type_of_piece, position) in white_pieces.iter() {
            let cell = self.find_cell_mut(position);
            cell.piece = Some(Piece {
                type_of_piece,
                color: ChessColor::White,
            });
        }

        for column in 0..8 {
            let cell = self.find_cell_mut([column, 1]);
            cell.piece = Some(Piece {
                type_of_piece: TypePiece::Pawn,
                color: ChessColor::White,
            });
        }

        let black_pieces = [
            (TypePiece::Rook, [0, 7]),
            (TypePiece::Knight, [1, 7]),
            (TypePiece::Bishop, [2, 7]),
            (TypePiece::Queen, [3, 7]),
            (TypePiece::King, [4, 7]),
            (TypePiece::Bishop, [5, 7]),
            (TypePiece::Knight, [6, 7]),
            (TypePiece::Rook, [7, 7]),
        ];

        for &(type_of_piece, position) in black_pieces.iter() {
            let cell = self.find_cell_mut(position);
            cell.piece = Some(Piece {
                type_of_piece,
                color: ChessColor::Black,
            });
        }

        for column in 0..8 {
            let cell = self.find_cell_mut([column, 6]);
            cell.piece = Some(Piece {
                type_of_piece: TypePiece::Pawn,
                color: ChessColor::Black,
            });
        }
    }

    pub fn new() -> Self {
        let mut cells = [[Cell {
            position: [0, 0],
            piece: None,
        }; 8]; 8];
        let mut grid = Grid { cells };
        grid.full_positions();
        grid.initialize_pieces();
        grid
    }

    pub fn print_board(&self) {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.cells[i][j].piece {
                    print!("{:?} ", piece.type_of_piece);
                } else {
                    print!("None ");
                }
            }
            println!();
        }
    }

    pub fn position_to_chess_notation(position: [usize; 2]) -> String {
        let column = (position[0] + 'a' as usize) as u8 as char;
        let row = (position[1] + 1).to_string();
        format!("{}{}", column, row)
    }
}

async fn load_textures() -> std::collections::HashMap<Piece, Texture2D> {
    let mut textures = std::collections::HashMap::new();

    let white_pawn = load_texture("newimg/white-pawn.png").await.unwrap();
    let black_pawn = load_texture("newimg/black-pawn.png").await.unwrap();
    let white_rook = load_texture("newimg/white-rook.png").await.unwrap();
    let black_rook = load_texture("newimg/black-rook.png").await.unwrap();
    let white_bishop = load_texture("newimg/white-bishop.png").await.unwrap();
    let black_bishop = load_texture("newimg/black-bishop.png").await.unwrap();
    let white_queen = load_texture("newimg/white-queen.png").await.unwrap();
    let black_queen = load_texture("newimg/black-queen.png").await.unwrap();
    let white_knight = load_texture("newimg/white-knight.png").await.unwrap();
    let black_knight = load_texture("newimg/black-knight.png").await.unwrap();
    let white_king = load_texture("newimg/white-king.png").await.unwrap();
    let black_king = load_texture("newimg/black-king.png").await.unwrap();

    textures.insert(
        Piece {
            type_of_piece: TypePiece::Pawn,
            color: ChessColor::White,
        },
        white_pawn,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Pawn,
            color: ChessColor::Black,
        },
        black_pawn,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Rook,
            color: ChessColor::White,
        },
        white_rook,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Rook,
            color: ChessColor::Black,
        },
        black_rook,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Bishop,
            color: ChessColor::White,
        },
        white_bishop,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Bishop,
            color: ChessColor::Black,
        },
        black_bishop,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Queen,
            color: ChessColor::White,
        },
        white_queen,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Queen,
            color: ChessColor::Black,
        },
        black_queen,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Knight,
            color: ChessColor::White,
        },
        white_knight,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::Knight,
            color: ChessColor::Black,
        },
        black_knight,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::King,
            color: ChessColor::White,
        },
        white_king,
    );
    textures.insert(
        Piece {
            type_of_piece: TypePiece::King,
            color: ChessColor::Black,
        },
        black_king,
    );
    textures
}

#[macroquad::main(conf())]
async fn main() {
    let mut app = App::new();
    let textures = load_textures().await;
    loop {
        clear_background(WHITE);

        let (screen_width, screen_height) = (screen_width(), screen_height());
        let cell_size = (screen_width.min(screen_height)) / 8.0;
        let offset_x = (screen_width - cell_size * 8.0) / 2.0;
        let offset_y = (screen_height - cell_size * 8.0) / 2.0;

        app.draw(&textures, cell_size, offset_x, offset_y);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            app.move_piece(mouse_x, mouse_y, cell_size, offset_x, offset_y);
        }

        next_frame().await
    }
}

fn conf() -> Conf {
    Conf {
        window_width: 900,
        window_height: 900,
        window_resizable: false,
        window_title: "Chess".to_string(),
        ..Default::default()
    }
}

// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn it_works(){
//         let piece = Piece{type_of_piece:TypePiece::King, color:ChessColor::Black};
//         let mut cell = Cell{name:['a','a'], piece:Some(piece)};
//         let mut grid = Grid{cells: [[cell.clone();8];8]};
//         grid.full_names();
//         for cell in grid.cells {
//             for i in cell{
//                 println!("{:?}",i.name);
//             }
//         }
//     }
// }
