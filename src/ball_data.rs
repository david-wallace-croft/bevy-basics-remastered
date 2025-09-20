use ::bevy::ecs::world::World;
use ::bevy::prelude::*;
use ::rand::SeedableRng;
use ::rand::rngs::StdRng;
use ::rand::seq::IndexedRandom;
use ::std::sync::{Mutex, MutexGuard};

#[derive(Resource)]
pub struct BallData {
  pub materials: Vec<Handle<StandardMaterial>>,
  pub mesh: Handle<Mesh>,
  pub rng: Mutex<StdRng>,
}

impl BallData {
  pub fn material(&self) -> Handle<StandardMaterial> {
    let mut rng: MutexGuard<'_, StdRng> = self.rng.lock().unwrap();

    self.materials.choose(&mut *rng).unwrap().clone()
  }
  pub fn mesh(&self) -> Handle<Mesh> {
    self.mesh.clone()
  }
}

impl FromWorld for BallData {
  fn from_world(world: &mut World) -> Self {
    let mesh: Handle<Mesh> =
      world.resource_mut::<Assets<Mesh>>().add(Sphere::new(1.));

    let rng: Mutex<StdRng> = Mutex::new(StdRng::from_os_rng());

    let mut materials: Vec<Handle<StandardMaterial>> = Default::default();

    let mut mat_assets = world.resource_mut::<Assets<StandardMaterial>>();

    for i in 0..36 {
      let color: Color = Color::hsl((i * 10) as f32, 1., 0.5);

      materials.push(mat_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
      }));
    }

    BallData {
      materials,
      mesh,
      rng,
    }
  }
}
