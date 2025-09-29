# Bevy Orbit Controls

Very simple camera controller for [bevy game engine](https://bevy.org) inspired by
[threejs orbit controls](https://threejs.org/examples/?q=orbit#misc_controls_orbit).

This repo includes a simple example. Check it out via `cargo run --example basic_usage`.

## Usage


Just add the `OrbitControlsPlugin` plugin during setup method and spawn the `OrbitCameraControllerBundle` component with
the camera you want it attached to. The bundle will add a `OrbitCameraController` as well as a `Transform` component to
the entity.

```rust
commands.spawn((
    Camera3d::default(),
    Projection::from(PerspectiveProjection {
        ..PerspectiveProjection::default()
    }),
    OrbitCameraControllerBundle::from(
        OrbitCameraController::new(vec3(5.0, 5.0, 5.0), Vec3::ZERO)
    )
));
```