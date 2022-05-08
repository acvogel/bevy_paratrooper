use crate::gun::GunBase;
use crate::menu::AttackState;
use crate::paratrooper::{Paratrooper, ParatrooperState, PARATROOPER_COLLISION_FILTER};
use crate::{entities_collision_started, AppState, LandingEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PARATROOPER_WALK_SPEED: f32 = 50.;
const PARATROOPER_ASSAULT_MIN: usize = 4;
const PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP: u32 = 0b1001;

/// Forms the base of the pyramid
#[derive(Component)]
pub struct Base;

/// Climbs on top of the Base
#[derive(Component)]
pub struct Climber;

/// Moves next to Base
#[derive(Component)]
pub struct SecondBase;

/// Climbs SecondBase, Climber, and walks to Gun
#[derive(Component)]
pub struct Sapper;

#[derive(Clone, PartialEq)]
enum AssaultState {
    Base,
    Climber,
    SecondBase,
    Sapper,
}

/// Walk towards the gun
fn base_assault(
    assault_state: Res<AssaultState>,
    mut query: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut ColliderFlagsComponent,
        ),
        With<Base>,
    >,
) {
    //if *assault_state == AssaultState::Base {
    let (rb_pos, mut rb_vel, mut col_flags) = query.single_mut();
    if assault_state.is_changed() {
        col_flags.collision_groups = InteractionGroups::new(
            PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
            PARATROOPER_COLLISION_FILTER,
        );
    }
    let heading = -1.0 * rb_pos.position.translation.x.signum();
    rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
    //}
}

/// Walk towards Base
fn second_base_assault(
    assault_state: Res<AssaultState>,
    mut query: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut ColliderFlagsComponent,
        ),
        With<SecondBase>,
    >,
) {
    let (rb_pos, mut rb_vel, mut col_flags) = query.single_mut();
    //if *assault_state == AssaultState::SecondBase {
    if assault_state.is_changed() {
        col_flags.collision_groups = InteractionGroups::new(
            PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
            PARATROOPER_COLLISION_FILTER,
        );
    }
    let heading = -1.0 * rb_pos.position.translation.x.signum();
    rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
    //}
}

/// Walk towards gun
fn climber_assault(
    assault_state: Res<AssaultState>,
    mut query: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut ColliderFlagsComponent,
        ),
        With<Climber>,
    >,
    mut base_query: Query<&RigidBodyPositionComponent, With<Base>>,
) {
    let (rb_pos, mut rb_vel, mut col_flags) = query.single_mut();
    let base_rb_pos = base_query.single_mut();
    let heading = -1.0 * rb_pos.position.translation.x.signum();
    if *assault_state == AssaultState::Climber {
        if assault_state.is_changed() {
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
            rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
        }
        // "walk" again if above Base
        if rb_pos.position.translation.y > base_rb_pos.position.translation.y + 20.0 {
            rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
        }
    }
}

/// Walk towards gun
fn sapper_assault(
    assault_state: Res<AssaultState>,
    mut query: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut ColliderFlagsComponent,
        ),
        With<Sapper>,
    >,
) {
    if *assault_state == AssaultState::Sapper {
        let (rb_pos, mut rb_vel, mut col_flags) = query.single_mut();
        if assault_state.is_changed() {
            info!("Starting sapper.");
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
            let heading = -1.0 * rb_pos.position.translation.x.signum();
            rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
        }
    }
}

/// Walk on ground initial velocity. when hit 2nd base from side, impulse jump up.
/// when hit 2nd base from top, walk left
/// then same on climber?
fn sapper_collision(
    mut contact_events: EventReader<ContactEvent>,
    assault_state: Res<AssaultState>,
    mut sapper_query: Query<
        (
            Entity,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Sapper>,
    >,
    sb_query: Query<(Entity, &RigidBodyPositionComponent), With<SecondBase>>,
    climber_query: Query<(Entity, &RigidBodyPositionComponent), With<Climber>>,
    gun_base_query: Query<(Entity, &RigidBodyPositionComponent), With<GunBase>>,
) {
    if *assault_state == AssaultState::Sapper {
        let (sapper_entity, sapper_rb_pos, mut sapper_rb_vel) = sapper_query.single_mut();
        let (sb_entity, sb_rb_pos) = sb_query.single();
        let (climber_entity, climber_rb_pos) = climber_query.single();
        let (gun_base_entity, gun_base_rb_pos) = gun_base_query.single();
        let heading = -1.0 * sapper_rb_pos.position.translation.x.signum();
        for contact_event in contact_events.iter() {
            // SecondBase
            if entities_collision_started(*contact_event, sb_entity, sapper_entity) {
                let diff = sapper_rb_pos.position.translation.y - sb_rb_pos.position.translation.y;
                if diff > 10. {
                    info!("Sapper walking on SecondBase. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 8., 90.).into();
                } else {
                    info!("Sapper jumping from SecondBase. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 6., 80.).into();
                }
            }

            // Climber
            if entities_collision_started(*contact_event, climber_entity, sapper_entity) {
                let diff =
                    sapper_rb_pos.position.translation.y - climber_rb_pos.position.translation.y;
                if diff > 10. {
                    info!("Sapper walking on Climber. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 20., 1.).into();
                } else {
                    info!("Sapper jumping impulse from Climber. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 3., 80.).into();
                }
            }

            if entities_collision_started(*contact_event, sapper_entity, gun_base_entity) {
                info!("Sapper <-> GunBase");
                let diff =
                    sapper_rb_pos.position.translation.y - gun_base_rb_pos.position.translation.y;
                if diff > 20. {
                    info!("Sapper walking on GunBase. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 15., 0.).into();
                } else {
                    // Jumpy
                    info!("Sapper jumping on GunBase. diff: {}", diff);
                    sapper_rb_vel.linvel = Vec2::new(heading * 5., 50.).into();
                }
            }
        }
    }
}

/// Lock Base in place after touching GunBase
fn base_gun_base_collision(
    mut assault_state: ResMut<AssaultState>,
    mut contact_events: EventReader<ContactEvent>,
    mut base_query: Query<
        (
            Entity,
            &mut RigidBodyTypeComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Base>,
    >,
    gun_base_query: Query<Entity, With<GunBase>>,
) {
    let gun_base_entity = gun_base_query.single();
    let (base_entity, mut rb_type, mut rb_vel) = base_query.single_mut();
    for contact_event in contact_events.iter() {
        if entities_collision_started(*contact_event, base_entity, gun_base_entity) {
            info!("Base arrived at GunBase.");
            rb_vel.linvel = Vec2::ZERO.into();
            *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
            *assault_state = AssaultState::Climber;
        }
    }
}

/// Climber phase is done after Climber touches GunBase
fn climber_gun_base_collision(
    mut assault_state: ResMut<AssaultState>,
    mut contact_events: EventReader<ContactEvent>,
    mut climber_query: Query<(Entity, &mut RigidBodyVelocityComponent), With<Climber>>,
    gun_base_query: Query<Entity, With<GunBase>>,
) {
    let gun_base_entity = gun_base_query.single();
    let (climber_entity, mut rb_vel) = climber_query.single_mut();
    for contact_event in contact_events.iter() {
        if entities_collision_started(*contact_event, gun_base_entity, climber_entity) {
            info!("Climber arrived at GunBase.");
            rb_vel.linvel.x = 0.;
            *assault_state = AssaultState::SecondBase;
        }
    }
}

/// Climber jumps after touching Base
fn climber_base_collision(
    assault_state: ResMut<AssaultState>,
    mut contact_events: EventReader<ContactEvent>,
    mut climber_query: Query<
        (
            Entity,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut RigidBodyTypeComponent,
        ),
        With<Climber>,
    >,
    base_query: Query<Entity, With<Base>>,
) {
    let (climber_entity, rb_pos, mut rb_vel, mut rb_type) = climber_query.single_mut();
    let base_entity = base_query.single();
    for contact_event in contact_events.iter() {
        if entities_collision_started(*contact_event, base_entity, climber_entity) {
            if *assault_state == AssaultState::Climber {
                info!("Climber jump over Base.");
                let heading = rb_pos.position.translation.x.signum();
                rb_vel.linvel = Vec2::new(heading * 5., 500.).into();
            } else if *assault_state == AssaultState::SecondBase
                || *assault_state == AssaultState::Sapper
            {
                info!("Climber locked in place.");
                *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
            }
        }
    }
}

/// SecondBase is done after colliding with Base
fn second_base_base_collision(
    mut assault_state: ResMut<AssaultState>,
    mut contact_events: EventReader<ContactEvent>,
    mut second_base_query: Query<(Entity, &mut RigidBodyTypeComponent), With<SecondBase>>,
    base_query: Query<Entity, With<Base>>,
) {
    if *assault_state == AssaultState::SecondBase {
        let (sb_entity, mut rb_type) = second_base_query.single_mut();
        let base_entity = base_query.single();
        for contact_event in contact_events.iter() {
            if entities_collision_started(*contact_event, base_entity, sb_entity) {
                info!("SecondBase arrived at Base.");
                *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
                *assault_state = AssaultState::Sapper;
            }
        }
    }
}

// XXX what about multiple landers hrm. could we just use the same "ai" for all of them.
// go towards gun. bump into guy towards gun, jump.
// simple_assault.rs or smthng? just write it in here. when landing event, turn on collider, set to Assault.
// then 2 collision systems: para/para: one farther from gun jumps, and only if within band of Y value. no "top" jumpers.
// "wind" blows towards gun, with possible like deadzone heights? hrm hrm.
// gun collision:

/// Turn a lander into an assaulter
fn enable_assault_system(
    //mut commands: Commands,
    mut query: Query<(Entity, &mut Paratrooper, &mut ColliderFlagsComponent)>,
    mut event_reader: EventReader<LandingEvent>,
) {
    for event in event_reader.iter() {
        if let Result::Ok((_entity, mut paratrooper, mut col_flags)) = query.get_mut(event.0) {
            info!("New assault trooper landed.");
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
            paratrooper.state = ParatrooperState::Assault;
        }
    }
}

fn assault_movement_system(
    mut query: Query<(
        &Paratrooper,
        &RigidBodyPositionComponent,
        &mut RigidBodyVelocityComponent,
    )>,
) {
    for (_paratrooper, rb_pos, mut rb_vel) in query
        .iter_mut()
        .filter(|(p, _, _)| p.state == ParatrooperState::Assault)
    {
        // Move towards gun.
        let heading = -1.0 * rb_pos.position.translation.x.signum();
        rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
    }
}

// XXX WIP
//fn assault_collision_system(
//    //mut assault_state: ResMut<AssaultState>,
//    mut contact_events: EventReader<ContactEvent>,
//    mut paratroopers: Query<(
//        Entity,
//        &Paratrooper,
//        &RigidBodyPositionComponent,
//        &mut RigidBodyVelocityComponent,
//    )>,
//    gun_base_query: Query<Entity, With<GunBase>>,
//) {
//    let gun_base_entity = gun_base_query.single();
//    for contact_event in contact_events.iter() {
//        if let ContactEvent::Started(handle1, handle2) = contact_event {
//            if gun_base_entity == handle1.entity() || gun_base_entity == handle2.entity() {
//                // gun base <-> paratrooper. maybe lock in.
//            } else {
//                if let Result::Ok((e1, p1, rb_pos_1, mut rb_vel_1)) =
//                    paratroopers.get_mut(handle1.entity())
//                {
//                    if let Result::Ok((e2, p2, rb_pos_2, mut rb_vel_2)) =
//                        paratroopers.get_mut(handle2.entity())
//                    {
//                        // if they are around same x, get the one with higher abs rb_pos
//                        let x1 = rb_pos_1.position.translation.x;
//                        let x2 = rb_pos_2.position.translation.x;
//                        if (x1 - x2).abs() < 10. {
//                            let rb_vel = if x1.abs() > x2.abs() {
//                                rb_vel_1
//                            } else {
//                                rb_vel_2
//                            };
//                            //let heading = x1.signum();
//                            //rb_vel.linvel = Vec2::new(heading * 5., 500.).into();
//                            rb_vel.linvel = Vec2::new(0., 500.).into();
//                        }
//                    }
//                }
//            }
//
//            // so can get query by entity.
//            //if let (entity, paratrooper, mut rb_vel) = paratroopers.get(handle1.entity())
//            let x = 5;
//        }
//    }
//    //
//    // no not quite right.
//    // question is do we get just 1 contact event. probably.
//    // if entities_collision_started(*contact_event, base_entity, climber_entity) {
//
//    //for (paratrooper)
//    //let (climber_entity, mut rb_vel) = climber_query.single_mut();
//}

/// Attempt 2.
/// When landed we assign?
/// LandingEvent listener!!
fn assign_assault_system(
    mut commands: Commands,
    query: Query<
        (Entity, &Paratrooper, &RigidBodyPositionComponent),
        (
            Without<Base>,
            Without<SecondBase>,
            Without<Climber>,
            Without<Sapper>,
        ),
    >,
    bases: Query<(&Paratrooper, &RigidBodyPositionComponent), With<Base>>,
    second_bases: Query<(&Paratrooper, &RigidBodyPositionComponent), With<SecondBase>>,
    climbers: Query<(&Paratrooper, &RigidBodyPositionComponent), With<Climber>>,
    sappers: Query<(&Paratrooper, &RigidBodyPositionComponent), With<Sapper>>,
    mut landing_event_reader: EventReader<LandingEvent>,
) {
    for event in landing_event_reader.iter() {
        // Lander
        if let Ok((entity, paratrooper, rb_pos)) = query.get(event.0) {
            // find next available Component on this side of gun.
            let assault_sign = rb_pos.position.translation.x.signum();
            if let Some((sapper, sapper_rb_pos)) = sappers
                .iter()
                .filter(|(_p, rb_pos)| rb_pos.position.translation.x.signum() == assault_sign)
                .next()
            {
                // This side assault already in progress. Do nothing.
            } else if let Some((second_base, second_base_rb_pos)) = second_bases
                .iter()
                .filter(|(_, rb_pos)| rb_pos.position.translation.x.signum() == assault_sign)
                .next()
            {
                // assign Sapper.
                commands.entity(entity).insert(Sapper);
            }

            // tag da trooper.
        }
    }
}

/// Detect when there are 4 paratroopers landed and launch assault
fn detect_assault_system(
    mut commands: Commands,
    mut paratrooper_query: Query<(Entity, &mut Paratrooper, &RigidBodyPositionComponent)>,
    gun_base_query: Query<&RigidBodyPositionComponent, With<GunBase>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let gun_base_rb_pos = gun_base_query.single();
    let mut landed_paratroopers: Vec<(Entity, Mut<'_, Paratrooper>, &RigidBodyPositionComponent)> =
        paratrooper_query
            .iter_mut()
            .filter(|(_e, paratrooper, _rb_pos)| {
                paratrooper.state == ParatrooperState::Landed && paratrooper.has_deployed_chute
            })
            .collect();

    let (landed_left_paratroopers, landed_right_paratroopers): (Vec<_>, Vec<_>) =
        landed_paratroopers
            .iter_mut()
            .partition(|(_e, _p, p_rb_pos)| {
                p_rb_pos.position.translation.x <= gun_base_rb_pos.position.translation.x
            });

    // Check if sufficient paratroopers on one side
    let assault_troops = if landed_left_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
        info!("left assault");
        Some(landed_left_paratroopers)
    } else if landed_right_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
        info!("right assault");
        Some(landed_right_paratroopers)
    } else {
        None
    };

    if let Some(mut assault_troops) = assault_troops {
        info!("Assault!!!");
        assault_troops.sort_by(|(_e1, _p1, rb_pos1), (_e2, _p2, rb_pos2)| {
            rb_pos1
                .position
                .translation
                .x
                .abs()
                .partial_cmp(&rb_pos2.position.translation.x.abs())
                .unwrap()
        });

        let active_assault_troops = assault_troops.iter_mut().take(4);
        for ((entity, paratrooper, _rb_pos), idx) in active_assault_troops.zip(0..4) {
            paratrooper.state = ParatrooperState::Assault;
            let mut e = commands.entity(*entity);
            match idx {
                0 => {
                    e.insert(Base);
                }

                1 => {
                    e.insert(Climber);
                }

                2 => {
                    e.insert(SecondBase);
                }

                3 => {
                    e.insert(Sapper);
                }
                _ => (),
            }
        }

        // Change game state
        app_state
            .set(AppState::InGame(AttackState::Ground))
            .unwrap();
    }
}

/// Create AssaultState resource
fn setup_ground_assault(mut commands: Commands) {
    commands.insert_resource(AssaultState::Base);
}

fn cleanup_assault_system(mut commands: Commands) {
    commands.remove_resource::<AssaultState>();
}

pub struct AssaultPlugin;

impl Plugin for AssaultPlugin {
    //fn build(&self, app: &mut App) {
    //    app.add_system_set(
    //        SystemSet::on_update(AppState::InGame(AttackState::Air))
    //            .with_system(detect_assault_system),
    //    )
    //    //.add_system_set(
    //    //    SystemSet::on_enter(AppState::InGame(AttackState::Ground))
    //    //        .with_system(setup_ground_assault),
    //    //)
    //    //.add_system_set(
    //    //    SystemSet::on_update(AppState::InGame(AttackState::Ground))
    //    //        .with_system(base_assault)
    //    //        .with_system(base_gun_base_collision)
    //    //        .with_system(climber_assault)
    //    //        .with_system(climber_gun_base_collision)
    //    //        .with_system(climber_base_collision)
    //    //        .with_system(second_base_assault)
    //    //        .with_system(second_base_base_collision)
    //    //        .with_system(sapper_assault)
    //    //        .with_system(sapper_collision),
    //    //)
    //    .add_system_set(
    //        SystemSet::on_exit(AppState::InGame(AttackState::Ground))
    //            .with_system(cleanup_assault_system),
    //    );
    //}

    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Air))
                .with_system(enable_assault_system)
                .with_system(assault_movement_system), //.with_system(assault_collision_system),
        );
    }
}
