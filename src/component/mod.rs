mod actor;
mod bot;
mod collision;
mod footsteps;
mod health;
mod inertia;
mod player;
mod projectile;
mod terrain;
mod weapon;

pub use self::{
    actor::*, bot::*, collision::*, footsteps::*, health::*, inertia::*, player::*, projectile::*,
    terrain::*, weapon::*,
};
