use std::ops::Not;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};

use crate::data::{
    physics::ComplexExternalForce,
    player::{Player, PlayerInfo, PlayerJump, PlayerMovement},
    web::Web,
};

use super::{PlayerControl, PlayerEvent};

pub fn handle_movement(
    input: Res<Input<KeyCode>>,
    control: Res<PlayerControl>,
    mut query: Query<
        (
            &mut PlayerMovement,
            &PlayerInfo,
            &mut Velocity,
            &mut ComplexExternalForce,
        ),
        With<Player>,
    >,
    q_web: Query<&Web>,
) {
    let (mut movement, info, mut vel, mut cef) = query.single_mut();
    let old_value = movement.axis;

    movement.axis = input.pressed(control.left).then_some(-1.0).unwrap_or(0.0)
        + input.pressed(control.right).then_some(1.0).unwrap_or(0.0);

    // Reduce vel when change direction or stop moving but still
    // still keep the possibility of speed boost.
    let vel_vec = old_value * movement.axis;
    let linvel = vel.linvel.clone();
    let reduce_vel = || {
        vel.linvel = Vec2::new(0.0, vel.linvel.y);
    };
    let web_unattached = q_web
        .is_empty()
        .not()
        .then_some(|| q_web.single().attached.then_some(true))
        .is_none();
    ((info.is_grounded || web_unattached) && vel_vec <= 0.0).then(reduce_vel);

    cef.forces
        .entry(movement.movement_force_id)
        .and_modify(|move_force| {
            move_force.x = (linvel.x.abs() < movement.max_velocity)
                .then_some(
                    movement.axis
                        * info
                            .is_grounded
                            .then_some(movement.acceleration)
                            .unwrap_or(movement.airborne_acceleration),
                )
                .unwrap_or(0.0);
        });
}

pub fn apply_accel_when_land(
    mut e_reader: EventReader<PlayerEvent>,
    mut query: Query<(&PlayerMovement, &mut ExternalImpulse), With<Player>>,
) {
    e_reader.iter().for_each(|e| {
        if let PlayerEvent::Grounded(player) = e {
            let (movement, mut force) = query
                .get_mut(*player)
                .expect("Player entity in event is not qualified as a player");

            force.impulse += Vec2::new(movement.landing_accel * movement.axis, 0.0);
        }
    });
}

pub fn send_vel_based_event(mut e_writer: EventWriter<PlayerEvent>) {}
