use std::fs;

use cursive::{views::{LinearLayout, TextView, TextArea, Button}, Cursive, view::{Resizable, Nameable}};


pub fn edit(title: String, file_path: String, content: String) {
  let mut plat = cursive::default();
  plat.set_user_data(file_path);
  plat.add_global_callback('q', quit);
  plat.add_layer(
      LinearLayout::vertical()
          .child(TextView::new(title))
          .child(
              TextArea::new()
                  .content(content)
                  .min_height(20)
                  .with_name("content"),
          )
          .child(
              LinearLayout::horizontal()
                  .child(Button::new("Save", save))
                  .child(Button::new("Save & Quit", save_quit))
                  .child(Button::new("Quit without saving", quit)),
          ),
  );
  plat.run();
}

fn save(cur: &mut Cursive) {
  let option_content = cur.find_name::<TextArea>("content");
  if let Some(text_view) = option_content {
      let str_content = text_view.get_content().to_string();
      if let Some(file_path) = cur.user_data::<String>() {
          fs::write(file_path, format!("{}", str_content)).unwrap_or_default();
      }
  }
}
fn save_quit(cur: &mut Cursive) {
  save(cur);
  quit(cur);
}
fn quit(cur: &mut Cursive) {
  cur.quit();
}
