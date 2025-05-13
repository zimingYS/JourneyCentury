use bevy::prelude::*;
use JourneyCentury::init::InitPlugin;
use JourneyCentury::player::PlayerPlugin;
use JourneyCentury::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(InitPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin)
        .run();
}