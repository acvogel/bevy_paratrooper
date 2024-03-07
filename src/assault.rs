use crate::gun::GunBase;
use crate::paratrooper::{Paratrooper, ParatrooperState};
use crate::{AppState, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PARATROOPER_WALK_SPEED: f32 = 50.;

/// Turn a lander into an assaulter
fn enable_assault_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Paratrooper, &mut CollisionGroups)>,
    mut event_reader: EventReader<LandingEvent>,
) {
    for event in event_reader.read() {
        if let Ok((entity, mut paratrooper, mut col_groups)) = query.get_mut(event.0) {
            col_groups.memberships = Group::GROUP_1;
            col_groups.filters = Group::GROUP_1 | Group::GROUP_4;
            paratrooper.state = ParatrooperState::Assault;
            commands.entity(entity).remove::<Sensor>();
        }
    }
}

/// Walk towards the gun
fn assault_movement_system(mut query: Query<(&Paratrooper, &Transform, &mut Velocity)>) {
    for (_paratrooper, transform, mut velocity) in query
        .iter_mut()
        .filter(|(p, _, _)| p.state == ParatrooperState::Assault)
    {
        // Move towards gun.
        let heading = -1.0 * transform.translation.x.signum();
        //velocity.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
        velocity.linvel.x = heading * PARATROOPER_WALK_SPEED;
    }
}

/// Jump paratrooper further from gun when colliding
fn assault_collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut paratroopers: Query<(&Paratrooper, &Transform, &mut Velocity)>,
    gun_base_query: Query<Entity, With<GunBase>>,
) {
    let gun_base_entity = gun_base_query.single();
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if gun_base_entity == *entity1 || gun_base_entity == *entity2 {
                info!("Gun base collision.");
            } else {
                // Paratrooper <-> Paratrooper
                if let Ok([(_p1, t1, v1), (_p2, t2, v2)]) =
                    paratroopers.get_many_mut([*entity1, *entity2])
                {
                    let x1 = t1.translation.x;
                    let x2 = t2.translation.x;
                    if (x1 - x2).abs() < 40. {
                        let mut velocity = if x1.abs() > x2.abs() { v1 } else { v2 };
                        velocity.linvel.y = 75.;
                    }
                }
            }
        }
    }
}

pub struct AssaultPlugin;

impl Plugin for AssaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enable_assault_system,
                assault_collision_system,
                assault_movement_system,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
