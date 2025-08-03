use crate::{camera::Camera, prelude::BTerm};

pub trait SceneComp {
    fn render(&self, ctx: &mut BTerm, camera: &Camera);
}
