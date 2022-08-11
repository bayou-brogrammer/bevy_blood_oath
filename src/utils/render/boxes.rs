use super::*;

///////////////////////////////////////////////////////////////////////////////
/// Box
///////////////////////////////////////////////////////////////////////////////
pub struct BoxConfig {
    pub color: ColorPair,
    pub double: bool,
    pub hollow: bool,
    pub dimensions: (i32, i32), // (width, height)
}

impl BoxConfig {
    pub fn new(dimensions: (i32, i32), color: ColorPair, double: bool, hollow: bool) -> Self {
        Self { dimensions, color, double, hollow }
    }
}

pub struct BoxConfigWithTitle {
    // box
    pub box_config: BoxConfig,
    // text
    pub text_config: TextConfig,
}

impl BoxConfigWithTitle {
    pub fn new(box_config: BoxConfig, text_config: TextConfig) -> Self { Self { box_config, text_config } }
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
    pub footer_color: Option<ColorPair>,
    pub with_dividers: bool,
}

impl TextConfig {
    pub fn new<S: ToString>(
        title: S,
        title_color: ColorPair,
        alignment: Alignment,
        with_dividers: bool,
    ) -> Self {
        Self {
            alignment,
            title_color,
            footer: None,
            with_dividers,
            footer_color: None,
            title: title.to_string(),
        }
    }

    pub fn with_footer<S: ToString, F: ToString>(
        title: S,
        footer: F,
        title_color: ColorPair,
        footer_color: ColorPair,
        alignment: Alignment,
        with_dividers: bool,
    ) -> Self {
        Self {
            alignment,
            title_color,
            with_dividers,
            title: title.to_string(),
            footer: Some(footer.to_string()),
            footer_color: Some(footer_color),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Box Implementations
///////////////////////////////////////////////////////////////////////////////

fn draw_box(batch: &mut DrawBatch, box_rect: Rect, double: bool, hollow: bool, color: ColorPair) -> Rect {
    match (double, hollow) {
        (true, true) => batch.draw_hollow_double_box(box_rect, color),
        (true, false) => batch.draw_double_box(box_rect, color),
        (false, true) => batch.draw_hollow_box(box_rect, color),
        (false, false) => batch.draw_box(box_rect, color),
    };

    box_rect
}

fn draw_title(batch: &mut DrawBatch, box_rect: Rect, text_config: TextConfig, box_config: BoxConfig) {
    // let BoxConfig { color, double, hollow, dimensions } = box_config;
    let TextConfig { title, alignment, title_color, footer, footer_color, with_dividers } = text_config;
    let box_color = box_config.color.fg;

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
            if with_dividers {
                print_label(
                    batch,
                    title,
                    Point::new(box_rect.x1, box_rect.y1),
                    box_rect.width(),
                    title_color.fg,
                    box_color,
                );
            } else {
                batch.print_color_centered_at(
                    Point::new(box_rect.x1 + box_rect.width() / 2, box_rect.y1),
                    title,
                    title_color,
                );
            }
        }
    }

    if let Some(footer) = footer {
        let footer_color = footer_color.unwrap_or(title_color);

        match alignment {
            Alignment::Left => {
                batch.print_color(Point::new(box_rect.x1 + 2, box_rect.y2), footer, footer_color);
            }
            Alignment::Right => {
                batch.print_color_right(
                    Point::new(box_rect.x2 - footer.len() as i32, box_rect.y2),
                    footer,
                    footer_color,
                );
            }
            Alignment::Center => {
                batch.print_color_centered_at(
                    Point::new(box_rect.x1 + box_rect.width() / 2, box_rect.y2),
                    footer,
                    footer_color,
                );
            }
        }
    }
}

pub fn box_with_title(batch: &mut DrawBatch, pt: Point, config: BoxConfigWithTitle) -> Rect {
    let BoxConfigWithTitle { box_config: BoxConfig { color, double, hollow, dimensions }, text_config } =
        config;

    let (box_w, box_h) = dimensions;

    assert!(pt.x >= 0);
    assert!(pt.y >= 0);
    assert!(box_w > 0);
    assert!(box_h > 0);

    let box_rect = Rect::with_size(pt.x, pt.y, box_w, box_h);

    draw_box(batch, box_rect, double, hollow, color);
    draw_title(batch, box_rect, text_config, config.box_config);

    box_rect
}

pub fn center_box(batch: &mut DrawBatch, screen_bounds: (i32, i32), config: BoxConfig) -> Rect {
    let BoxConfig { color, double, hollow, dimensions } = config;

    let (screen_w, screen_h) = screen_bounds;
    let (box_w, box_h) = dimensions;

    assert!(screen_w > box_w);
    assert!(screen_h > box_h);

    let start_x = (screen_w / 2) - (box_w / 2);
    let start_y = (screen_h / 2) - (box_h / 2);
    let end_x = box_w;
    let end_y = box_h;

    let box_rect = Rect::with_size(start_x, start_y, end_x, end_y);
    draw_box(batch, box_rect, double, hollow, color)
}

pub fn center_box_with_title(
    batch: &mut DrawBatch,
    screen_bounds: (i32, i32),
    config: BoxConfigWithTitle,
) -> Rect {
    let BoxConfigWithTitle { box_config: BoxConfig { color, double, hollow, dimensions }, text_config } =
        config;

    let (screen_w, screen_h) = screen_bounds;
    let (box_w, box_h) = dimensions;

    assert!(screen_w > box_w);
    assert!(screen_h > box_h);

    let start_x = (screen_w / 2) - (box_w / 2);
    let start_y = (screen_h / 2) - (box_h / 2);
    let end_x = box_w;
    let end_y = box_h;

    let box_rect = Rect::with_size(start_x, start_y, end_x, end_y);

    draw_box(batch, box_rect, double, hollow, color);
    draw_title(batch, box_rect, text_config, config.box_config);

    box_rect
}
