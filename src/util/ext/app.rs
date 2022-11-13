use bevy::app::App;
use bevy::app::Plugin;

pub trait AppExt {
    fn add_plugin_if<T: Plugin>(&mut self, condition: bool, f: fn() -> T) -> &mut Self;
}

impl AppExt for App {
    fn add_plugin_if<T: Plugin>(&mut self, condition: bool, f: fn() -> T) -> &mut Self {
        if condition {
            self.add_plugin(f());
        }

        return self;
    }
}
