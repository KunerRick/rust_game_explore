use crate::prelude::BTerm;

pub trait SceneComp {
    fn render(&self, ctx: &mut BTerm);
}
