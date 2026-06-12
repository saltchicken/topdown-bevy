use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer, Default, Clone, Copy, Debug)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Interactable,
    Enemy,
}
