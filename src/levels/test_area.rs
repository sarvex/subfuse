use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{
    assets::{ModelAssets, MyStates},
    entity::{
        door_linear::DoorLinear,
        trigger::{TriggerEnterEvent, TriggerExitEvent},
    },
    Sun,
};

pub struct TestAreaLevelPlugin;
impl Plugin for TestAreaLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(MyStates::RunLevel, setup);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(MyStates::RunLevel)
                .with_system(door_triggers)
                .into(),
        );
    }
}

fn setup(mut cmds: Commands, model_assets: Res<ModelAssets>) {
    // sun, TODO: pull from blender
    cmds.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
            shadow_projection: OrthographicProjection {
                left: -100.0,
                right: 100.0,
                bottom: -100.0,
                top: 100.0,
                near: -500.0,
                far: 500.0,
                scale: 1.0,
                ..default()
            },
            //shadow_depth_bias: 0.1,
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0f32.to_radians(),
            45.0f32.to_radians(),
            0.0,
        )),
        ..default()
    })
    .insert(Sun);

    cmds.spawn_bundle(SceneBundle {
        scene: model_assets.test_area.clone(),
        ..default()
    });
}

fn door_triggers(
    mut doors: Query<(&Name, &mut DoorLinear)>,
    mut trigger_enter_events: EventReader<TriggerEnterEvent>,
    mut trigger_exit_events: EventReader<TriggerExitEvent>,
) {
    iter_trigger_events(
        trigger_enter_events
            .iter()
            .map(|event| event.trigger_name.as_str()),
        &mut doors,
        true,
    );
    iter_trigger_events(
        trigger_exit_events
            .iter()
            .map(|event| event.trigger_name.as_str()),
        &mut doors,
        false,
    );
}

fn iter_trigger_events<'a>(
    events: impl Iterator<Item = &'a str>,
    doors: &mut Query<(&Name, &mut DoorLinear)>,
    open: bool,
) {
    for name in events {
        match name {
            "DOOR TRIG 1" => {
                for mut door in doors.iter_mut().filter_map(|(name, door)| {
                    if name.contains("DOOR_LINEAR Door 1") {
                        Some(door)
                    } else {
                        None
                    }
                }) {
                    door.is_open = open;
                }
            }
            "DOOR TRIG 2" => {
                for mut door in doors.iter_mut().filter_map(|(name, door)| {
                    if name.contains("DOOR_LINEAR Door 2") {
                        Some(door)
                    } else {
                        None
                    }
                }) {
                    door.is_open = open;
                }
            }
            _ => {}
        }
    }
}