mod camera;
mod client;
mod input_sync;
mod interpolation;
mod player;
mod server;
mod terrain;
mod transform_sync;
mod ui_resize;
mod ui_task;

pub use self::camera::*;
pub use self::client::*;
pub use self::input_sync::*;
pub use self::interpolation::*;
pub use self::player::*;
pub use self::server::*;
pub use self::terrain::*;
pub use self::transform_sync::*;
pub use self::ui_resize::*;
pub use self::ui_task::*;
use crate::tools::net::message::Message;
use crate::tools::net::postman::Postman;
use std::net::SocketAddr;

trait NetworkSystem<T, S: Message, R: Message> {
    fn run(&mut self, data: &mut T) {
        for (address, message) in self.get_postman_mut().pull_messages::<S>() {
            self.on_message(address, &message, data);
        }

        self.get_postman_mut().update();
    }

    fn on_message(&mut self, address: SocketAddr, message: &R, data: &mut T);

    fn get_postman_mut(&mut self) -> &mut Postman<R>;
}
