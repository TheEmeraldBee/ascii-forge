use std::error::Error;

use crate::window::Window;

pub type SceneResult = Option<Box<dyn Scene>>;

/// No need to implement, only allows for Scenes to turn into Boxes of scenes, simplifying the workflow.
pub trait SceneRet {
    fn scene(self) -> SceneResult;
}

impl<T: Scene + 'static> SceneRet for T {
    fn scene(self) -> SceneResult {
        Some(Box::new(self))
    }
}

/// A scene, that will render everthing, and will either exit the game, or return a new scene to transition to.
pub trait Scene {
    fn run(&mut self, window: &mut Window) -> Result<SceneResult, Box<dyn Error>>;
}
