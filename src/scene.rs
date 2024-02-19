use std::error::Error;

use crate::window::Window;

pub type SceneResult = Option<Box<dyn Scene>>;

pub trait SceneRet {
    fn scene(self) -> SceneResult;
}

impl<T: Scene + 'static> SceneRet for T {
    fn scene(self) -> SceneResult {
        Some(Box::new(self))
    }
}

pub trait Scene {
    fn run(&mut self, window: &mut Window) -> Result<SceneResult, Box<dyn Error>>;
}
