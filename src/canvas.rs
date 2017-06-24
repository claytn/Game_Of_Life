/* Mock up of the Game_Of_Life grid that will be used */

#[cfg(all(feature="winit", feature="glium"))] #[macro_use] extern crate conrod;
#[cfg(all(feature="winit", feature="glium"))] mod support;

fn main() {
    feature::main();
}

#[cfg(all(feature="winit", feature="glium"))]
mod feature {
    extern crate find_folder;
    use conrod;
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
    use support;

    pub fn main() {
        const WIDTH: u32 = 600;
        const HEIGHT: u32 = 600;

        // Build the window.
        let display = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_dimensions(WIDTH, HEIGHT)
            .with_title("Canvas Demo")
            .with_multisampling(4)
            .build_glium()
            .unwrap();

        // construct our `Ui`.
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        // Add a `Font` to the `Ui`'s `font::Map` from file.
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();

        // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
        // for drawing to the glium `Surface`.
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        // The image map describing each of our widget->image mappings (in our case, none).
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        // Instantiate the generated list of widget identifiers.
        let ids = &mut Ids::new(ui.widget_id_generator());

        // Poll events from the window.
        let mut event_loop = support::EventLoop::new();
        'main: loop {

            // Handle all events.
            for event in event_loop.next(&display) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed =>
                        break 'main,
                    _ => {},
                }
            }

            // Instnatiate all widgets in the GUI.
            set_widgets(ui.set_widgets(), ids);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    // Draw the Ui.
    fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

        // Construct our main `Canvas` tree.
        widget::Canvas::new().flow_down(&[
            (ids.footer, widget::Canvas::new().color(color::WHITE).scroll_kids_vertically()),
        ]).set(ids.master, ui);

        // A scrollbar for the `FOOTER` canvas.
        widget::Scrollbar::y_axis(ids.footer).auto_hide(true).set(ids.footer_scrollbar, ui);

        // Here we make some canvas `Tabs` in the middle column.
        let footer_wh = ui.wh_of(ids.footer).unwrap();
        let mut elements = widget::Matrix::new(COLS, ROWS)
            .w_h(footer_wh[0], footer_wh[1] * 2.0)
            .mid_top_of(ids.footer)
            .set(ids.button_matrix, ui);
        while let Some(elem) = elements.next(ui) {
            let (r, c) = (elem.row, elem.col);
            let n = c + r * c;
            let luminance = n as f32 / (COLS * ROWS) as f32;
            let button = widget::Button::new().color(color::WHITE);
            for _click in elem.set(button, ui) {
                println!("Hey! {:?}", (r, c));
            }
        }
    }


    // Button matrix dimensions.
    const ROWS: usize = 8;
    const COLS: usize = 8;

    // Generate a unique `WidgetId` for each widget.
    widget_ids! {
        struct Ids {
            master,
            header,
            body,
            left_column,
            middle_column,
            right_column,
            footer,
            footer_scrollbar,
            floating_a,
            floating_b,
            tabs,
            tab_foo,
            tab_bar,
            tab_baz,

            title,
            subtitle,
            top_left,
            bottom_right,
            foo_label,
            bar_label,
            baz_label,
            button_matrix,
            bing,
            bong,
        }
    }
}

#[cfg(not(all(feature="winit", feature="glium")))]
mod feature {
    pub fn main() {
        println!("This example requires the `winit` and `glium` features. \
                 Try running `cargo run --release --features=\"winit glium\" --example <example_name>`");
    }
}
