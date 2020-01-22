use amethyst::{
    core::{
        math::Vector3,
        timing::Time,
        transform::{Transform, TransformBundle},
        Parent,
    },
    prelude::*,
    renderer::{
        palette::{LinSrgba, Srgb},
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        types,
        RenderingBundle,
    }

};
    use rand::prelude::*;

use amethyst_nphysics::NPhysicsBackend;
use amethyst_physics::{prelude::*, PhysicsBundle};

mod visual_utils;

// Create floor
        create_floor(data.world);

        // Create Box
        add_cube_entity(data.world, Vector3::new(0.0, 6.0, 0.0));
    


pub fn create_floor(world: &mut World) {
    let shape = {
        let desc = ShapeDesc::Cube {
            half_extents: Vector3::new(20.0, 0.2, 20.0),
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;

        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    let mesh = {
        let mesh_data: types::MeshData = Shape::Cube
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                20.0, 0.2, 20.0,
            )))
            .into();

        visual_utils::create_mesh(world, mesh_data)
    };

    let mat = visual_utils::create_material(
        world,
        LinSrgba::new(0.0, 1.0, 0.0, 1.0),
        0.0, // Metallic
        1.0, // Roughness
    );

    world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(BoundingSphere::origin(20.0))
        .with(Transform::default())
        .with(shape)
        .with(rb)
        .build();
}

pub fn add_cube_entity(world: &mut World, pos: Vector3<f32>) {
    let shape = {
        let desc = ShapeDesc::Cube {
            half_extents: Vector3::new(1.0, 1.0, 1.0),
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let rb_desc = RigidBodyDesc::default();

        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    let mesh = {
        let mesh_data: types::MeshData = Shape::Cube
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                1.0, 1.0, 1.0,
            )))
            .into();

        visual_utils::create_mesh(world, mesh_data)
    };

    let mut rng = rand::thread_rng();
    let mat = visual_utils::create_material(
        world,
        LinSrgba::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
        0.0,
        1.0,
    );

    let mut transf = Transform::default();
    transf.set_translation(pos);

    world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(BoundingSphere::origin(1.0))
        .with(transf)
        .with(shape)
        .with(rb)
        .build();
}
