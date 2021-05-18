mod cube;
mod solve;

use crate::cube::{Cube, Move};
use crate::solve::solve;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn rand_cube() -> String {
    Cube::random(15).to_string()
}

#[wasm_bindgen]
pub fn solve_cube(cube: String) -> Result<js_sys::Array, JsValue> {
    match solve(&cube.parse()?) {
        Some(solution) => Ok(solution
            .into_iter()
            .map(|mv| JsValue::from_str(&format!("{}", mv)))
            .collect::<js_sys::Array>()),
        None => Err(JsValue::from_str("Cube is unsolveable")),
    }
}

#[wasm_bindgen]
pub fn apply_cube_moves(cube: String, moves: js_sys::Array) -> Result<String, JsValue> {
    let actions: Vec<Move> = moves
        .iter()
        .map(|mv| (mv.as_string().unwrap_or("".to_string())).parse())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(cube.parse::<Cube>()?.apply_moves(actions).to_string())
}
