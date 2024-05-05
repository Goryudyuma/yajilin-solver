use std::fmt;

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
