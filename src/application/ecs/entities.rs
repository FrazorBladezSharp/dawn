use crate::applicationinfo::*;
use super::components::*;
use sfml::graphics::Rect;
use specs::{Builder, Entity, World, WorldExt};

pub fn create_entities(app_world: &mut World) {
    // call init with each entities data array
    init(app_world, String::from("Bat"), BAT_DATA);
    init(app_world, String::from("Ball"), BALL_DATA);

    /* How to access a component from:  let player: Entity = init( ...vars );

    let temp = app_world.read_storage::<CName>();
    let player_name = temp.get(player).unwrap();
    println!("Players name: {}", player_name.name);

    */ 
}

fn init(app_world: &mut World, name: String, data: [f32; 6]) -> Entity {
    app_world
        .create_entity()
        .with(CName {
            name,
        })
        .with(CRectangle {
            rectangle: Rect::new(data[0], data[1], data[2], data[3]),
        })
        .with(CVelocity {
            x: data[4],
            y: data[5],
        })
        .build()
}

/*
how to get a specific component from an entity?

entities are used as keys in a component storage lookup. example:

world.read_storage::<Component>().get(entity)
*/