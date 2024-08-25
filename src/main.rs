use std::panic;

use macroquad::prelude::*;
use macroquad::window::Conf;

pub struct App{
    turn:Turn,
    grid:Grid,
    selected_piece: Option<(usize,usize)>
}
impl App {
    pub fn new() -> Self {
        App {
            turn: Turn::White,
            grid: Grid::new(),
            selected_piece: None,
        }
    }

    pub fn move_piece(&mut self, mouse_x: f32, mouse_y: f32) {
        let col = (mouse_x / 125.0) as usize;
        let row = 7 - (mouse_y / 125.0) as usize;

        if col < 8 && row < 8 {
            match self.selected_piece {
                None => {
                    if self.grid.cells[col][row].piece.is_some() {
                        self.selected_piece = Some((col, row));
                    }
                }
                Some((from_col, from_row)) => {
                    if self.grid.is_move_legal([from_col,from_row],[col,row]){
                        let piece = self.grid.cells[from_col][from_row].piece.take();
                        self.grid.cells[col][row].piece = piece;
                        self.selected_piece = None;
                    }
                    self.selected_piece = None;
                }
            }
        }
    }

    pub fn draw(&self, textures: &std::collections::HashMap<Piece, Texture2D>) {
        for i in 0..=7 {
            for j in 0..=7 {
                let x = 125.0 * i as f32;
                let y = 125.0 * j as f32;
        
                if (i + j) % 2 == 0 {
                    draw_rectangle(x, y, 125.0, 125.0, WHITE);
                } else {
                    draw_rectangle(x, y, 125.0, 125.0, GRAY);
                }
            }
        }
        
        for i in 0..=7 {
            for j in 0..=7 {
                let cell = &self.grid.cells[i][j];
                if let Some(piece) = cell.piece {
                    if let Some(texture) = textures.get(&piece) {
                        let x = 125.0 * i as f32;
                        let y = 125.0 * (7 - j) as f32;
                        draw_texture(texture, x, y, WHITE);
                    }
                }
            }
        }

        if let Some((col, row)) = self.selected_piece {
            let x = 125.0 * col as f32;
            let y = 125.0 * (7 - row) as f32;
            draw_rectangle_lines(x, y, 125.0, 125.0, 3.0, YELLOW);
        }
    }
}
pub enum Turn{
    White,Black
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypePiece{
    King,Queen,Rook,Bishop,Knight,Pawn
}
#[derive(Clone, Copy, Debug,PartialEq, Eq, Hash)]
pub enum ChessColor{
    White,Black
}
#[derive(Clone, Copy, Debug,PartialEq, Eq, Hash)]
pub struct Piece{
    type_of_piece: TypePiece,
    color: ChessColor,
}
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    position: [usize; 2],
    piece: Option<Piece>,
}

#[derive(Debug)]
pub struct Grid {
    cells: [[Cell; 8]; 8],
}

impl Grid {
    
    pub fn is_move_legal(&self, from_position: [usize; 2],to_position: [usize; 2])-> bool{
        if {
            match self.find_cell(from_position).piece.unwrap().type_of_piece {
                TypePiece::Pawn => self.is_pawn_move_legal(from_position, to_position),
                TypePiece::Bishop => self.is_bishop_move_legal(from_position, to_position),
                _ => false
            }
        }{
            return true;
        }
        false
    }
    pub fn is_bishop_move_legal(&self, from_position: [usize; 2],to_position: [usize; 2])->bool{
        todo!()
    }
    pub fn is_pawn_move_legal(&self, from_position: [usize; 2],to_position: [usize; 2])->bool{
        
        let direction = if self.find_cell(from_position).piece.unwrap().color == ChessColor::White { 1 } else { -1 };
    
        let row_diff = (to_position[1] as isize - from_position[1] as isize) * direction;
        let col_diff = (to_position[0] as isize - from_position[0] as isize).abs();

        match (row_diff, col_diff) {
            (1, 0) => self.find_cell(to_position).piece.is_none(), // Simple movement
            (2, 0) if from_position[1] == 1 || from_position[1] == 6 => self.find_cell(to_position).piece.is_none(), // Double initial movement 
            (1, 1) => self.find_cell(to_position).piece.is_some(), // Diagonal capture
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
        let mut cells = [[Cell { position: [0, 0], piece: None }; 8]; 8];
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

    let white_pawn = load_texture("img/white-pawn.png").await.unwrap();
    let black_pawn = load_texture("img/black-pawn.png").await.unwrap();
    let white_rook = load_texture("img/white-rook.png").await.unwrap();
    let black_rook = load_texture("img/black-rook.png").await.unwrap();
    let white_bishop = load_texture("img/white-bishop.png").await.unwrap();
    let black_bishop = load_texture("img/black-bishop.png").await.unwrap();
    let white_queen = load_texture("img/white-queen.png").await.unwrap();
    let black_queen = load_texture("img/black-queen.png").await.unwrap();
    let white_knight = load_texture("img/white-knight.png").await.unwrap();
    let black_knight = load_texture("img/black-knight.png").await.unwrap();
    let white_king = load_texture("img/white-king.png").await.unwrap();
    let black_king = load_texture("img/black-king.png").await.unwrap();
    
    textures.insert(Piece { type_of_piece: TypePiece::Pawn, color: ChessColor::White }, white_pawn);
    textures.insert(Piece { type_of_piece: TypePiece::Pawn, color: ChessColor::Black }, black_pawn);
    textures.insert(Piece { type_of_piece: TypePiece::Rook, color: ChessColor::White }, white_rook);
    textures.insert(Piece { type_of_piece: TypePiece::Rook, color: ChessColor::Black }, black_rook);
    textures.insert(Piece { type_of_piece: TypePiece::Bishop, color: ChessColor::White }, white_bishop);
    textures.insert(Piece { type_of_piece: TypePiece::Bishop, color: ChessColor::Black }, black_bishop);
    textures.insert(Piece { type_of_piece: TypePiece::Queen, color: ChessColor::White }, white_queen);
    textures.insert(Piece { type_of_piece: TypePiece::Queen, color: ChessColor::Black }, black_queen);
    textures.insert(Piece { type_of_piece: TypePiece::Knight, color: ChessColor::White }, white_knight);
    textures.insert(Piece { type_of_piece: TypePiece::Knight, color: ChessColor::Black }, black_knight);
    textures.insert(Piece { type_of_piece: TypePiece::King, color: ChessColor::White }, white_king);
    textures.insert(Piece { type_of_piece: TypePiece::King, color: ChessColor::Black }, black_king);
    textures
}

fn move_piece(grid: &mut Grid, textures: &std::collections::HashMap<Piece, Texture2D>) {
    static mut SELECTED_PIECE: Option<(usize, usize)> = None;

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        let col = (mouse_x / 125.0) as usize;
        let row = 7 - (mouse_y / 125.0) as usize;

        if col < 8 && row < 8 {
            unsafe {
                match SELECTED_PIECE {
                    None => {
                        if grid.cells[col][row].piece.is_some() {
                            SELECTED_PIECE = Some((col, row));
                        }
                    }
                    Some((from_col, from_row)) => {
                        let piece = grid.cells[from_col][from_row].piece.take();
                        grid.cells[col][row].piece = piece;
                        SELECTED_PIECE = None;
                    }
                }
            }
        }
    }

    unsafe {
        if let Some((col, row)) = SELECTED_PIECE {
            let x = 125.0 * col as f32;
            let y = 125.0 * (7 - row) as f32;
            draw_rectangle_lines(x, y, 125.0, 125.0, 3.0, YELLOW);
        }
    }
}



#[macroquad::main(conf())]
async fn main(){
    let mut app = App::new();
    let textures = load_textures().await;
    app.grid.print_board();
    loop {
        clear_background(WHITE);
        
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            app.move_piece(mouse_x, mouse_y);
        }

        app.draw(&textures);

        next_frame().await
    }
}


fn conf()-> Conf{
    Conf{
        window_width:1000,
        window_height:1000,
        window_resizable:false,
        window_title:"Chess".to_string(),
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