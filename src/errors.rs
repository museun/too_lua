use too::{math::pos2, renderer::Grapheme};

#[derive(Default)]
pub struct Errors {
    errors: Vec<(&'static str, String)>,
}

impl Errors {
    pub fn handle_lua_error(&mut self, hint: &'static str, err: mlua::Error) {
        match err {
            mlua::Error::SyntaxError { message, .. } => {
                if let Some(msg) = message.splitn(5, ':').last().map(<str>::trim) {
                    self.show_error(hint, format!("cannot load configuration: {msg}"))
                } else {
                    self.show_error(hint, "configuration has invalid syntax");
                }
            }

            mlua::Error::RuntimeError(err) => {
                if let Some(mut first) = err.lines().nth(0) {
                    if first.starts_with('[') {
                        if let Some((_, tail)) = first.rsplit_once(':') {
                            first = tail.trim()
                        }
                    }

                    self.show_error(hint, format!("runtime error: {first}"));
                }
                // self.show_error(format!("runtime error: {err}"));
            }

            mlua::Error::BadArgument {
                to: Some(to),
                name: Some(name),
                ..
            } => {
                self.show_error(hint, format!("bad argument: {to} w/ {name}"));
            }

            mlua::Error::CallbackError { cause, .. } => {
                self.show_error(hint, format!("{cause}"));
            }

            _ => {
                self.show_error(hint, "cannot load configuration");
            }
        }
    }

    fn show_error(&mut self, hint: &'static str, err: impl ToString) {
        self.errors.extend(
            err.to_string()
                .lines()
                .filter(|c| !c.is_empty())
                .map(|s| (hint, s.to_string())),
        );
    }

    pub fn render_errors(&mut self, surface: &mut too::renderer::Surface) {
        if self.errors.is_empty() {
            return;
        }

        for (y, (hint, error)) in self.errors.drain(..).enumerate() {
            surface.set(pos2(0, y as i32), Grapheme::new(hint).fg("#F0F").bg("#000"));

            surface.set(
                pos2(hint.len() as i32 + 1, y as i32),
                Grapheme::new(error).fg("#F00").bg("#000"),
            );
        }
    }
}
