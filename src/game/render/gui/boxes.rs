use super::*;

///////////////////////////////////////////////////////////////////////////////
/// Box
///////////////////////////////////////////////////////////////////////////////
pub struct BoxConfig {
    pub color: ColorPair,
    pub double: bool,
    pub hollow: bool,
    pub dimensions: (usize, usize), // (width, height)
}

impl BoxConfig {
    pub fn new(dimensions: (usize, usize), color: ColorPair, double: bool, hollow: bool) -> Self {
        Self {
            dimensions,
            color,
            double,
            hollow,
        }
    }
}

pub struct BoxConfigWithTitle {
    // box
    pub box_config: BoxConfig,
    // text
    pub text_config: TextConfig,
}

impl BoxConfigWithTitle {
    pub fn new(box_config: BoxConfig, text_config: TextConfig) -> Self {
        Self {
            box_config,
            text_config,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Text
///////////////////////////////////////////////////////////////////////////////
pub enum Alignment {
    Left,
    Right,
    Center,
}

pub struct TextConfig {
    pub title: String,
    pub alignment: Alignment,
    pub title_color: ColorPair,
    pub footer: Option<String>,
}

impl TextConfig {
    pub fn new(title: String, title_color: ColorPair, alignment: Alignment) -> Self {
        Self {
            title,
            alignment,
            title_color,
            footer: None,
        }
    }

    pub fn with_footer<S: ToString, F: ToString>(
        title: S,
        footer: F,
        title_color: ColorPair,
        alignment: Alignment,
    ) -> Self {
        Self {
            title: title.to_string(),
            alignment,
            title_color,
            footer: Some(footer.to_string()),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Box Implementations
///////////////////////////////////////////////////////////////////////////////

pub fn center_box(batch: &mut DrawBatch, config: BoxConfig) -> Rect {
    batch.target(LAYER_TEXT); // Draw on the text layer

    let BoxConfig {
        color,
        double,
        hollow,
        dimensions,
    } = config;

    let (width, height) = dimensions;
    assert!(*MAP_PANEL_WIDTH / 2 > width);
    assert!(*MAP_PANEL_HEIGHT / 2 > height);

    let start_x = *MAP_PANEL_WIDTH / 2 - width / 2;
    let start_y = *MAP_PANEL_HEIGHT / 2 - height / 2;
    let end_x = width;
    let end_y = height / 2;

    let box_rect = Rect::with_size(start_x, start_y, end_x, end_y);

    match (double, hollow) {
        (true, true) => batch.draw_hollow_double_box(box_rect, color),
        (true, false) => batch.draw_double_box(box_rect, color),
        (false, true) => batch.draw_hollow_box(box_rect, color),
        (false, false) => batch.draw_box(box_rect, color),
    };

    box_rect
}

pub fn center_box_with_title(batch: &mut DrawBatch, config: BoxConfigWithTitle) -> Rect {
    batch.target(LAYER_TEXT); // Draw on the text layer

    let BoxConfigWithTitle {
        box_config,
        text_config,
    } = config;

    let BoxConfig {
        dimensions,
        double,
        hollow,
        color,
    } = box_config;
    let (width, height) = dimensions;

    assert!(*MAP_PANEL_WIDTH / 2 > width);
    assert!(*MAP_PANEL_HEIGHT / 2 > height);

    let start_x = *MAP_PANEL_WIDTH / 2 - width / 2;
    let start_y = *MAP_PANEL_HEIGHT / 2 - height / 2;
    let end_x = width;
    let end_y = height;

    let box_rect = Rect::with_size(start_x, start_y, end_x, end_y);

    match (double, hollow) {
        (true, true) => batch.draw_hollow_double_box(box_rect, color),
        (true, false) => batch.draw_double_box(box_rect, color),
        (false, true) => batch.draw_hollow_box(box_rect, color),
        (false, false) => batch.draw_box(box_rect, color),
    };

    let TextConfig {
        title,
        alignment,
        title_color,
        footer,
    } = text_config;

    match alignment {
        Alignment::Left => {
            batch.print_color(Point::new(box_rect.x1 + 2, box_rect.y1), title, title_color);
        }
        Alignment::Right => {
            batch.print_color_right(
                Point::new(box_rect.x2 - title.len() as i32, box_rect.y1),
                title,
                title_color,
            );
        }
        Alignment::Center => {
            batch.print_color_centered_at(
                Point::new(box_rect.width() / 2, box_rect.y1),
                title,
                title_color,
            );
        }
    }

    if let Some(footer) = footer {
        match alignment {
            Alignment::Left => {
                batch.print_color(
                    Point::new(box_rect.x1 + 2, box_rect.y2),
                    footer,
                    title_color,
                );
            }
            Alignment::Right => {
                batch.print_color_right(
                    Point::new(box_rect.x2 - footer.len() as i32, box_rect.y2),
                    footer,
                    title_color,
                );
            }
            Alignment::Center => {
                batch.print_color_centered_at(
                    Point::new(box_rect.width() / 2, box_rect.y2),
                    footer,
                    title_color,
                );
            }
        }
    }

    box_rect
}
