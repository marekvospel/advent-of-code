use std::rc::Rc;

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let pieces: Vec<Vec<Piece>> = input
            .trim_end()
            .lines()
            .map(|l| l.chars().map(Piece::parse).collect())
            .collect();

        let pieces = Rc::new(pieces);

        let mut start_x = 0;
        let mut start_y = 0;

        for (row_i, row) in pieces.iter().enumerate() {
            for (col_i, piece) in row.iter().enumerate() {
                if let Piece::Start = piece {
                    start_y = row_i;
                    start_x = col_i;
                }
            }
        }

        let (start_x, start_y, direction) = find_matching(start_x, start_y, &pieces);

        let node = PieceNode {
            map: pieces.clone(),
            x: start_x,
            y: start_y,
            from: direction,
        };
        let iter = node.into_iter();
        let count = iter.count();

        let result = count / 2 + if count % 2 == 0 { 0 } else { 1 };

        Ok(result.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let pieces: Vec<Vec<Piece>> = input
            .trim_end()
            .lines()
            .map(|l| l.chars().map(Piece::parse).collect())
            .collect();

        let pieces = Rc::new(pieces);

        let mut start_x = 0;
        let mut start_y = 0;

        for (row_i, row) in pieces.iter().enumerate() {
            for (col_i, piece) in row.iter().enumerate() {
                if let Piece::Start = piece {
                    start_y = row_i;
                    start_x = col_i;
                }
            }
        }

        let (next_x, next_y, direction) = find_matching(start_x, start_y, &pieces);

        let node = PieceNode {
            map: pieces.clone(),
            x: next_x,
            y: next_y,
            from: direction,
        };
        let mut nodes: Vec<PieceNode> = node.clone().into_iter().collect();
        nodes.push(PieceNode {
            map: pieces.clone(),
            x: start_x,
            y: start_y,
            from: direction,
        });
        let mut res = 0;

        for row in 0..pieces.len() {
            let mut zone_start: Option<&PieceNode> = None;
            let mut inside = false;

            for x in 0..pieces[row].len() {
                let node = nodes.iter().find(|n| n.y == row && n.x == x);

                if inside && node.is_none() {
                    res += 1;
                }

                if zone_start.is_none() && node.is_some() {
                    zone_start = node;
                }

                // Start a new zone
                if zone_start.is_some()
                    && (node.is_none()
                        || !matches!(node.unwrap().piece(), Piece::LR | Piece::TR | Piece::RB))
                {
                    fn start_end(start: &Piece, end: &Piece, pieces: &Vec<Vec<Piece>>) -> bool {
                        match if matches!(start, Piece::Start) {
                            start.find_starting(pieces)
                        } else {
                            *start
                        } {
                            Piece::TL | Piece::TR => matches!(end, Piece::RB | Piece::LB),
                            Piece::LB | Piece::RB => matches!(end, Piece::TR | Piece::TL),
                            _ => false,
                        }
                    }

                    if node.is_some()
                        && start_end(
                            &node.unwrap().piece(),
                            &zone_start.unwrap().piece(),
                            &pieces,
                        )
                    {
                        inside = !inside;
                    }
                    zone_start = None;
                }
            }
        }

        Ok(res.to_string())
    }
}

fn find_matching(
    start_x: usize,
    start_y: usize,
    pieces: &Vec<Vec<Piece>>,
) -> (usize, usize, Direction) {
    if start_x > 1
        && let Some(Some(x)) = pieces.get(start_y).map(|p| p.get(start_x - 1))
        && x.works_from(Direction::Right)
    {
        return (start_x - 1, start_y, Direction::Right);
    }

    if start_y > 1
        && let Some(Some(x)) = pieces.get(start_y - 1).map(|p| p.get(start_x))
        && x.works_from(Direction::Bottom)
    {
        return (start_x, start_y - 1, Direction::Bottom);
    }

    if let Some(Some(x)) = pieces.get(start_y + 1).map(|p| p.get(start_x))
        && x.works_from(Direction::Top)
    {
        return (start_x, start_y + 1, Direction::Top);
    }

    if let Some(Some(x)) = pieces.get(start_y).map(|p| p.get(start_x + 1))
        && x.works_from(Direction::Left)
    {
        return (start_x + 1, start_y, Direction::Left);
    }

    panic!(":(")
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    TB,
    LR,
    TL,
    TR,
    LB,
    RB,
    None,
    Start,
}

impl Piece {
    fn parse(input: char) -> Self {
        match input {
            'S' => Piece::Start,
            '-' => Piece::LR,
            '|' => Piece::TB,
            'L' => Piece::TR,
            'F' => Piece::RB,
            'J' => Piece::TL,
            '7' => Piece::LB,
            _ => Piece::None,
        }
    }

    fn works_from(&self, direction: Direction) -> bool {
        match self {
            Piece::LR => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    return true;
                }
            }
            Piece::TB => {
                if matches!(direction, Direction::Top | Direction::Bottom) {
                    return true;
                }
            }
            Piece::TL => {
                if matches!(direction, Direction::Top | Direction::Left) {
                    return true;
                }
            }
            Piece::TR => {
                if matches!(direction, Direction::Top | Direction::Right) {
                    return true;
                }
            }
            Piece::LB => {
                if matches!(direction, Direction::Left | Direction::Bottom) {
                    return true;
                }
            }
            Piece::RB => {
                if matches!(direction, Direction::Right | Direction::Bottom) {
                    return true;
                }
            }
            Piece::Start => {
                return true;
            }
            Piece::None => {}
        }

        false
    }

    fn find_starting(&self, pieces: &Vec<Vec<Piece>>) -> Self {
        Self::TB
    }
}

#[derive(Debug, Clone)]
struct PieceNode {
    x: usize,
    y: usize,
    from: Direction,
    map: Rc<Vec<Vec<Piece>>>,
}

impl PieceNode {
    fn piece(&self) -> Piece {
        self.map[self.y][self.x]
    }

    fn peak_next(&self) -> PieceNode {
        let piece = self.piece();

        let mut next_x = self.x;
        let mut next_y = self.y;
        let next_direction;

        match piece {
            Piece::LR => match self.from {
                Direction::Left => {
                    next_x += 1;
                    next_direction = Direction::Left;
                }
                Direction::Right => {
                    next_x -= 1;
                    next_direction = Direction::Right;
                }
                _ => panic!("Can't come into LR from {:?}", self.from),
            },
            Piece::TB => match self.from {
                Direction::Top => {
                    next_y += 1;
                    next_direction = Direction::Top;
                }
                Direction::Bottom => {
                    next_y -= 1;
                    next_direction = Direction::Bottom;
                }
                _ => panic!("Can't come into TB from {:?}", self.from),
            },
            Piece::TL => match self.from {
                Direction::Top => {
                    next_x -= 1;
                    next_direction = Direction::Right;
                }
                Direction::Left => {
                    next_y -= 1;
                    next_direction = Direction::Bottom;
                }
                _ => panic!("Can't come into TL from {:?}", self.from),
            },
            Piece::TR => match self.from {
                Direction::Top => {
                    next_x += 1;
                    next_direction = Direction::Left;
                }
                Direction::Right => {
                    next_y -= 1;
                    next_direction = Direction::Bottom;
                }
                _ => panic!("Can't come into TR from {:?}", self.from),
            },
            Piece::LB => match self.from {
                Direction::Left => {
                    next_y += 1;
                    next_direction = Direction::Top;
                }
                Direction::Bottom => {
                    next_x -= 1;
                    next_direction = Direction::Right;
                }
                _ => panic!("Can't come into LB from {:?}", self.from),
            },
            Piece::RB => match self.from {
                Direction::Right => {
                    next_y += 1;
                    next_direction = Direction::Top;
                }
                Direction::Bottom => {
                    next_x += 1;
                    next_direction = Direction::Left;
                }
                _ => panic!("Can't come into RB from {:?}", self.from),
            },
            _ => panic!("Can't get next node for {piece:?}"),
        }

        PieceNode {
            x: next_x,
            y: next_y,
            from: next_direction,
            map: self.map.clone(),
        }
    }
}

impl IntoIterator for PieceNode {
    type Item = PieceNode;
    type IntoIter = PieceNodeIter;

    fn into_iter(self) -> PieceNodeIter {
        PieceNodeIter {
            current: Some(self),
        }
    }
}

#[derive(Clone)]
struct PieceNodeIter {
    current: Option<PieceNode>,
}

impl Iterator for PieceNodeIter {
    type Item = PieceNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();

        if current.is_none() {
            return None;
        }
        let current = current.unwrap();
        let next_piece = current.peak_next();

        if let Piece::Start = next_piece.piece() {
            self.current = None;
        } else {
            self.current = Some(next_piece);
        }

        Some(current)
    }
}
