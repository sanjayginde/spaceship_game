use bevy::{prelude::*, utils::HashMap};

use crate::{asteroids::Asteroid, schedule::InGameSet, spaceship::Spaceship};

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
        app.add_systems(
            Update,
            colliding_entities.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            (
                handle_collisions::<Asteroid>,
                handle_collisions::<Spaceship>,
            )
                .in_set(InGameSet::DespawnEntities),
        );
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

fn handle_collisions<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid collided with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }

            // Despawn the asteroid.
            commands.entity(entity).despawn_recursive();
        }
    }
}
