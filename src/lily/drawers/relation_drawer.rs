use tui::{
    widgets::Widget,
    buffer::Buffer,
    buffer::Cell,
    layout::Rect,
    style::Color,
    style::Modifier,
};

pub struct RelationDrawer{
    pub x_off: f64,
    pub y_off: f64,
    pub x_zm : f64,
    pub y_zm : f64,

    pub dist : f64,
    pub function: fn(&f64, &f64) -> f64,
}

// why is it implemented like that
// I don't fucking know lmao
impl Widget for &RelationDrawer{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x_con = self.x_zm * (1.0 / area.width  as f64);
        let y_con = self.y_zm * (1.0 / area.height as f64);

        for i in 0..area.width/2{
            for j in 0..area.height{
                if i < 8 && j == 0{
                    continue;
                }
                let x = ((4 * i) as f64) * x_con + self.x_off / 2.0;
                let y = (j as f64) * y_con + self.y_off / 2.0;

                let result = (self.function)(&x, &y);
                let d = (255 as f64 * result / self.dist) as u8;
                let cell = Cell {
                    symbol: " ".to_string(),
                    fg: Color::Rgb(d, d, d),
                    bg: Color::Rgb(d, d, d),
                    modifier: Modifier::BOLD,
                };
                buf.get_mut(2 * i + 0, j).clone_from(&cell);
                buf.get_mut(2 * i + 1, j).clone_from(&cell);
            }
        }
    }
}
