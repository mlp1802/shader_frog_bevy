//! A shader that uses the GLSL shading language.

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(update_material)
        .run();
}

fn update_material(
    mut materials: ResMut<Assets<CustomMaterial>>,
    custom_materials: Query<&mut Handle<CustomMaterial>>,
) {
    //
    for m in &custom_materials {
        let c_mat = materials.get_mut(m).unwrap();
        c_mat.time = c_mat.time + 0.001;
    }
}
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {
            // color_texture: Some(asset_server.load("branding/icon.png")),
            // alpha_mode: AlphaMode::Blend,
            color: Color::BLUE,
            scale: 2.0,
            displacement: 1.0,
            time: 24.0,
            speed: 1.0,
            hellScale: 1.0,
            hellColor: Color::BLUE,
        }),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    scale: f32,
    #[uniform(0)]
    displacement: f32,
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    speed: f32,
    #[uniform(0)]
    hellScale: f32,
    #[uniform(0)]
    hellColor: Color,
    //   #[texture(1)]
    //   #[sampler(2)]
    //   color_texture: Option<Handle<Image>>,
    //   alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/spooky_boom.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/spooky_boom.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        //self.alpha_mode
        AlphaMode::Blend
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
