use super::*;

pub struct ExitSystem {}
impl<'a> System<'a> for ExitSystem {
    type SystemData = (Read<'a, Key>, Write<'a, Quit>);

    fn run(&mut self, data: Self::SystemData) {
        let (key, mut quit) = data;

        if key.0.is_some_and(|key| *key == VirtualKeyCode::Escape) {
            quit.0 = true;
        }
    }
}
