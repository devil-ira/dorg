
use std::{process::Command, path::{Path, PathBuf}};

use druid::{
    widget::{Container, Controller}, Widget, Event,
};
use log::error;

use crate::{DorgState, constants::START_PACKAGE, data::wad};

pub struct RootController;

impl Controller<DorgState, Container<DorgState>> for RootController {
    fn event(
        &mut self,
        child: &mut Container<DorgState>,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut DorgState,
        env: &druid::Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(START_PACKAGE) => {
                let package = cmd.get_unchecked(START_PACKAGE);
                println!("Running package: {}", package.name);

                let mut command = Command::new(Path::new("F:/Development/dorg/gzdoom-4-7-1-Windows-64bit/gzdoom.exe").as_os_str());

                if let Ok(iwad_path) = wad::find(&package.iwad) {
                    command.arg("-iwad");
                    command.arg(iwad_path.as_os_str());
                } else {
                    error!("Could not find wad: '{}'!", package.iwad);
                    return;
                }

                command.arg("-savedir");
                command.arg(&PathBuf::from("saves").join(&package.name));

                let file_paths: Vec<_> = package
                    .wads
                    .iter()
                    .filter_map(|w| wad::find(&w.name).ok())
                    .collect();

                if file_paths.len() > 0 {
                    command.arg("-file");
                    command.args(file_paths.iter().map(|p| p.as_os_str()));
                }

                let _ = command.spawn();
            }
            // Event::Command(cmd) if cmd.is(SOURCEPORT_LOADED) => {
            //     let sourceport = cmd.get_unchecked(SOURCEPORT_LOADED);
            //     data.sourceports.push_back(sourceport.clone());
            // }
            // Event::Command(cmd) if cmd.is(PACKAGE_LOADED) => {
            //     let package = cmd.get_unchecked(PACKAGE_LOADED);
            //     data.packages.push_back(package.clone());
            // }
            // Event::Command(cmd) if cmd.is(DOWNLOAD) => {
            //     let name = cmd.get_unchecked(DOWNLOAD).clone();
            //     let bitness = data.bitness.clone();
            //     thread::spawn(move || {
            //         if let Err(e) = other::download(name, None, bitness) {
            //             error!("DOWNLOAD {}", e)
            //         }
            //     });
            // }
            // Event::Command(cmd) if cmd.is(RELOAD_PACKAGE_DEPENDENCIES) => {
            //     for p in data.packages.iter_mut() {
            //         p.update_dependency_paths()
            //     }
            // }
            _ => {}
        }
        child.event(ctx, event, data, env)
    }
}
