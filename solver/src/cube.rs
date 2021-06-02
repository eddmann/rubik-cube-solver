use rand::prelude::SliceRandom;

use crate::moves::Move;

pub(crate) trait Cube: Sized + Default {
    fn apply_move(self, action: &Move) -> Self;

    fn apply_moves(self, actions: &Vec<Move>) -> Self {
        actions
            .iter()
            .fold(self, |cube, action| cube.apply_move(action))
    }

    fn random(total_moves: u8) -> Self {
        let mut rng = rand::thread_rng();
        (0..total_moves).fold(Self::default(), |cube, _| {
            cube.apply_move(&Move::available().choose(&mut rng).unwrap())
        })
    }
}
