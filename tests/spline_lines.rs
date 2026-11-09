//! The splines' editor lines over a world (`splines_component::SplineLines`):
//! they rebuild only when a curve or its owner changes, every rebuild
//! matches building them from scratch, and what dragging a curve's owner
//! costs (an ignored report). Moved from the component crate's unit tests:
//! attaching a spline needs its class registered, which the root does.

// The plugin as a game links it: its root, with every registration.
use splines_plugin as _;

use splines_component::{
    CurveAlgorithm, SplineComponent, SplineLines, SplinePoint, spline_debug_lines,
};

/// A two-point curve.
fn line() -> SplineComponent {
    let mut d = SplineComponent::default();
    d.points = vec![SplinePoint::new([0.; 3]), SplinePoint::new([3., 0., 4.])];
    d.auto_tangents();
    d
}

/// A long open B-spline (the costliest kind to sample) per scene, for
/// measuring what dragging its owner costs (Pulsar-Native#995).
fn long_bspline(points: usize) -> SplineComponent {
    let mut d = SplineComponent::default();
    d.algorithm = CurveAlgorithm::BSpline;
    d.resolution = 128;
    d.points = (0..points)
        .map(|i| SplinePoint::new([i as f32, (i as f32 * 0.3).sin() * 4.0, 0.0]))
        .collect();
    d
}

#[test]
#[ignore = "perf report: cargo test -p splines_plugin --test spline_lines drag_cost -- --ignored --nocapture"]
fn drag_cost_report() {
    use pulsar_scene_model::{SpawnObject, Transform, world_ext::SceneWorldExt};
    for &(splines, points) in &[(1usize, 64usize), (1, 256), (8, 256)] {
        let mut world = pulsar_scenedb::World::new();
        let mut owners = Vec::new();
        for i in 0..splines {
            let owner = world
                .spawn_object(SpawnObject::new(format!("s{i}")))
                .unwrap();
            pulsar_world_registry::attach_value(&mut world, owner, long_bspline(points)).unwrap();
            owners.push(owner);
        }
        let mut lines = SplineLines::new(&world);
        lines.poll(&world).unwrap();
        let frames = 20;
        let start = std::time::Instant::now();
        for frame in 0..frames {
            world.get_mut::<Transform>(owners[0]).unwrap().position = [f64::from(frame), 0.0, 0.0];
            lines.poll(&world).expect("the owner moved");
        }
        let poll = start.elapsed() / frames;
        let spline = long_bspline(points);
        let start = std::time::Instant::now();
        let length = spline.total_length_m();
        let samples = spline.samples().len();
        let panel = start.elapsed();
        eprintln!(
            "{splines} spline(s) x {points} points: poll per drag frame {poll:?}; \
             samples() + total_length_m() {panel:?} ({samples} samples, {length:.1} m)"
        );
    }
}

#[test]
fn spline_lines_rebuild_only_when_a_spline_or_its_owner_changes() {
    use pulsar_scene_model::{SpawnObject, Transform, Visibility, world_ext::SceneWorldExt};
    let mut world = pulsar_scenedb::World::new();
    let curve = world.spawn_object(SpawnObject::new("curve")).unwrap();
    let bystander = world.spawn_object(SpawnObject::new("bystander")).unwrap();
    let spline = pulsar_world_registry::attach_value(&mut world, curve, line()).unwrap();
    // A second curve that never changes: its cached lines are reused.
    let still = world.spawn_object(SpawnObject::new("still")).unwrap();
    pulsar_world_registry::attach_value(&mut world, still, long_bspline(9)).unwrap();
    let mut inner = SplineLines::new(&world);
    // Every rebuild matches building the lines from scratch.
    let mut lines = Checked(&mut inner);
    struct Checked<'a>(&'a mut SplineLines);
    impl Checked<'_> {
        fn poll(&mut self, world: &pulsar_scenedb::World) -> Option<Vec<helio::WorldVertex>> {
            let lines = self.0.poll(world)?;
            let fresh = spline_debug_lines(world);
            let key = |l: &[helio::WorldVertex]| {
                l.iter().map(|v| (v.position, v.color)).collect::<Vec<_>>()
            };
            assert_eq!(
                key(&lines),
                key(&fresh),
                "cached lines match a full rebuild"
            );
            Some(lines)
        }
    }

    let first = lines.poll(&world).expect("the first poll builds");
    assert!(!first.is_empty());
    assert!(lines.poll(&world).is_none(), "nothing changed");

    world.get_mut::<Transform>(bystander).unwrap().position = [5.0, 0.0, 0.0];
    assert!(lines.poll(&world).is_none(), "an unrelated object moved");

    world.get_mut::<Transform>(curve).unwrap().position = [5.0, 0.0, 0.0];
    let moved = lines.poll(&world).expect("the owner moved");
    assert!(moved != first, "the owner moved");

    world.get_mut::<SplineComponent>(spline).unwrap().points[1].position = [0.0, 0.0, 9.0];
    assert!(lines.poll(&world).is_some(), "the curve changed");

    let shown = spline_debug_lines(&world).len();
    world.get_mut::<Visibility>(curve).unwrap().visible = false;
    let hidden = lines.poll(&world).expect("hidden").len();
    assert!(hidden < shown, "the hidden curve's lines are gone");
    world.get_mut::<Visibility>(curve).unwrap().visible = true;
    assert_eq!(
        lines.poll(&world).map(|l| l.len()),
        Some(shown),
        "shown again"
    );

    pulsar_scene_model::attachments::set_enabled(&mut world, spline, false);
    assert_eq!(
        lines.poll(&world).map(|l| l.len()),
        Some(hidden),
        "disabled"
    );
}
