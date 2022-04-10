use crate::gun::GunBase;
use crate::menu::AttackState;
use crate::paratrooper::{Paratrooper, ParatrooperState, PARATROOPER_COLLISION_FILTER};
use crate::{entities_collision_started, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// assault start makes a Base, kicks off. Set AssaultState base
// collision gunbase <-> Base fires, set to static. assign a climber (next closest) and start them walking
// collision gunbase <-> Climber sets AssaultPhase SecondBase. assign closest dude to SecondBase. start walking
// base <-> climber system jumps climber, if assaultphase is climber. if climber "above" base, just left instead of jump. if assaultphase SecondBase, set climber to static.
// SB <-> Base collision: SB static. AssaultPhase == Sapper. Tag and start sapper.
// AssaultEvents makes sense. Should almost make a new assault.rs. ? holds structs etc.

const PARATROOPER_WALK_SPEED: f32 = 50.;
const PARATROOPER_ASSAULT_MIN: usize = 4;
const PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP: u32 = 0b1001;

//#[derive(Clone, PartialEq)]
//enum AssaultRole {
//    Base,
//    Climber,
//    SecondBase,
//    Sapper,
//}
//
//#[derive(PartialEq)]
//enum JobStatus {
//    Waiting,
//    InProgress,
//    Completed,
//}

//#[derive(Component)]
//pub struct Assaulter {
//    role: AssaultRole,
//    status: JobStatus,
//}

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
    if *assault_state == AssaultState::Base {
        let (rb_pos, mut rb_vel, mut col_flags) = query.single_mut();
        if assault_state.is_changed() {
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
        }
        let heading = -1.0 * rb_pos.position.translation.x.signum();
        rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
    }
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
    if *assault_state == AssaultState::SecondBase {
        if assault_state.is_changed() {
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
        }
        let heading = -1.0 * rb_pos.position.translation.x.signum();
        rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
    }
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
    if *assault_state == AssaultState::Climber {
        if assault_state.is_changed() {
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
            let heading = -1.0 * rb_pos.position.translation.x.signum();
            rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
        }
        // "walk" again if above Base
        if rb_pos.position.translation.y > base_rb_pos.position.translation.y + 20.0 {
            let heading = -1.0 * rb_pos.position.translation.x.signum();
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
        }
        let heading = -1.0 * rb_pos.position.translation.x.signum();
        rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
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
    let gun_base_entity = gun_base_query.get_single().unwrap();
    let (base_entity, mut rb_type, mut rb_vel) = base_query.get_single_mut().unwrap();
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
    let gun_base_entity = gun_base_query.get_single().unwrap();
    let (climber_entity, mut rb_vel) = climber_query.get_single_mut().unwrap();
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
    let (climber_entity, rb_pos, mut rb_vel, mut rb_type) = climber_query.get_single_mut().unwrap();
    let base_entity = base_query.get_single().unwrap();
    for contact_event in contact_events.iter() {
        if entities_collision_started(*contact_event, base_entity, climber_entity) {
            if *assault_state == AssaultState::Climber {
                info!("Climber jump over Base.");
                let heading = rb_pos.position.translation.x.signum();
                rb_vel.linvel = Vec2::new(heading * 3., 80.).into();
            } else {
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
        let (sb_entity, mut rb_type) = second_base_query.get_single_mut().unwrap();
        let base_entity = base_query.get_single().unwrap();
        for contact_event in contact_events.iter() {
            if entities_collision_started(*contact_event, base_entity, sb_entity) {
                info!("SecondBase arrived at Base.");
                *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
                *assault_state = AssaultState::Sapper;
            }
        }
    }
}

/// Listen to changes in AssaultState resource. Tag the closest next paratrooper.
fn assault_state_listener(assault_state: Res<AssaultState>) {
    if assault_state.is_changed() || assault_state.is_added() {
        match *assault_state {
            AssaultState::Base => (),
            AssaultState::Climber => (),
            AssaultState::SecondBase => (),
            AssaultState::Sapper => (),
        }
    }
}

///// Paratrooper <-> Paratrooper collisions
//fn assault_paratrooper_collision_system(
//    mut contact_events: EventReader<ContactEvent>,
//    mut query: Query<(
//        Entity,
//        &mut Assaulter,
//        &RigidBodyPositionComponent,
//        &mut RigidBodyVelocityComponent,
//    )>,
//) {
//    let base = query
//        .iter()
//        .filter(|(_e, assaulter, _rb_pos, _rb_vel)| assaulter.role == AssaultRole::Base)
//        .next()
//        .unwrap();
//    //let mut sapper = query
//    //    .iter_mut()
//    //    .filter(|(_e, assaulter, _rb_pos, _rb_vel)| assaulter.role == AssaultRole::Sapper)
//    //    .next()
//    //    .unwrap();
//
//    for contact_event in contact_events.iter() {
//        if let Started(handle1, handle2) = contact_event {
//            info!(
//                "Paratrooper collision event started! {:?} {:?}",
//                handle1, handle2
//            );
//            {
//                let mut climber = query
//                    .iter_mut()
//                    .filter(|(_e, assaulter, _rb_pos, _rb_vel)| {
//                        assaulter.role == AssaultRole::Climber
//                    })
//                    .next()
//                    .unwrap();
//                // Base <-> Climber contact
//                if handle1.entity() == climber.0 && handle2.entity() == base.0
//                    || handle2.entity() == climber.0 && handle1.entity() == base.0
//                {
//                    match climber.1.status {
//                        JobStatus::InProgress => {
//                            // Jump in direction of Base
//                            let multiplier = (base.2.position.translation.x
//                                - climber.2.position.translation.x)
//                                .signum();
//                            climber.3.linvel = Vec2::new(multiplier * 3., 80.).into();
//                        }
//                        JobStatus::Completed => (),
//                        _ => (),
//                    }
//                }
//            }
//
//            let mut second_base = query
//                .iter_mut()
//                .filter(|(_e, assaulter, _rb_pos, _rb_vel)| {
//                    assaulter.role == AssaultRole::SecondBase
//                })
//                .next()
//                .unwrap();
//
//            // Looking for "InProgress" paratrooper
//            // If base, nothing
//            // If Climber <-> Base then climb/jump
//            // if SecondBase <-> Base, then done with that one.
//            // if Sapper, <-> SecondBase/Climber
//        }
//    }
//}

///// Handles pyramid building progress
//fn assault_collision_system(
//    mut contact_events: EventReader<ContactEvent>,
//    gun_base_query: Query<Entity, With<GunBase>>,
//    mut paratrooper_query: Query<(
//        Entity,
//        &mut Assaulter,
//        &mut RigidBodyTypeComponent,
//        &mut RigidBodyVelocityComponent,
//    )>,
//) {
//    let gun_base_entity = gun_base_query.get_single().unwrap();
//    for contact_event in contact_events.iter() {
//        if let Started(handle1, handle2) = contact_event {
//            // Gun base collision
//            if handle1.entity() == gun_base_entity || handle2.entity() == gun_base_entity {
//                let other_entity = if handle1.entity() == gun_base_entity {
//                    handle2.entity()
//                } else {
//                    handle1.entity()
//                };
//                for (paratrooper_entity, mut assaulter, mut rb_type, mut rb_vel) in
//                    paratrooper_query.iter_mut()
//                {
//                    if other_entity == paratrooper_entity {
//                        match *assaulter {
//                            Assaulter {
//                                role: AssaultRole::Base,
//                                status: JobStatus::InProgress,
//                            } => {
//                                info!("Assaulter Base is in place.");
//                                assaulter.status = JobStatus::Completed;
//                                *rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
//                            }
//                            Assaulter {
//                                role: AssaultRole::Climber,
//                                status: JobStatus::InProgress,
//                            } => {
//                                info!("Assaulter Climber is in place.");
//                                assaulter.status = JobStatus::Completed;
//                                // XXX gets stuck on the wall.
//                                // here's what we want. if climber colliders with base, jump if near y
//                                // if collision has climber > base, head left/right
//                                // or: teleport lol.
//                                //*rb_type = RigidBodyTypeComponent(RigidBodyType::Static);
//                                rb_vel.linvel = Vec2::ZERO.into();
//                            }
//                            //,Assaulter {
//                            //    role: AssaultRole::SecondBase,
//                            //    status: JobStatus::InProgress,
//                            //},
//                            Assaulter {
//                                role: AssaultRole::Sapper,
//                                status: JobStatus::InProgress,
//                            } => {
//                                info!("Assaulter Sapper has touched the base");
//                                // teleport to top of gun base, or jump
//                            }
//                            _ => (),
//                        }
//                    }
//                }
//            }
//        }
//    }
//}

/// Moves paratroopers in ground attack
//fn paratrooper_assault_system(
//    mut query: Query<(
//        &Paratrooper,
//        &mut Assaulter,
//        &mut RigidBodyTypeComponent,
//        &RigidBodyPositionComponent,
//        &mut RigidBodyVelocityComponent,
//    )>,
//) {
//    for (_paratrooper, mut assaulter, mut _rb_type, rb_pos, mut rb_vel) in query.iter_mut() {
//        let heading = -1.0 * rb_pos.position.translation.x.signum();
//        match *assaulter {
//            Assaulter {
//                role: AssaultRole::Base,
//                status: JobStatus::Waiting,
//            } => {
//                info!("Starting Base assault");
//                assaulter.status = JobStatus::InProgress;
//                rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
//                break;
//            }
//            Assaulter {
//                role: AssaultRole::Base,
//                status: JobStatus::InProgress,
//            } => {
//                break;
//            }
//            Assaulter {
//                role: AssaultRole::Climber,
//                status: JobStatus::Waiting,
//            } => {
//                info!("Starting Climber assault");
//                assaulter.status = JobStatus::InProgress;
//                rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
//                break;
//            }
//            Assaulter {
//                role: AssaultRole::Climber,
//                status: JobStatus::InProgress,
//            } => (),
//            Assaulter {
//                role: AssaultRole::SecondBase,
//                status: JobStatus::Waiting,
//            } => {
//                // start walking
//                assaulter.status = JobStatus::InProgress;
//                rb_vel.linvel = Vec2::new(heading * PARATROOPER_WALK_SPEED, 0.0).into();
//            }
//            _ => (),
//        }
//    }
//}

fn detect_assault_system_new(
    mut commands: Commands,
    mut paratrooper_query: Query<(
        Entity,
        &mut Paratrooper,
        &RigidBodyPositionComponent,
        &mut ColliderFlagsComponent,
    )>,
    gun_base_query: Query<&RigidBodyPositionComponent, With<GunBase>>,
    mut app_state: ResMut<State<AppState>>,
) {
    let gun_base_rb_pos = gun_base_query.get_single().unwrap();
    let mut landed_paratroopers: Vec<(
        Entity,
        Mut<'_, Paratrooper>,
        &RigidBodyPositionComponent,
        Mut<'_, ColliderFlagsComponent>,
    )> = paratrooper_query
        .iter_mut()
        .filter(|(_e, paratrooper, _rb_pos, _col_flags)| {
            paratrooper.state == ParatrooperState::Landed
        })
        .collect();

    let (landed_left_paratroopers, landed_right_paratroopers): (Vec<_>, Vec<_>) =
        landed_paratroopers
            .iter_mut()
            .partition(|(_e, _p, p_rb_pos, _col_flags)| {
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
        // Set troopers to Assault mode
        assault_troops.sort_by(
            |(_e1, _p1, rb_pos1, _col_flags1), (_e2, _p2, rb_pos2, _col_flags2)| {
                rb_pos1
                    .position
                    .translation
                    .x
                    .abs()
                    .partial_cmp(&rb_pos2.position.translation.x.abs())
                    .unwrap()
            },
        );

        let active_assault_troops = assault_troops.iter_mut().take(4);
        for ((entity, paratrooper, _rb_pos, col_flags), idx) in active_assault_troops.zip(0..4) {
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
            // TODO should be in "start" of other assault phases?
            col_flags.collision_groups = InteractionGroups::new(
                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
                PARATROOPER_COLLISION_FILTER,
            );
        }

        // Change game state
        app_state
            .set(AppState::InGame(AttackState::Ground))
            .unwrap();
    }
}

/// Waits for 4 landed paratroopers on one side of the gun
//fn detect_assault_system(
//    mut commands: Commands,
//    mut paratrooper_query: Query<(
//        Entity,
//        &mut Paratrooper,
//        &RigidBodyPositionComponent,
//        &mut ColliderFlagsComponent,
//    )>,
//    gun_base_query: Query<&RigidBodyPositionComponent, With<GunBase>>,
//    mut app_state: ResMut<State<AppState>>,
//) {
//    let gun_base_rb_pos = gun_base_query.get_single().unwrap();
//    let mut landed_paratroopers: Vec<(
//        Entity,
//        Mut<'_, Paratrooper>,
//        &RigidBodyPositionComponent,
//        Mut<'_, ColliderFlagsComponent>,
//    )> = paratrooper_query
//        .iter_mut()
//        .filter(|(_e, paratrooper, _rb_pos, _col_flags)| {
//            paratrooper.state == ParatrooperState::Landed
//        })
//        .collect();
//
//    let (landed_left_paratroopers, landed_right_paratroopers): (Vec<_>, Vec<_>) =
//        landed_paratroopers
//            .iter_mut()
//            .partition(|(_e, _p, p_rb_pos, _col_flags)| {
//                p_rb_pos.position.translation.x <= gun_base_rb_pos.position.translation.x
//            });
//
//    // Check if sufficient paratroopers on one side
//    let assault_troops = if landed_left_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
//        info!("left assault");
//        Some(landed_left_paratroopers)
//    } else if landed_right_paratroopers.len() >= PARATROOPER_ASSAULT_MIN {
//        info!("right assault");
//        Some(landed_right_paratroopers)
//    } else {
//        None
//    };
//
//    // TODO we could just switch to Assault game state and then have a generic paratrooper finding system to tag progress
//    // so we have a setup_assault function. it creates the AssaultState resource and tags the closest paratrooper.
//    // helper fn takes the array of paratroopers and returns the closest one. or we just tag them all upfront similar to now.
//
//    if let Some(mut assault_troops) = assault_troops {
//        info!("Assault!!!");
//        // Set troopers to Assault mode
//        assault_troops.sort_by(
//            |(_e1, _p1, rb_pos1, _col_flags1), (_e2, _p2, rb_pos2, _col_flags2)| {
//                rb_pos1
//                    .position
//                    .translation
//                    .x
//                    .abs()
//                    .partial_cmp(&rb_pos2.position.translation.x.abs())
//                    .unwrap()
//            },
//        );
//
//        //let x = vec![Base, Climber, SecondBase];
//        // hrm how do we loop like that. sort next()?
//        let roles = vec![
//            AssaultRole::Base,
//            AssaultRole::Climber,
//            AssaultRole::SecondBase,
//            AssaultRole::Sapper,
//        ];
//        let active_assault_troops = assault_troops.iter_mut().take(roles.len());
//        for ((entity, paratrooper, _rb_pos, col_flags), role) in active_assault_troops.zip(roles) {
//            // Update paratrooper state
//            paratrooper.state = ParatrooperState::Assault;
//            commands.entity(*entity).insert(Assaulter {
//                role: role,
//                status: JobStatus::Waiting,
//            });
//            // Enable collider with other paratroopers for climbing
//            // TODO should be in "start" of other assault phases?
//            col_flags.collision_groups = InteractionGroups::new(
//                PARATROOPER_ASSAULT_COLLISION_MEMBERSHIP,
//                PARATROOPER_COLLISION_FILTER,
//            );
//        }
//
//        // Change game state
//        app_state
//            .set(AppState::InGame(AttackState::Ground))
//            .unwrap();
//    }
//}

/// Create AssaultState resource
fn setup_ground_assault(mut commands: Commands) {
    commands.insert_resource(AssaultState::Base);
}

pub struct AssaultPlugin;

impl Plugin for AssaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Air))
                .with_system(detect_assault_system_new), //    .with_system(detect_assault_system),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame(AttackState::Ground))
                .with_system(setup_ground_assault),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(AttackState::Ground))
                .with_system(assault_state_listener)
                .with_system(base_assault)
                .with_system(base_gun_base_collision)
                .with_system(climber_assault)
                .with_system(climber_gun_base_collision)
                .with_system(climber_base_collision)
                .with_system(second_base_assault)
                .with_system(second_base_base_collision)
                .with_system(sapper_assault)
                //.with_system(paratrooper_assault_system)
                //.with_system(assault_collision_system), //.with_system(assault_paratrooper_collision_system),
        );
    }
}
