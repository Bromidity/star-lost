//pub mod controls;
pub mod camera;
// mod planet;
// mod route;
// mod station;
// mod thrust;
pub mod tracking;

/*
#[allow(dead_code)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let a = station::spawn_station(
        &mut commands,
        &asset_server,
        Vec3::from_slice(&[30.0, 10.0, 30.0]),
        -0.1,
    );
    let b = station::spawn_station(
        &mut commands,
        &asset_server,
        -Vec3::from_slice(&[30.0, 10.0, 30.0]),
        0.3,
    );

    planet::spawn_planet(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        0.1,
    );

    planet::spawn_sun(&mut commands, &mut meshes, &mut materials, 0.1);
    route::spawn_route_ship(&mut commands, &asset_server, vec![a.into(), b.into()]);
    controls::spawn_player_ship(&mut commands, asset_server);
}
*/
