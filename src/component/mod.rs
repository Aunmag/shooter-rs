mod actor;
mod bot;
mod collision;
mod footsteps;
mod health;
mod health_bar;
mod inertia;
mod interpolation;
mod player;
mod projectile;
mod terrain;
mod weapon;

pub use self::{
    actor::*, bot::*, collision::*, footsteps::*, health::*, health_bar::*, inertia::*,
    interpolation::*, player::*, projectile::*, terrain::*, weapon::*,
};
