use crate::gun::GunBase;
use crate::menu::AttackState;
use crate::paratrooper::{Paratrooper, ParatrooperState};
use crate::{AppState, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PARATROOPER_WALK_SPEED: f32 = 50.;
const PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP: u32 = 0b1001;
const PARATROOPER_ASSAULT_COLLISION_FILTER: u32 = 0b1001;

/// Turn a lander into an assaulter
fn enable_assault_system(
    mut query: Query<(Entity, &mut Paratrooper, &mut CollisionGroups, &mut Sensor)>,
    mut event_reader: EventReader<LandingEvent>,
) {
    for event in event_reader.iter() {
        if let Result::Ok((entity, mut paratrooper, mut col_groups, mut sensor)) =
            query.get_mut(event.0)
        {
            info!("New assault trooper landed {:?}.", entity);
            col_groups.memberships = PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP;
            col_groups.filters = PARATROOPER_ASSAULT_COLLISION_FILTER;
            paratrooper.state = ParatrooperState::Assault;
            sensor.0 = false;
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
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            if gun_base_entity == *entity1 || gun_base_entity == *entity2 {
                info!("Gun base collision.");
            } else {
                // Paratrooper <-> Paratrooper
                if let Ok([(_p1, t1, v1), (_p2, t2, v2)]) =
                    paratroopers.get_many_mut([*entity1, *entity2])
                {
                    info!("Paratrooper collision.");
                    let x1 = t1.translation.x;
                    let x2 = t2.translation.x;
                    if (x1 - x2).abs() < 40. {
                        info!("Jumping.");
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
        app.add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Ground))
                .with_system(enable_assault_system)
                .with_system(assault_collision_system)
                .with_system(assault_movement_system),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Air))
                .with_system(enable_assault_system)
                .with_system(assault_collision_system)
                .with_system(assault_movement_system),
        );
    }
}
