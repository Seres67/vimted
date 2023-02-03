use cursive::{Cursive, CursiveExt, With};
use cursive::views::{Dialog, EditView, OnEventView, TextArea};
use cursive::view::{Nameable, Resizable, Scrollable};
use std::fs::File;
use std::path::{PathBuf};
use cursive::event::Event::{CtrlChar};
use std::io::{Read, Write};
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Black);

    theme
}

fn main() {
    let mut siv = Cursive::new();
    let theme = custom_theme_from_cursive(&siv);
    siv.set_window_title("Vimted");
    siv.set_theme(theme);

    let editor_content = TextArea::new()
        .with_name("editor_content")
        .scrollable()
        .wrap_with(OnEventView::new);

    siv.add_fullscreen_layer(editor_content);

    siv.add_global_callback(CtrlChar('o'), |s| {
        let dialog = Dialog::new()
            .title("Open a file")
            .content(
                EditView::new().with_name("file_name").fixed_width(30),
            )
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .button("Open", |s| {
                let file_name = s
                    .call_on_name("file_name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let file_path = PathBuf::from(file_name.trim());
                let mut file = match File::open(file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        println!("Failed to open file: {e}");
                        let _ = Dialog::info("Failed to open file")
                            .title("Error")
                            .button("Ok", |s| {
                                s.pop_layer();
                            });
                        return;
                    }
                };

                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        s.call_on_name("editor_content", |view: &mut TextArea| {
                            view.set_content(contents);
                        });
                    }
                    Err(e) => {
                        println!("Failed to read file: {e}");
                        let _ = Dialog::info("Failed to read file")
                            .title("Error")
                            .button("Ok", |s| {
                                s.pop_layer();
                            });
                    }
                };
                s.pop_layer();
            });
        s.add_layer(dialog);
    });

    siv.add_global_callback(CtrlChar('s'), |s| {
        let dialog = Dialog::new()
            .title("Save a file")
            .content(
                EditView::new().with_name("file_name").fixed_width(30),
            )
            .button("Cancel", |s| {
                s.pop_layer();
            })
            .button("Save", |s| {
                let file_name = s
                    .call_on_name("file_name", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                let file_path = PathBuf::from(file_name.trim());
                let file = File::create(file_path).map_err(|e| {
                    println!("Failed to create file: {e}");
                    let _ = Dialog::info("Failed to create file")
                        .title("Error")
                        .button("Ok", |s| {
                            s.pop_layer();
                        });
                });
                let contents = s.call_on_name("editor_content", |view: &mut TextArea| {
                    view.get_content().to_owned()
                }).unwrap();
                file.unwrap().write_all(contents.as_bytes())
                    .expect("Failed to write file");
                s.pop_layer();
            });
        s.add_layer(dialog);
    });
    siv.run();
}