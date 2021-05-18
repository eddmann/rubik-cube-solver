use crate::cube::{Cube, Move};
use std::collections::{HashMap, VecDeque};

// 11 moves using half-turn metric,
// or 14 using the quarter-turn metric
const GODS_NUMBER: usize = 11;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

fn build_move_seq(history: HashMap<(Cube, Direction), Vec<Move>>, state: &Cube) -> Vec<Move> {
    let mut moves = history.get(&(*state, Direction::Forward)).unwrap().clone();

    let mut ending: Vec<Move> = history
        .get(&(*state, Direction::Backward))
        .unwrap()
        .iter()
        .map(|m| m.inverse())
        .collect::<Vec<_>>();
    ending.reverse();

    moves.append(&mut ending);
    moves
}

pub fn solve(cube: &Cube) -> Option<Vec<Move>> {
    if *cube == Cube::solved() {
        return Some(vec![]);
    }

    let mut history: HashMap<(Cube, Direction), Vec<Move>> = HashMap::new();
    let mut queue: VecDeque<(Cube, Direction)> = VecDeque::new();

    history.insert((*cube, Direction::Forward), vec![]);
    history.insert((Cube::solved(), Direction::Backward), vec![]);

    queue.push_back((*cube, Direction::Forward));
    queue.push_back((Cube::solved(), Direction::Backward));

    while let Some((state, direction)) = queue.pop_front() {
        for (action, next_state) in &state.next_states() {
            if history.contains_key(&(*next_state, direction)) {
                continue;
            }

            let mut state_moves: Vec<Move> = history.get(&(state, direction)).unwrap().clone();
            state_moves.push(*action);

            if state_moves.len() > GODS_NUMBER / 2 {
                continue;
            }

            history.insert((*next_state, direction), state_moves);

            if history.contains_key(&(*next_state, direction.opposite())) {
                return Some(build_move_seq(history, next_state));
            }

            queue.push_back((*next_state, direction));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_random_cube() {
        let cube = Cube::random(15);
        let solution = solve(&cube);
        assert_eq!(Cube::solved(), cube.apply_moves(solution.unwrap()));
    }
}
