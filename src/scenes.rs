use crate::*;

pub struct SceneTweaksPlugin;

impl Plugin for SceneTweaksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_scene_hook::HookPlugin)
            .init_resource::<SceneMaterialMeshes>()
            .add_startup_system(init_material_meshes)
            .add_system(populate_mesh);
    }
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

//  Fills in Mesh and Material components for entity names registered in
//  `SceneMaterialMeshes`
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
