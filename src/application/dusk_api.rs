use dusk::physx::aabb_collision::AABoundingBox;
use super::ecs::components::CRectangle;

pub fn create_aabb(rect: &CRectangle) -> AABoundingBox {
    AABoundingBox::new(
        rect.rectangle.left,
        rect.rectangle.top,
        rect.rectangle.width,
        rect.rectangle.height,
    )
}
