use bevy::{
    app::AppExit,
    prelude::*,
    utils::HashMap,
    xr::{XrSessionMode, XrSystem},
};
#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

pub fn game_main() {
    let mut app = App::new();
    #[cfg(feature = "editor")]
    {
        let mut wgpu_settings = bevy::render::settings::WgpuSettings::default();
        wgpu_settings.features |= bevy::render::settings::WgpuFeatures::POLYGON_MODE_LINE;
        app.insert_resource(wgpu_settings);
    }
    app.add_plugins(DefaultPlugins);
    #[cfg(all(feature = "editor", not(target_os = "android")))]
    {
        //  EditorPlugin must be added after DefaultPlugins.
        app.add_plugin(EditorPlugin);
        app.add_startup_system(editor_startup);
    }
    app.add_startup_system(startup)
        .add_startup_system(load_start_scene)
        .run();
}

#[cfg(all(feature = "editor", not(target_os = "android")))]
fn editor_startup(mut c: Commands) {
    c.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

struct MaterialMesh {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}

#[derive(Resource, Default)]
struct SceneMaterialMeshes {
    material_mesh_by_name: HashMap<String, MaterialMesh>,
}

fn init_material_meshes(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_material_meshes: ResMut<SceneMaterialMeshes>,
) {
    scene_material_meshes.material_mesh_by_name.insert(
        "Cube".into(),
        MaterialMesh {
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        },
    );
}

fn startup(
    mut c: Commands,
    mut xr_system: ResMut<XrSystem>,
    mut app_exit_events: EventWriter<AppExit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if xr_system.is_session_mode_supported(XrSessionMode::ImmersiveVR) {
        xr_system.request_session_mode(XrSessionMode::ImmersiveVR);
    } else {
        bevy::log::error!("The XR device does not support immersive VR mode");
        app_exit_events.send(AppExit)
    }

    c.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.4, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    c.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    println!("startup done");
}

fn load_start_scene(mut c: Commands, asset_server: Res<AssetServer>, meshes: ResMut<Assets<Mesh>>) {
    c.spawn(DynamicSceneBundle {
        scene: asset_server.load("scenes/start.scn.ron"),
        ..Default::default()
    });
    let meshes = meshes.clone();
    c.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("scene.glb#Scene0"),
            ..default()
        },
        hook: SceneHook::new(
            |entity, cmds| match entity.get::<Name>().map(|t| t.as_str()) {
                Some("Cube") => {
                    cmds.insert(meshes.add(Mesh::from(shape::Cube { size: 1.0 })));
                }
                _ => {}
            },
        ),
    });
}
