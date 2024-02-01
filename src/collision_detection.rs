use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, colliding_entities);
    }
}

fn colliding_entities(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity, transform, collider) in query.iter() {
        for (other_entity, other_transform, other_collider) in query.iter() {
            if entity != other_entity {
                let distance = transform
                    .translation()
                    .distance(other_transform.translation());
                if distance < collider.radius + other_collider.radius {
                    colliding_entities
                        .entry(entity)
                        .or_insert_with(Vec::new)
                        .push(other_entity);
                }
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter());
        }
    }
}
