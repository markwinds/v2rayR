use std::thread;

pub fn add_tray() -> Result<(), systray::Error> {
    let mut app = systray::Application::new()?;

    app.set_icon_from_resource("IDI_ICON1")?;


    // app.add_menu_item("Print a thing", |_| {
    //     println!("Printing a thing!");
    //     Ok::<_, systray::Error>(())
    // })?;
    //
    // app.add_menu_item("Add Menu Item", |window| {
    //     window.add_menu_item("Interior item", |_| {
    //         println!("what");
    //         Ok::<_, systray::Error>(())
    //     })?;
    //     window.add_menu_separator()?;
    //     Ok::<_, systray::Error>(())
    // })?;
    //
    // app.add_menu_separator()?;
    //
    // app.add_menu_item("Quit", |window| {
    //     window.quit();
    //     Ok::<_, systray::Error>(())
    // })?;

    thread::spawn(move || {
        app.wait_for_message().unwrap();
    });
    Ok(())
}