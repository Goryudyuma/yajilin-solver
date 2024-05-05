use std::cmp::PartialEq;
use std::fmt;
use union_find::UnionFind;

pub struct Board(Vec<Vec<Cell>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Wall(wall) => {
                        match wall {
                            WallEnum::Wall => write!(f, "W")?,
                            WallEnum::Hint(dir, num) => write!(f, "{}{}", dir_to_char(dir.clone()), num)?,
                        }
                    }
                    Cell::Space(_, _) => write!(f, "・")?,
                    Cell::Unknown => write!(f, "  ")?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, " ")
    }
}

#[derive(Debug, Clone)]
enum DirectionEnum {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl PartialEq for DirectionEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DirectionEnum::None, DirectionEnum::None) => true,
            (DirectionEnum::Up, DirectionEnum::Up) => true,
            (DirectionEnum::Down, DirectionEnum::Down) => true,
            (DirectionEnum::Left, DirectionEnum::Left) => true,
            (DirectionEnum::Right, DirectionEnum::Right) => true,
            _ => false,
        }
    }
}

impl DirectionEnum {
    fn to_vector(&self) -> (i32, i32) {
        match self {
            DirectionEnum::None => (0, 0),
            DirectionEnum::Up => (-1, 0),
            DirectionEnum::Down => (1, 0),
            DirectionEnum::Left => (0, -1),
            DirectionEnum::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
enum WallEnum {
    Wall,
    Hint(DirectionEnum, i64),
}

#[derive(Debug, Clone)]
enum CellEnum {
    Wall(WallEnum),
    Space(Option<DirectionEnum>, Option<DirectionEnum>),
    Unknown,
}

type Cell = CellEnum;

fn main() {
    let problem = "10/10/202022l40i4141h40f122242l31i2131h30b42c101210c41i";
    let board = create_board(problem);
    println!("{}", board);
    println!("{:?}", check(board))
}

fn create_board(problem: &str) -> Board {
    // まずは/で分割
    let iter: Vec<&str> = problem.split("/").collect();

    // 高さと幅を取得
    let height: usize = iter[0].parse().unwrap();
    let width: usize = iter[1].parse().unwrap();

    // iter[2]を一文字ずつ取り出して処理
    let b = create_board_sub(iter[2].chars());

    return Board(b.chunks(width).map(|x| x.to_vec()).collect());
}


/*
var ca = bstr.charAt(i), cell=bd.cell[c];

			if(this.include(ca,"0","4")){
				var ca1 = bstr.charAt(i+1);
				cell.qdir = parseInt(ca,16);
				cell.qnum = (ca1!=="." ? parseInt(ca1,16) : -2);
				i++;
			}
			else if(this.include(ca,"5","9")){
				cell.qdir = parseInt(ca,16)-5;
				cell.qnum = parseInt(bstr.substr(i+1,2),16);
				i+=2;
			}
			else if(ca==="-"){
				cell.qdir = parseInt(bstr.substr(i+1,1),16);
				cell.qnum = parseInt(bstr.substr(i+2,3),16);
				i+=4;
			}
			else if(ca>='a' && ca<='z'){ c+=(parseInt(ca,36)-10);}

			c++;
			if(!bd.cell[c]){ break;}
 */
fn create_board_sub(mut chars: std::str::Chars) -> Vec<CellEnum> {
    let now = chars.next();
    return match now {
        None => {
            Vec::new()
        }
        Some(c) => {
            match c {
                '0'..='4' => {
                    let next = chars.next();
                    let qdir = c.to_digit(16).unwrap();
                    let qnum = match next {
                        Some('.') => -2,
                        Some(ca1) => ca1.to_digit(16).unwrap() as i64,
                        None => -2,
                    };
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                '5'..='9' => {
                    let qdir = c.to_digit(16).unwrap();
                    let qnum = chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64;
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                '-' => {
                    let qdir = chars.next().unwrap().to_digit(16).unwrap();
                    let qnum = chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64 * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64;
                    let mut result = vec![Cell::Wall(WallEnum::Hint(dir_to_direction_enum(qdir), qnum))];
                    result.extend(create_board_sub(chars));
                    result
                }
                'a'..='z' => {
                    let c = c.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
                    let mut result = vec![Cell::Unknown; c as usize];
                    result.extend(create_board_sub(chars));
                    result
                }
                _ => {
                    panic!("error")
                }
            }
        }
    };
}

fn dir_to_direction_enum(dir: u32) -> DirectionEnum {
    return match dir {
        1 => DirectionEnum::Up,
        2 => DirectionEnum::Down,
        3 => DirectionEnum::Left,
        4 => DirectionEnum::Right,
        _ => DirectionEnum::None,
    };
}

fn dir_to_char(dir: DirectionEnum) -> char {
    return match dir {
        DirectionEnum::Up => '↑',
        DirectionEnum::Down => '↓',
        DirectionEnum::Left => '←',
        DirectionEnum::Right => '→',
        _ => ' ',
    };
}

#[derive(Debug)]
enum CheckResultInvalidEnum {
    AdjacentWall(i32, i32),
    Hint(i32, i32),
}

#[derive(Debug)]
enum CheckResultEnum {
    Valid, // 矛盾点はない
    Invalid(CheckResultInvalidEnum), // 矛盾点がある
    Complete, // 解けている
}

impl PartialEq for CellEnum {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellEnum::Wall(WallEnum::Wall), CellEnum::Wall(WallEnum::Wall)) => true,
            (CellEnum::Wall(WallEnum::Hint(_, _)), CellEnum::Wall(WallEnum::Hint(_, _))) => true,
            (CellEnum::Space(_, _), CellEnum::Space(_, _)) => true,
            (CellEnum::Unknown, CellEnum::Unknown) => true,
            _ => false,
        }
    }
}


fn check(board: Board) -> CheckResultEnum {
    let mut complete_flag = true;
    for (i_x, row) in board.0.iter().enumerate() {
        let i = i_x as i32;
        for (j_x, cell) in row.iter().enumerate() {
            let j = j_x as i32;
            match cell {
                Cell::Wall(wall) => {
                    match wall {
                        WallEnum::Wall => {
                            // 4方向を確認し、壁があれば矛盾
                            if i - 1 >= 0 {
                                if board.0[(i - 1) as usize][j as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if i + 1 < board.0.len() as i32 {
                                if board.0[(i + 1) as usize][j as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if j - 1 >= 0 {
                                if board.0[i as usize][(j - 1) as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                            if j + 1 < row.len() as i32 {
                                if board.0[i as usize][(j + 1) as usize] == Cell::Wall(WallEnum::Wall) {
                                    return CheckResultEnum::Invalid(CheckResultInvalidEnum::AdjacentWall(i, j));
                                }
                            }
                        }
                        WallEnum::Hint(dir, num) => {
                            if dir.clone() == DirectionEnum::None {
                                continue;
                            }
                            let vec = dir.to_vector();
                            let mut now = (i as i32, j as i32);
                            let mut wall_count: i64 = 0;
                            let mut unknown_count: i64 = 0;
                            // breakされるまで
                            loop {
                                now.0 += vec.0;
                                now.1 += vec.1;
                                if now.0 < 0 || now.0 >= board.0.len() as i32 || now.1 < 0 || now.1 >= row.len() as i32 {
                                    break;
                                }
                                if board.0[now.0 as usize][now.1 as usize] == Cell::Wall(WallEnum::Wall) {
                                    wall_count += 1;
                                    continue;
                                }
                                if board.0[now.0 as usize][now.1 as usize] == Cell::Unknown {
                                    unknown_count += 1;
                                    continue;
                                }
                            }
                            // 壁の数がすでにnumを超えていたら矛盾
                            if wall_count.clone() > *num {
                                return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                            }
                            // 壁の数壁を置けるマスの数の合計がnumを超えていたら矛盾
                            if wall_count.clone() + unknown_count.clone() < *num {
                                return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                            }
                        }
                    }
                }
                Cell::Space(one, two) => {
                    if let CheckResultEnum::Invalid(_) = check_direction_and_return_result(one, i, j, &board) {
                        return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                    }
                    if let CheckResultEnum::Invalid(_) = check_direction_and_return_result(two, i, j, &board) {
                        return CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j));
                    }
                }
                Cell::Unknown => {
                    complete_flag = false;
                }
            }
        }
    }

    // TODO 一つの線で繋がっていることのチェック
    if complete_flag {
        return CheckResultEnum::Complete;
    }
    return CheckResultEnum::Valid;
}

fn check_direction_and_continue(direction: &Option<DirectionEnum>, next: (i32, i32), target: (i32, i32)) -> bool {
    match direction {
        Some(dir) => {
            let vec = dir.to_vector();
            let next = (next.0 + vec.0, next.1 + vec.1);
            next == target
        }
        None => false,
    }
}

fn check_direction_and_return_result(direction: &Option<DirectionEnum>, i: i32, j: i32, board: &Board) -> CheckResultEnum {
    match direction {
        Some(now) => {
            let vec = now.to_vector();
            let next = (i + vec.0, j + vec.1);
            match &board.0[next.0 as usize][next.1 as usize] {
                Cell::Space(another_one, another_two) => {
                    if check_direction_and_continue(another_one, next, (i, j)) {
                        return CheckResultEnum::Valid;
                    }
                    if check_direction_and_continue(another_two, next, (i, j)) {
                        return CheckResultEnum::Valid;
                    }
                    CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j))
                }
                _ => CheckResultEnum::Invalid(CheckResultInvalidEnum::Hint(i, j)),
            }
        }
        None => CheckResultEnum::Valid,
    }
}