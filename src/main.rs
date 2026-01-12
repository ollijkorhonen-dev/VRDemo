use bevy::prelude::*;
use vr_demo::VrDemoPlugin;

fn main() {
    App::new()
        .add_plugins(VrDemoPlugin)
        .run();
}
