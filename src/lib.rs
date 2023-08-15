mod local_state;

use std::ops::DerefMut;
use extism_pdk::*;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use serde_json::from_str;
use noise::{NoiseFn, Perlin};
use matricks_plugin::{PluginUpdate, MatrixConfiguration};

use crate::local_state::LocalState;

lazy_static! {
    static ref CONFIG: Arc<Mutex<MatrixConfiguration>> = Arc::new(Mutex::new(MatrixConfiguration::default()));
    static ref LOCAL: Arc<Mutex<LocalState>> = Arc::new(Mutex::new(LocalState::new_rand_seeds()));
}

const Z_STEP: f64 = 0.01;

#[plugin_fn]
pub fn setup(mat_config_json: String) -> FnResult<()> {
    let mut config = CONFIG.lock().unwrap();
    let config = config.deref_mut();

    *config = from_str(&*mat_config_json).expect("unable to deserialize json!");

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
    let config = CONFIG.lock().unwrap();
    let mut local = LOCAL.lock().unwrap();

    let mut state: Vec<Vec<[u8; 4]>> = vec![];

    let perlin_red = Perlin::new(local.red_seed);
    let perlin_green = Perlin::new(local.green_seed);
    let perlin_blue = Perlin::new(local.blue_seed);

    for y in 0..config.height {
        state.push(vec![]);
        for x in 0..config.width {
            let red_val: u8 = (perlin_red.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;
            let green_val: u8 = (perlin_green.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;
            let blue_val: u8 = (perlin_blue.get([x as f64 / 10.0, y as f64 / 10.0, local.z_value]) * 255.0) as u8;

            state[y].push([blue_val, green_val, red_val, 0]);
        }
    }

    local.z_value += Z_STEP;

    Ok(Json(PluginUpdate {
        state,
        ..Default::default()
    }))
}