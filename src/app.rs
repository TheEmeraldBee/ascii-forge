use std::error::Error;

use crate::{
    prelude::{handle_panics, Window},
    scene::SceneRet,
};

pub fn app(scene: impl SceneRet) -> Result<(), Box<dyn Error>> {
    let mut window = Window::init()?;
    handle_panics();
    let mut scene = scene.scene().unwrap();

    while let Some(next_scene) = scene.run(&mut window)? {
        scene = next_scene;
    }

    Ok(window.restore()?)
}
