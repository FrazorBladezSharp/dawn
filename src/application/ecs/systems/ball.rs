use crate::{application::{dusk_api::create_aabb, ecs::components::*}, applicationinfo::*};
use dusk::physx::aabb_collision::AABoundingBox;
use specs::Join;

pub fn detect_window_boarder(
    ball_object: String,
    names: &specs::Storage<
        '_,
        CName,
        specs::shred::Fetch<'_, specs::storage::MaskedStorage<CName>>,
    >,
    rectangles: &mut specs::Storage<
        '_,
        CRectangle,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CRectangle>>,
    >,
    velocities: &mut specs::Storage<
        '_,
        CVelocity,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CVelocity>>,
    >,
) {
    for (name, rect, vel) in (names, rectangles, velocities).join() {
        if name.name == ball_object {
            // detect if a collision occurs
            let x = -((rect.rectangle.left <= 0.0) as i32)
                + ((rect.rectangle.left + rect.rectangle.width >= WINDOW_WIDTH as f32) as i32);
            let y = -((rect.rectangle.top <= 0.0) as i32)
                + ((rect.rectangle.top + rect.rectangle.height >= WINDOW_HEIGHT as f32) as i32);

            // resolve when collision detected : bounce back
            vel.x *= ((x == 0) as i32 - (x != 0) as i32) as f32;
            vel.y *= ((y == 0) as i32 - (y != 0) as i32) as f32;
        }
    }
}

pub fn ball_bat_collision(
    names: &specs::Storage<
        '_,
        CName,
        specs::shred::Fetch<'_, specs::storage::MaskedStorage<CName>>,
    >,
    rectangles: &mut specs::Storage<
        '_,
        CRectangle,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CRectangle>>,
    >,
    velocities: &mut specs::Storage<
        '_,
        CVelocity,
        specs::shred::FetchMut<'_, specs::storage::MaskedStorage<CVelocity>>,
    >,
) {
    // variables
    let mut ball_aabb = AABoundingBox::default();
    let mut bat_aabb = AABoundingBox::default();
    let mut ball_cvel = &mut CVelocity::default();

    // TODO: remove variables and data gathering
    //  :   so we can pass the correct variables to this function
    // gather data
    for (name, rect, vel) in (names, rectangles, velocities).join() {
        // setup the bounding boxes
        if name.name == "Ball" {
            ball_aabb = create_aabb(rect);
            ball_cvel = vel;
        }

        if name.name == "Bat" {
            bat_aabb = create_aabb(rect);
        }
    }

    // check collision
    let result = ball_aabb.detect_collision(&bat_aabb);

    // TODO: resolve the ball movement
    //  : we need to stop the ball being caught inside the bat
    //  : ie rest ball position ouside of the bat
    ball_cvel.y *= result;
    
}

