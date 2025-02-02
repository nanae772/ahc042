#![allow(clippy::needless_range_loop, clippy::ptr_arg)]
use std::fmt;

const BOARD_SIZE: usize = 20;

#[derive(Clone, Copy, Debug, PartialEq)]
enum RoomState {
    Oni = -1,
    Fuku = 1,
    Vacant = 0,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "L",
                Direction::Right => "R",
                Direction::Up => "U",
                Direction::Down => "D",
            }
        )
    }
}

#[derive(Clone, Copy)]
struct Operation {
    dir: Direction,
    index: usize,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.dir, self.index)
    }
}

#[derive(Clone)]
struct BoardState {
    n: usize,
    board: Vec<Vec<RoomState>>,
    num_oni: usize,
    num_fuku: usize,
}

impl BoardState {
    /// y行目の(begin, end)間に福がいるか
    fn exists_fuku_in_row(&self, y: usize, begin: usize, end: usize) -> bool {
        self.board[y][begin..end].contains(&RoomState::Fuku)
    }

    /// x列目の(begin, end)間に福がいるか
    fn exists_fuku_in_col(&self, x: usize, begin: usize, end: usize) -> bool {
        for i in begin..end {
            if self.board[i][x] == RoomState::Fuku {
                return true;
            }
        }
        false
    }

    fn exists_oni_around_all_fuku(&self) -> bool {
        for i in 0..self.n {
            for j in 0..self.n {
                if self.board[i][j] != RoomState::Oni {
                    continue;
                }
                assert_eq!(self.board[i][j], RoomState::Oni);
                let can_fall = !self.exists_fuku_in_col(j, 0, i)
                    || !self.exists_fuku_in_col(j, i + 1, self.n)
                    || !self.exists_fuku_in_row(i, 0, j)
                    || !self.exists_fuku_in_row(i, j + 1, self.n);
                if !can_fall {
                    return true;
                }
            }
        }

        false
    }

    fn apply(&mut self, operation: &Operation) {
        match operation.dir {
            Direction::Left => {
                match self.board[operation.index][0] {
                    RoomState::Oni => self.num_oni -= 1,
                    RoomState::Fuku => self.num_fuku -= 1,
                    RoomState::Vacant => (),
                }
                for j in 0..self.n - 1 {
                    self.board[operation.index][j] = self.board[operation.index][j + 1];
                }
                self.board[operation.index][self.n - 1] = RoomState::Vacant;
            }
            Direction::Right => {
                match self.board[operation.index][self.n - 1] {
                    RoomState::Oni => self.num_oni -= 1,
                    RoomState::Fuku => self.num_fuku -= 1,
                    RoomState::Vacant => (),
                }
                for j in (1..self.n).rev() {
                    self.board[operation.index][j] = self.board[operation.index][j - 1];
                }
                self.board[operation.index][0] = RoomState::Vacant;
            }
            Direction::Up => {
                match self.board[0][operation.index] {
                    RoomState::Oni => self.num_oni -= 1,
                    RoomState::Fuku => self.num_fuku -= 1,
                    RoomState::Vacant => (),
                }
                for i in 0..self.n - 1 {
                    self.board[i][operation.index] = self.board[i + 1][operation.index];
                }
                self.board[self.n - 1][operation.index] = RoomState::Vacant;
            }
            Direction::Down => {
                match self.board[self.n - 1][operation.index] {
                    RoomState::Oni => self.num_oni -= 1,
                    RoomState::Fuku => self.num_fuku -= 1,
                    RoomState::Vacant => (),
                }
                for i in (1..self.n).rev() {
                    self.board[i][operation.index] = self.board[i - 1][operation.index];
                }
                self.board[0][operation.index] = RoomState::Vacant;
            }
        }
    }
}

impl fmt::Debug for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty_print_board_row = |row: &Vec<RoomState>| -> String {
            let mut s = row
                .iter()
                .map(|rs| match rs {
                    RoomState::Oni => "x",
                    RoomState::Fuku => "o",
                    RoomState::Vacant => ".",
                })
                .collect::<String>();
            s.push('\n');
            s
        };
        let str_board: String = self.board.iter().map(pretty_print_board_row).collect();
        write!(f, "{}", str_board)
    }
}

fn input_parser() -> BoardState {
    proconio::input! {
        n: usize,
    }

    assert_eq!(n, BOARD_SIZE);
    let mut board = vec![vec![RoomState::Vacant; n]; n];

    for i in 0..n {
        proconio::input! {
            si: proconio::marker::Chars,
        }
        assert_eq!(si.len(), n);
        for j in 0..n {
            board[i][j] = match si[j] {
                'x' => RoomState::Oni,
                'o' => RoomState::Fuku,
                '.' => RoomState::Vacant,
                _ => unreachable!(),
            }
        }
    }

    BoardState {
        n,
        board,
        num_oni: 2 * n,
        num_fuku: 2 * n,
    }
}

fn find_oni(board_state: &BoardState) -> (usize, usize) {
    assert!(board_state.num_oni > 0);
    for i in 0..board_state.n {
        for j in 0..board_state.n {
            if board_state.board[i][j] == RoomState::Oni {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn get_rev_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}

/// (y, x) にいる鬼を一番近い端に落としてから初期盤面に戻すという操作列を返す
fn fall_oni_and_reset(y: usize, x: usize, board_state: &BoardState) -> Vec<Operation> {
    assert_eq!(board_state.board[y][x], RoomState::Oni);

    let mut dir = None;
    let mut min_op = 1000;
    let mut index = 0;

    if !board_state.exists_fuku_in_row(y, 0, x) && x + 1 < min_op {
        dir = Some(Direction::Left);
        min_op = x + 1;
        index = y;
    }

    if !board_state.exists_fuku_in_row(y, x + 1, board_state.n) && board_state.n - x < min_op {
        dir = Some(Direction::Right);
        min_op = board_state.n - x;
        index = y;
    }

    if !board_state.exists_fuku_in_col(x, 0, y) && y + 1 < min_op {
        dir = Some(Direction::Up);
        min_op = y + 1;
        index = x;
    }

    if !board_state.exists_fuku_in_col(x, y + 1, board_state.n) && board_state.n - y < min_op {
        dir = Some(Direction::Down);
        min_op = board_state.n - y;
        index = x;
    }

    assert!(dir.is_some());
    let dir = dir.unwrap();

    let mut res = Vec::new();
    for _ in 0..min_op {
        res.push(Operation { dir, index });
    }

    // 動かした行（列）に福がいないなら元に戻さなくてよい
    match dir {
        Direction::Left | Direction::Right => {
            if !board_state.exists_fuku_in_row(index, 0, board_state.n) {
                return res;
            }
        }
        Direction::Down | Direction::Up => {
            if !board_state.exists_fuku_in_col(x, 0, board_state.n) {
                return res;
            }
        }
    }

    // 動かした後の盤面において、全ての鬼が四方を福に囲まれていなければ元に戻さなくてよい
    let mut after_board = board_state.clone();
    for operation in &res {
        after_board.apply(operation);
    }

    if !after_board.exists_oni_around_all_fuku() {
        return res;
    }

    // 初期盤面に戻す
    for _ in 0..min_op {
        res.push(Operation {
            dir: get_rev_dir(dir),
            index,
        })
    }
    res
}

fn main() {
    let mut board_state = input_parser();
    let mut ans = Vec::new();

    while board_state.num_oni > 0 {
        let (y, x) = find_oni(&board_state);
        let operations = fall_oni_and_reset(y, x, &board_state);
        for operation in operations {
            board_state.apply(&operation);
            ans.push(operation);
            // eprintln!("op: {}", operation);
        }
        // eprintln!("num_oni: {}", board_state.num_oni);
    }

    for operation in ans {
        println!("{}", operation);
    }
}

#[cfg(test)]
mod unittests {
    use super::*;

    #[test]
    fn test_apply_operation_left() {
        let mut board_state = BoardState {
            n: 3,
            board: vec![
                vec![RoomState::Oni, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ],
            num_oni: 3,
            num_fuku: 3,
        };

        board_state.apply(&Operation {
            dir: Direction::Left,
            index: 0,
        });

        assert_eq!(
            board_state.board,
            vec![
                vec![RoomState::Vacant, RoomState::Fuku, RoomState::Vacant],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ]
        );
        assert_eq!(board_state.num_oni, 2);
        assert_eq!(board_state.num_fuku, 3);
    }

    #[test]
    fn test_apply_operation_right() {
        let mut board_state = BoardState {
            n: 3,
            board: vec![
                vec![RoomState::Oni, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ],
            num_oni: 3,
            num_fuku: 3,
        };

        board_state.apply(&Operation {
            dir: Direction::Right,
            index: 0,
        });

        assert_eq!(
            board_state.board,
            vec![
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Vacant],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ]
        );
        assert_eq!(board_state.num_oni, 3);
        assert_eq!(board_state.num_fuku, 2);
    }

    #[test]
    fn test_apply_operation_up() {
        let mut board_state = BoardState {
            n: 3,
            board: vec![
                vec![RoomState::Oni, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ],
            num_oni: 3,
            num_fuku: 3,
        };

        board_state.apply(&Operation {
            dir: Direction::Up,
            index: 0,
        });

        assert_eq!(
            board_state.board,
            vec![
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Vacant, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ]
        );
        assert_eq!(board_state.num_oni, 2);
        assert_eq!(board_state.num_fuku, 3);
    }

    #[test]
    fn test_apply_operation_down() {
        let mut board_state = BoardState {
            n: 3,
            board: vec![
                vec![RoomState::Oni, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Oni],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Fuku],
            ],
            num_oni: 3,
            num_fuku: 3,
        };

        board_state.apply(&Operation {
            dir: Direction::Down,
            index: 2,
        });

        assert_eq!(
            board_state.board,
            vec![
                vec![RoomState::Oni, RoomState::Vacant, RoomState::Vacant],
                vec![RoomState::Fuku, RoomState::Vacant, RoomState::Fuku],
                vec![RoomState::Vacant, RoomState::Oni, RoomState::Oni],
            ]
        );
        assert_eq!(board_state.num_oni, 3);
        assert_eq!(board_state.num_fuku, 2);
    }
}
