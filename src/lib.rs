use bevy::{
    app::AppExit,
    prelude::*,
    utils::HashMap,
    xr::{XrSessionMode, XrSystem},
};
#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;
use bevy_scene_hook::{HookedDynamicSceneBundle, HookedSceneBundle, SceneHook};

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
    //  Scene management
    {
        app.add_plugin(bevy_scene_hook::HookPlugin)
            .init_resource::<SceneMaterialMeshes>()
            .add_startup_system(init_material_meshes)
            .add_system(populate_mesh)
    };
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

#[derive(Bundle, Clone)]
struct MaterialMesh {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}

#[derive(Resource, Default)]
struct SceneMaterialMeshes {
    material_mesh_by_name: HashMap<String, MaterialMesh>,
}

fn init_material_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_material_meshes: ResMut<SceneMaterialMeshes>,
) {
    scene_material_meshes.material_mesh_by_name.insert(
        "BevyCube".into(),
        MaterialMesh {
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        },
    );
    scene_material_meshes.material_mesh_by_name.insert(
        "BevyPlane".into(),
        MaterialMesh {
            material: materials.add(Color::rgb(0.4, 0.5, 0.3).into()),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        },
    );
}

fn startup(mut xr_system: ResMut<XrSystem>, mut app_exit_events: EventWriter<AppExit>) {
    if xr_system.is_session_mode_supported(XrSessionMode::ImmersiveVR) {
        xr_system.request_session_mode(XrSessionMode::ImmersiveVR);
    } else {
        bevy::log::error!("The XR device does not support immersive VR mode");
        app_exit_events.send(AppExit)
    }

    println!("startup done");
}

#[derive(Component)]
struct NeedsMesh;

fn load_start_scene(mut c: Commands, asset_server: Res<AssetServer>) {
    //  NOTE: use DynamicHookedSceneBundle for .ron scenes and HookedSceneBundle for .gltf scenes.
    c.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("scenes/untitled.gltf#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str()) {
                Some("Cube" | "Plane") => {
                    cmds.insert(NeedsMesh);
                }
                _ => {}
            };
            //  Fix Blender light intensities and enable shadows
            if let Some(light) = entity.get::<SpotLight>() {
                cmds.insert(SpotLight {
                    intensity: light.intensity / 200.,
                    shadows_enabled: true,
                    ..light.clone()
                });
            }
            if let Some(light) = entity.get::<PointLight>() {
                cmds.insert(PointLight {
                    intensity: light.intensity / 200.,
                    shadows_enabled: true,
                    ..light.clone()
                });
            }
            // entity.get::<Light>();
        }),
    });
}

fn populate_mesh(
    mut c: Commands,
    q: Query<(Entity, &Name), With<NeedsMesh>>,
    meshes: Res<SceneMaterialMeshes>,
) {
    for (ent, name) in q.iter() {
        let mut ent = c.entity(ent);
        ent.remove::<NeedsMesh>();
        if let Some(mesh_mat) = meshes.material_mesh_by_name.get(name.as_str()) {
            ent.insert(mesh_mat.clone());
        }
    }
}
