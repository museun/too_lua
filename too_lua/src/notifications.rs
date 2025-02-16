use std::time::{Duration, Instant};

use too::{
    layout::Anchor2,
    math::{inverse_lerp, lerp, pos2, vec2, Rect},
    renderer::{Pixel, Surface},
    view::{CroppedSurface, Palette},
};

pub struct Notification {
    pub message: String,
    pub start: Instant,
    pub timeout: Duration,
}

impl Notification {
    pub fn new(message: impl ToString, timeout: Duration) -> Self {
        Self {
            message: message.to_string(),
            start: Instant::now(),
            timeout,
        }
    }
}

pub struct Notifications {
    notifications: Vec<Notification>,
    anchor: Anchor2,
}

impl Notifications {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            anchor: Anchor2::RIGHT_BOTTOM,
        }
    }

    pub fn anchor(mut self, anchor: Anchor2) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn push(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn render(&mut self, offset: i32, palette: &Palette, surface: &mut Surface) {
        self.notifications
            .retain(|notification| notification.start.elapsed() < notification.timeout);

        let (bg, pg, fg) = (palette.surface, palette.outline, palette.primary);

        let size = surface.rect().size();
        for (y, notification) in self.notifications.iter().enumerate() {
            let w = notification.message.len() as i32;
            let rect =
                Rect::from_min_size(pos2(size.x - w, size.y - y as i32 - 1 - offset), vec2(w, 1));

            let dt = notification.timeout - notification.start.elapsed();
            let dt =
                inverse_lerp(0.0, notification.timeout.as_secs_f32(), dt.as_secs_f32()).unwrap();
            let offset = lerp(0.0, rect.width() as f32, dt);

            let mut surface = CroppedSurface::new(rect, surface);
            for (i, ch) in notification.message.char_indices() {
                let mut g = Pixel::new(ch).fg(fg).bg(bg);
                if (i as f32) < offset.ceil() {
                    g = g.bg(pg)
                }
                surface.set(pos2(i as i32, 0), g);
            }
        }
    }
}
