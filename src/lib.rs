mod local_state;

use extism_pdk::*;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use noise::{NoiseFn, Perlin};

use crate::local_state::LocalState;

lazy_static! {
    static ref LOCAL: Arc<Mutex<LocalState>> = Arc::new(Mutex::new(LocalState::new_rand_seeds()));
}

const Z_STEP: f64 = 0.01;

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // No need to do anything here!
    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // Pull the width and height of the matrix from the config
    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();

    // Make perlin noise for the red, green, and blue channels
    let mut local = LOCAL.lock().unwrap();
    let perlin_red = Perlin::new(local.red_seed);
    let perlin_green = Perlin::new(local.green_seed);
    let perlin_blue = Perlin::new(local.blue_seed);

    // Make a new matrix state
    let mut state: Vec<Vec<[u8; 4]>> = vec![];
    for y in 0..height {
        state.push(vec![]);
        for x in 0..width {
            let red_val: u8 = (perlin_red.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;
            let green_val: u8 = (perlin_green.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;
            let blue_val: u8 = (perlin_blue.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;

            state[y].push([blue_val, green_val, red_val, 0]);
        }
    }

    // Slightly change the Z value for the next iteration
    local.z_value += Z_STEP;

    // Return the new matrix state
    Ok(Json(Some(state)))
}