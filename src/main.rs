// #![windows_subsystem = "windows"]

use std::{path::{Path, PathBuf}, env, fs};
use commands::RootController;
use constants::START_PACKAGE;
use serde::{Serialize, Deserialize};

use druid::{
    im::{Vector, vector},
    widget::{
        prelude::*,
        {Flex, Label, TextBox}, List, Scroll, CrossAxisAlignment, Button, Container
    },
    {AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc}, Color, Command, Target
};
use data::{wad::Wad, package::{Package, SerializePackage}};

mod data;
mod commands;
mod constants;

#[derive(Clone, Data, Lens)]
struct DorgState {
    packages: Vector<Package>,
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("Dorg!")
        .window_size((1200.0, 900.0));

    // Create directories
    let root_dir = env::current_dir().expect("Failed to get root directory.");

    fs::create_dir_all(root_dir.join("saves")).expect("Failed to create 'saves' directory.");
    fs::create_dir_all(root_dir.join("packages")).expect("Failed to create 'packages' directory.");
    fs::create_dir_all(root_dir.join("wads")).expect("Failed to create 'wads' directory.");

    let mut packages = vector![
        Package {
            name: "Doom II".to_string(),
            iwad: "doom2.wad".to_string(),
            wads: vector![],
            hide_if_dependencies_are_missing: true,
        },
    ];

    for package in globwalk::glob("packages/**/*.toml").unwrap().into_iter().filter_map(|r| r.ok()) {
        if let Ok(s) = fs::read_to_string(package.path()) {
            match toml::from_str::<SerializePackage>(&s) {
                Ok(p) => packages.push_back(p.transform()),
                Err(err) => println!("{err}"),
            } 
        }
    }

    // create the initial app state
    let initial_state = DorgState {
        packages,
    };

    for p in initial_state.packages.iter() {
        println!("{}", p.name);
    }

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<DorgState> {
    // a label that will determine its text based on the current app data.
    // let label = Label::new(|data: &DorgState, _env: &Env| {
    //     if data.name.is_empty() {
    //         "Hello anybody!?".to_string()
    //     } else {
    //         format!("Hello {}!", data.name)
    //     }
    // })
    // .with_text_size(32.0);

    let list = Scroll::new(List::new(|| 
        Flex::row()
            .with_child(
                Label::new(|item: &Package, _env: &_| item.name.clone()).with_text_size(17.)
            )
            .with_child(
                Button::new("Start")
                    .on_click(|ctx, package: &mut Package, _| {
                        ctx.submit_command(Command::new(
                            START_PACKAGE,
                            package.clone(),
                            Target::Auto,
                        ));
                    })
                    .fix_height(40.0)
                    .fix_width(100.0),
            )
    ))
    .vertical()
    .lens(DorgState::packages)
    .background(Color::rgba(1.0, 1.0, 1.0, 0.2));

    
        
    
    Container::new(
        Flex::row()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_flex_child(list, 1.0)
    )
    .controller(RootController)
}

