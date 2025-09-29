use crate::spherical::SphericalCoords;
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::{Vec2, Vec3, Vec4};
use bevy::prelude::Vec4Swizzles;
use bevy::prelude::{
    Bundle, Camera3d, Component, MouseButton, Projection, Res, Single, Time, Transform, Window,
    With,
};
use bevy::render::camera::CameraProjection;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;
use std::ops::{AddAssign, Mul, Sub};

mod spherical;

pub struct OrbitControlsPlugin;

impl Plugin for OrbitControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_wheel)
            .add_systems(Update, handle_mouse_input)
            .add_systems(Update, update);
    }
}

#[derive(PartialEq)]
enum OrbitControllerState {
    Idle,
    Pivoting {
        cursor_start: Vec2,
        spherical_start: SphericalCoords,
    },
    Panning {
        cursor_start: Vec2,
        target_start: Vec3,
    },
}

#[derive(Component)]
pub struct OrbitCameraController {
    state: OrbitControllerState,
    target: Vec3,
    spherical_pos: SphericalCoords,
    pub zoom_speed: f32,
    pub pan_speed: f32,
    pub pivot_speed: f32,
}

impl OrbitCameraController {
    pub fn new(camera_position: Vec3, camera_target: Vec3) -> Self {
        OrbitCameraController {
            spherical_pos: SphericalCoords::from(camera_position - camera_target),
            target: camera_target,
            ..OrbitCameraController::default()
        }
    }
}

impl Default for OrbitCameraController {
    fn default() -> Self {
        Self {
            state: OrbitControllerState::Idle,
            target: Vec3::ZERO,
            spherical_pos: SphericalCoords::from(Vec3::ONE),
            zoom_speed: 0.5,
            pan_speed: 3.0,
            pivot_speed: 2.0,
        }
    }
}

#[derive(Bundle)]
pub struct OrbitCameraControllerBundle {
    controller: OrbitCameraController,
    transform: Transform,
}

impl From<OrbitCameraController> for OrbitCameraControllerBundle {
    fn from(value: OrbitCameraController) -> Self {
        let transform = Transform::from_translation(value.spherical_pos.into())
            .looking_at(value.target, Vec3::Y);
        Self {
            controller: value,
            transform,
        }
    }
}

fn handle_mouse_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_target: Single<&mut OrbitCameraController, With<Camera3d>>,
) {
    let mut controller = q_target.into_inner();
    let cursor_pos = q_window.cursor_position();

    if mouse_input.just_pressed(MouseButton::Left)
        && controller.state == OrbitControllerState::Idle
        && let Some(cursor_start) = cursor_pos
    {
        controller.state = OrbitControllerState::Pivoting {
            cursor_start,
            spherical_start: controller.spherical_pos,
        }
    }

    if mouse_input.just_released(MouseButton::Left)
        && let OrbitControllerState::Pivoting { .. } = controller.state
    {
        controller.state = OrbitControllerState::Idle;
    }

    if mouse_input.just_pressed(MouseButton::Right)
        && controller.state == OrbitControllerState::Idle
        && let Some(cursor_start) = cursor_pos
    {
        controller.state = OrbitControllerState::Panning {
            cursor_start,
            target_start: controller.target,
        }
    }

    if mouse_input.just_released(MouseButton::Right) {
        controller.state = OrbitControllerState::Idle;
    }
}

fn handle_wheel(
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
    camera_transform: Single<(&mut Transform, &mut OrbitCameraController), With<Camera3d>>,
    time: Res<Time>,
) {
    let (mut transform, mut controller) = camera_transform.into_inner();

    if mouse_wheel_input.delta.y == 0.0 || controller.state != OrbitControllerState::Idle {
        return;
    }

    let dir = controller.target.sub(transform.translation);
    let dir_n = dir.normalize();
    let delta = mouse_wheel_input.delta.y
        * controller.spherical_pos.radius
        * controller.zoom_speed
        * time.delta_secs();

    transform.translation.add_assign(dir_n.mul(delta));

    controller.spherical_pos = SphericalCoords::from(transform.translation);
}

fn update(
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_controller: Single<(&mut OrbitCameraController, &mut Transform, &Projection), With<Camera3d>>,
) {
    let cursor_pos = q_window.cursor_position();

    if cursor_pos.is_none() {
        return;
    }

    let height = q_window.height();
    let width = q_window.width();

    let (mut controller, mut transform, projection) = q_controller.into_inner();

    match controller.state {
        OrbitControllerState::Pivoting {
            cursor_start,
            spherical_start,
        } => {
            let Vec2 {
                x: cursor_x,
                y: cursor_y,
            } = cursor_pos.unwrap();

            controller.spherical_pos.theta = spherical_start.theta
                - ((cursor_x - cursor_start.x) / height) * PI * controller.pivot_speed;
            controller.spherical_pos.phi = spherical_start.phi
                - ((cursor_y - cursor_start.y) / height) * PI * controller.pivot_speed;

            transform.translation = Vec3::from(controller.spherical_pos) + controller.target;
        }
        OrbitControllerState::Panning {
            cursor_start,
            target_start,
        } => {
            let Vec2 {
                x: cursor_x,
                y: cursor_y,
            } = cursor_pos.unwrap();

            let delta_x = 0.5 * (cursor_x - cursor_start.x) / width;
            let delta_y = 0.5 * (cursor_y - cursor_start.y) / height;

            if delta_y != 0.0 || delta_x != 0.0 {
                let proj_inv = projection.get_clip_from_view().inverse();
                let view_inv = transform.compute_matrix();

                let diff = view_inv
                    .mul_vec4(proj_inv.mul_vec4(Vec4::new(delta_x, -delta_y, 0.0, 0.0)))
                    .xyz()
                    * controller.spherical_pos.radius
                    * controller.pan_speed;

                let dir = transform.translation - controller.target;

                controller.target = target_start - diff;
                transform.translation = controller.target + dir;
                let sph_diff = transform.translation - controller.target;
                controller.spherical_pos = sph_diff.into();
                controller.spherical_pos.clamp_phi();
            }
        }
        _ => {}
    }

    transform.look_at(controller.target, Vec3::Y);
}
