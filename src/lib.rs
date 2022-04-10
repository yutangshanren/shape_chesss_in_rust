pub mod shape;

use std::os::raw::c_float;
use std::process::exit;
use colored::Colorize;
use std::rc::Rc;
use crate::shape::*;
use std::time::Instant;

const BOARD_SIZE: usize = 10;

struct Config {
    items: Vec<[usize;4]>,
}

impl Config {
    fn new_from_file() -> Config {
        let mut items: Vec<[usize;4]> = vec![];

        let a:[usize;4]= [2,0,0,7];
        items.push(a);

        let a:[usize;4] = [1,0,2,7];
        items.push(a);

        Config {
            items,
        }
    }
}

struct Board {
    // store the chess number plus 1.
    // e.g.
    // if chess is ShapeTypeStar, then 1
    // if chess is ShapeTypeM, then 2
    // etc.
    board: Box<[[usize;BOARD_SIZE];BOARD_SIZE]>,
}

impl Board {
    pub fn new() -> Board {
        let mut b = [[100usize;BOARD_SIZE];BOARD_SIZE];
        for h in 0..BOARD_SIZE {
            for w in 0..BOARD_SIZE {
                if w <=h {
                    b[h][w] = 0;
                }
            }
        }
        Board {
            board: Box::new(b),
        }
    }

    pub fn new_board_from_tree(tree_node: Rc<ShapeTreeNode>) -> Option<Board> {
        //let mut board = Box::new([[0usize;BOARD_SIZE];BOARD_SIZE]);
        let mut board_wrapper = Board::new();
        let board = &mut board_wrapper.board;

        let p = &tree_node;
        if p.check_shape_on_board(board) {
            p.put_shape_on_board(board);
        }
        else {
            println!("Construct board error for self!");
            return None;
        }

        let mut x = p;
        while let Some(t) = &x.parent {
            if t.check_shape_on_board(board) {
                t.put_shape_on_board(board);
            }
            else {
                println!("Construct board error for parent!");
                return None;
            }
            x = t;
        }
        return Some(board_wrapper);
    }

    fn show_board(&self) {
        let pb = &self.board;
        for h in 0..BOARD_SIZE {
            for w in 0..BOARD_SIZE {
                if w <= h {
                    print!("{} ",pb[h][w]);
                }
            }
            println!(" ");
        }
    }

    fn show_board2(&self) {
        let pb = &self.board;

        for h in 0..BOARD_SIZE {
            let mut start_row = BOARD_SIZE - 1;
            let mut start_column = h;

            for _ in 0..BOARD_SIZE-1-h {
                print!("  ");
            }

            for _ in 0..=h {
                if pb[start_row][start_column] == 0 {
                    print!("*    ");
                }
                else {
                    let shape_type = map_from_index_to_shape(pb[start_row][start_column]-1);
                    match shape_type {
                        ShapeType::ShapeTypeStar => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).red());
                        }
                        ShapeType::ShapeTypeM => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).red());
                        }
                        ShapeType::ShapeTypeBigCorner => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).blue());
                        }
                        ShapeType::ShapeTypeZigzag => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).purple());
                        }
                        ShapeType::ShapeTypeSquareHorn => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).cyan());
                        }
                        ShapeType::ShapeTypeOpenMouth => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).white());
                        }
                        ShapeType::ShapeTypeBigL => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).yellow());
                        }
                        ShapeType::ShapeTypeLineHorn => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).green());
                        }
                        ShapeType::ShapeTypeSmallL => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).white());
                        }
                        ShapeType::ShapeTypeSmallCorner => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).red());
                        }
                        ShapeType::ShapeTypeSquare => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).yellow());
                        }
                        ShapeType::ShapeTypeLine => {
                            print!("{}   ", format!("{:02}",pb[start_row][start_column]-1).green());
                        }
                        ShapeType::ShapeTypeMax => {
                            print!("  ");
                        }
                    }
                }

                if start_row > 0 {
                    start_row -= 1;
                }

                if start_column > 0 {
                    start_column -= 1;
                }

            }
            println!();
        }
    }

    fn test_set_board(&mut self) {
        let pb = &mut self.board;
        for h in 0..BOARD_SIZE {
            for w in 0..BOARD_SIZE {
                if w <= h {
                    pb[h][w] = h+1;
                }
            }
        }
    }
}

pub fn play_game() {

    let mut play_tree: Vec<Vec<Rc<ShapeTreeNode>>> = vec![];

    //------------------------------------------------
    // Initialize shape index map.
    //------------------------------------------------
    let shape_index_max = map_from_shape_to_index(ShapeType::ShapeTypeMax);
    let mut shape_valid_map: Vec<usize> = vec![];
    for _ in 0..shape_index_max {
        shape_valid_map.push(1);
    }

    //------------------------------------------------
    // Read config from file.
    //------------------------------------------------
    let conf = Config::new_from_file();


    //------------------------------------------------
    // Construct the initial tree.
    //------------------------------------------------
    for item in conf.items {

        if item[0] >= shape_index_max {
            panic!("{}",format!("======Error shape index: {}===========", item[0]).red());
        }

        let mut nodes_gen_by_current_shape: Vec<Rc<ShapeTreeNode>> = vec![];
        if play_tree.len() == 0 {
            let node = Rc::new(ShapeTreeNode::new(item[0],item[1],item[2],item[3],None));
            nodes_gen_by_current_shape.push(node);
            shape_valid_map[item[0]] = 0;
        }
        else {
            let parent_node = &play_tree[play_tree.len()-1];
            let t= &parent_node[0];
            let node = Rc::new(ShapeTreeNode::new(item[0],item[1],item[2],item[3],Some(Rc::clone(t))));
            nodes_gen_by_current_shape.push(node);
            shape_valid_map[item[0]] = 0;
        }
        play_tree.push(nodes_gen_by_current_shape);
    }

    /*
    for nod in &play_tree {
        for p in nod {
            println!("[{},{},{},{}]",p.s_index,p.sub_index,p.board_x,p.board_y);
            let mut x = p;
            while let Some(t) = &x.parent {
                println!("    |    ");
                println!("    v    ");
                println!("[{},{},{},{}]",t.s_index,t.sub_index,t.board_x,t.board_y);
                x = t;
            }
            println!("=================");
        }
    }


    let tail = &play_tree[play_tree.len()-1];
    for p in tail {
        println!("[{},{},{},{}]",p.s_index,p.sub_index,p.board_x,p.board_y);
        let mut x = p;
        while let Some(t) = &x.parent {
            println!("    |    ");
            println!("    v    ");
            println!("[{},{},{},{}]",t.s_index,t.sub_index,t.board_x,t.board_y);
            x = t;
        }
        println!("=================");
    }
     */

    //------------------------------------------------
    // Play the game.
    //------------------------------------------------

    {
        // show initial board.
        let tail = &play_tree[play_tree.len()-1];
        let board = match Board::new_board_from_tree(Rc::clone(&tail[0])) {
            Some(tmp) => tmp,
            None => {
                panic!("{}",format!("======Play error when construct board.===========").red());
            }
        };
        println!("Initial board is:");
        board.show_board2();

    }

    let now = Instant::now();

    for shape_idx in 0..shape_index_max {
        if shape_valid_map[shape_idx] == 0 {
            // current shape is already on board.
            continue;
        }

        let now2 = Instant::now();
        //println!("{:4.2}%", f32::from(shape_idx as u8) / f32::from(shape_index_max as u8) * 100.0);

        // mark this shape invalid.
        shape_valid_map[shape_idx] = 0;

        // current nodes vector.
        let mut nodes_gen_by_current_shape: Vec<Rc<ShapeTreeNode>> = vec![];

        let possible_parents = &play_tree[play_tree.len() - 1];
        for cur_parents in possible_parents {
            let board_wrap = match Board::new_board_from_tree(Rc::clone(cur_parents)) {
                Some(tmp) => tmp,
                None => {
                    panic!("{}",format!("======Play error when construct board.===========").red());
                }
            };
            let board = &board_wrap.board;

            let sub_shape_number = get_sub_shape_number(shape_idx);

            for n in 0..sub_shape_number {
                for y in 0..BOARD_SIZE {
                    for x in 0..=y {
                        let node = Rc::new(ShapeTreeNode::new(shape_idx,n,x,y,Some(Rc::clone(cur_parents))));
                        let mut valid = false;

                        {
                            let node_ref = &node;
                            if node_ref.check_shape_on_board(&board) {
                                valid = true;
                            }
                        }

                        if valid {
                            nodes_gen_by_current_shape.push(node);
                        }

                    }
                }
            }
        }

        if nodes_gen_by_current_shape.len() == 0 {
            panic!("{}",format!("======Could not find a solution.===========").red());
        }
        play_tree.push(nodes_gen_by_current_shape);

        print!("{:4.2}%", f32::from(shape_idx as u8) / f32::from(shape_index_max as u8) * 100.0);
        let elapsed_time2 = now2.elapsed();
        println!("    => {} Seconds", elapsed_time2.as_secs());
    }

    println!("100%");

    let elapsed_time = now.elapsed();
    println!(" Total took {} seconds to finish!", elapsed_time.as_secs());

    {
        // show final board.
        let tail = &play_tree[play_tree.len()-1];

        for t in tail {
            let board = match Board::new_board_from_tree(Rc::clone(t)) {
                Some(tmp) => tmp,
                None => {
                    panic!("{}",format!("======Play error when construct board in final show.===========").red());
                }
            };
            println!();
            board.show_board2();
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_board() {
        let mut play_board = Board::new();
        play_board.test_set_board();
        play_board.show_board2();
    }
}