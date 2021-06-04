use std::collections::{HashMap, VecDeque};

use crate::cube::Cube;
use crate::cubie_cube::Corner::*;
use crate::cubie_cube::Edge::*;
use crate::cubie_cube::{Corner, CubieCube, Edge};
use crate::moves::Direction::*;
use crate::moves::Position::*;
use crate::moves::{Direction, Move};

type PochmannCube = [u32; 40];

fn to_pochmann_edge(e: Edge) -> u32 {
    match e {
        UF => 0,
        UR => 1,
        UB => 2,
        UL => 3,
        DF => 4,
        DR => 5,
        DB => 6,
        DL => 7,
        FR => 8,
        FL => 9,
        BR => 10,
        BL => 11,
    }
}

fn to_pochmann_corner(c: Corner) -> u32 {
    match c {
        URF => 12,
        UBR => 13,
        ULB => 14,
        UFL => 15,
        DFR => 16,
        DLF => 17,
        DBL => 18,
        DRB => 19,
    }
}

fn to_pochmann_cube(cube: CubieCube) -> PochmannCube {
    let mut pc = [0 as u32; 40];

    pc[0] = to_pochmann_edge(cube.ep[UF as usize]);
    pc[1] = to_pochmann_edge(cube.ep[UR as usize]);
    pc[2] = to_pochmann_edge(cube.ep[UB as usize]);
    pc[3] = to_pochmann_edge(cube.ep[UL as usize]);
    pc[4] = to_pochmann_edge(cube.ep[DF as usize]);
    pc[5] = to_pochmann_edge(cube.ep[DR as usize]);
    pc[6] = to_pochmann_edge(cube.ep[DB as usize]);
    pc[7] = to_pochmann_edge(cube.ep[DL as usize]);
    pc[8] = to_pochmann_edge(cube.ep[FR as usize]);
    pc[9] = to_pochmann_edge(cube.ep[FL as usize]);
    pc[10] = to_pochmann_edge(cube.ep[BR as usize]);
    pc[11] = to_pochmann_edge(cube.ep[BL as usize]);

    pc[12] = to_pochmann_corner(cube.cp[URF as usize]);
    pc[13] = to_pochmann_corner(cube.cp[UBR as usize]);
    pc[14] = to_pochmann_corner(cube.cp[ULB as usize]);
    pc[15] = to_pochmann_corner(cube.cp[UFL as usize]);
    pc[16] = to_pochmann_corner(cube.cp[DFR as usize]);
    pc[17] = to_pochmann_corner(cube.cp[DLF as usize]);
    pc[18] = to_pochmann_corner(cube.cp[DBL as usize]);
    pc[19] = to_pochmann_corner(cube.cp[DRB as usize]);

    pc[20] = cube.eo[UF as usize] as u32;
    pc[21] = cube.eo[UR as usize] as u32;
    pc[22] = cube.eo[UB as usize] as u32;
    pc[23] = cube.eo[UL as usize] as u32;
    pc[24] = cube.eo[DF as usize] as u32;
    pc[25] = cube.eo[DR as usize] as u32;
    pc[26] = cube.eo[DB as usize] as u32;
    pc[27] = cube.eo[DL as usize] as u32;
    pc[28] = cube.eo[FR as usize] as u32;
    pc[29] = cube.eo[FL as usize] as u32;
    pc[30] = cube.eo[BR as usize] as u32;
    pc[31] = cube.eo[BL as usize] as u32;

    pc[32] = cube.co[URF as usize] as u32;
    pc[33] = cube.co[UBR as usize] as u32;
    pc[34] = cube.co[ULB as usize] as u32;
    pc[35] = cube.co[UFL as usize] as u32;
    pc[36] = cube.co[DFR as usize] as u32;
    pc[37] = cube.co[DLF as usize] as u32;
    pc[38] = cube.co[DBL as usize] as u32;
    pc[39] = cube.co[DRB as usize] as u32;

    pc
}

fn to_phase_id(phase: Phase, cube: CubieCube) -> PochmannCube {
    let pc = to_pochmann_cube(cube);

    match phase {
        Phase::One => {
            let mut r = [0 as u32; 40];
            for idx in 0..12 {
                r[idx] = pc[20 + idx] as u32;
            }
            r
        }
        Phase::Two => {
            let mut r = [0 as u32; 40];
            for idx in 0..8 {
                r[idx] = pc[32 + idx] as u32;
            }
            for idx in 0..12 {
                r[0] |= (pc[idx] as u32 / 8) << idx;
            }
            r
        }
        Phase::Three => {
            let mut r = [0 as u32; 40];
            for idx in 0..12 {
                r[0] |= (if pc[idx] > 7 { 2 } else { pc[idx] as u32 & 1 }) << (2 * idx);
            }
            for idx in 0..8 {
                r[1] |= (((pc[idx + 12] as u32) - 12) & 5) << (3 * idx);
            }
            for i in 12..20 {
                for j in i + 1..20 {
                    r[2] ^= if pc[i] > pc[j] { 1 } else { 0 }
                }
            }
            r
        }
        Phase::Four => pc,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum QueueDirection {
    Forward,
    Backward,
}

impl QueueDirection {
    fn opposite(self) -> Self {
        match self {
            QueueDirection::Forward => QueueDirection::Backward,
            QueueDirection::Backward => QueueDirection::Forward,
        }
    }
}

fn build_move_seq(
    history: HashMap<(PochmannCube, QueueDirection), Vec<Move>>,
    state: &PochmannCube,
) -> Vec<Move> {
    let mut moves = history
        .get(&(*state, QueueDirection::Forward))
        .unwrap()
        .clone();

    let mut ending: Vec<Move> = history
        .get(&(*state, QueueDirection::Backward))
        .unwrap()
        .iter()
        .map(|m| m.inverse())
        .collect::<Vec<_>>();
    ending.reverse();

    moves.append(&mut ending);
    moves
}

#[derive(Copy, Clone)]
enum Phase {
    One,
    Two,
    Three,
    Four,
}

impl Phase {
    fn iterator() -> impl Iterator<Item = Phase> {
        [Phase::One, Phase::Two, Phase::Three, Phase::Four]
            .iter()
            .copied()
    }
}

fn get_permitted_moves(phase: Phase) -> Vec<Move> {
    match phase {
        Phase::One => vec![
            Move(Up, Normal),
            Move(Up, Prime),
            Move(Up, Half),
            Move(Down, Normal),
            Move(Down, Prime),
            Move(Down, Half),
            Move(Left, Normal),
            Move(Left, Prime),
            Move(Left, Half),
            Move(Right, Normal),
            Move(Right, Prime),
            Move(Right, Half),
            Move(Front, Normal),
            Move(Front, Prime),
            Move(Front, Half),
            Move(Back, Normal),
            Move(Back, Prime),
            Move(Back, Half),
        ],
        Phase::Two => vec![
            Move(Up, Normal),
            Move(Up, Prime),
            Move(Up, Half),
            Move(Down, Normal),
            Move(Down, Prime),
            Move(Down, Half),
            Move(Left, Normal),
            Move(Left, Prime),
            Move(Left, Half),
            Move(Right, Normal),
            Move(Right, Prime),
            Move(Right, Half),
            Move(Front, Half),
            Move(Back, Half),
        ],
        Phase::Three => vec![
            Move(Up, Normal),
            Move(Up, Prime),
            Move(Up, Half),
            Move(Down, Normal),
            Move(Down, Prime),
            Move(Down, Half),
            Move(Left, Half),
            Move(Right, Half),
            Move(Front, Half),
            Move(Back, Half),
        ],
        Phase::Four => vec![
            Move(Up, Half),
            Move(Down, Half),
            Move(Left, Half),
            Move(Right, Half),
            Move(Front, Half),
            Move(Back, Half),
        ],
    }
}

fn bidirection_bfs(phase: Phase, current_cube: CubieCube) -> Vec<Move> {
    let current_id = to_phase_id(phase, current_cube);
    let goal_id = to_phase_id(phase, CubieCube::default());

    if current_id == goal_id {
        return vec![];
    };

    let mut history: HashMap<(PochmannCube, QueueDirection), Vec<Move>> = HashMap::new();
    let mut queue: VecDeque<(CubieCube, QueueDirection)> = VecDeque::new();

    history.insert((current_id, QueueDirection::Forward), vec![]);
    history.insert((goal_id, QueueDirection::Backward), vec![]);

    queue.push_back((current_cube, QueueDirection::Forward));
    queue.push_back((CubieCube::default(), QueueDirection::Backward));

    while let Some((state, direction)) = queue.pop_front() {
        for action in get_permitted_moves(phase) {
            let next_state = state.apply_move(&action);
            let next_id = to_phase_id(phase, next_state);

            if history.contains_key(&(next_id, direction)) {
                continue;
            }

            let mut state_moves: Vec<Move> = history
                .get(&(to_phase_id(phase, state), direction))
                .unwrap()
                .clone();
            state_moves.push(action);

            history.insert((next_id, direction), state_moves);

            if history.contains_key(&(next_id, direction.opposite())) {
                return build_move_seq(history, &next_id);
            }

            queue.push_back((next_state, direction));
        }
    }

    vec![]
}

pub(crate) fn solve(cube: &CubieCube) -> Option<Vec<Move>> {
    let mut solution = vec![];

    for phase in Phase::iterator() {
        let phase_moves = bidirection_bfs(phase, cube.apply_moves(&solution));
        solution.extend(phase_moves);
    }

    Some(simplify_multi_face_moves(&solution))
}

fn simplify_multi_face_moves(solution: &Vec<Move>) -> Vec<Move> {
    fn to_quarter_turns(d: Direction) -> u8 {
        match d {
            Normal => 1,
            Prime => 3,
            Half => 2,
        }
    }

    fn to_direction(quarter_turns: u8) -> Option<Direction> {
        match quarter_turns % 4 {
            1 => Some(Normal),
            2 => Some(Half),
            3 => Some(Prime),
            _ => None,
        }
    }

    let mut simplified_solution: Vec<Move> = vec![];

    for &Move(position, direction) in solution.iter() {
        match simplified_solution.last() {
            Some(&Move(last_position, last_direction)) if position == last_position => {
                simplified_solution.pop();
                if let Some(new_direction) =
                    to_direction(to_quarter_turns(direction) + to_quarter_turns(last_direction))
                {
                    simplified_solution.push(Move(position, new_direction));
                }
            }
            _ => simplified_solution.push(Move(position, direction)),
        }
    }

    simplified_solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CubieCube;
    use crate::FaceletCube;

    #[test]
    fn solve_random_cube() {
        let cube = CubieCube::random(100);
        let solution = solve(&cube);
        assert_eq!(CubieCube::default(), cube.apply_moves(&solution.unwrap()));
    }

    #[test]
    fn solutions_do_not_have_multi_face_turns() {
        let cube = "OGOYWWWWYRBYRRRORRORBYGGWOBBWYBYYRWWWBBGOOGORGOGBBYGGY"
            .parse::<FaceletCube>()
            .unwrap();
        let solution = solve(&CubieCube::from(cube)).unwrap();

        assert_no_multi_face_turns(&solution);
        assert_eq!(FaceletCube::default(), cube.apply_moves(&solution));

        let cube = "BGYRWOYGOWYOWRRYYBOWGBGRBORYWGGYBRBWWYGBORWYRBOOGBORWG"
            .parse::<FaceletCube>()
            .unwrap();
        let solution = solve(&CubieCube::from(cube)).unwrap();

        assert_no_multi_face_turns(&solution);
        assert_eq!(FaceletCube::default(), cube.apply_moves(&solution));

        let cube = "BRGOWGWWBOOYRRBOGROOYOGYGRGYWWBYYRBBYBBGOYWWORGRYBWWRG"
            .parse::<FaceletCube>()
            .unwrap();
        let solution = solve(&CubieCube::from(cube)).unwrap();

        assert_no_multi_face_turns(&solution);
        assert_eq!(FaceletCube::default(), cube.apply_moves(&solution));
    }

    fn assert_no_multi_face_turns(solution: &Vec<Move>) {
        let has_multi_face_turn = solution
            .iter()
            .zip(solution.iter().skip(1))
            .any(|(a, b)| a.0 == b.0);

        assert!(!has_multi_face_turn);
    }
}
