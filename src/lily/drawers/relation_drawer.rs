use tui::{
    widgets::Widget,
    buffer::Buffer,
    buffer::Cell,
    layout::Rect,
    style::Color,
    style::Modifier,
};

static mut SYM_VEC: Vec<Cell> = Vec::new();
static mut SYM_LEN: usize = 0;

pub struct RelationDrawer{
    pub x_off: f64,
    pub y_off: f64,
    pub x_zm : f64,
    pub y_zm : f64,

    pub dist : f64,
    pub function: fn(&f64, &f64) -> f64,
}

pub fn setup() {
    unsafe {
        for i in 0..8{
            let inte: u8 = u8::MAX >> i;
            let col: Color = Color::Rgb(inte, inte, inte);
            let modif: Modifier = Modifier::BOLD;
            SYM_VEC.push(Cell { symbol: " ".to_string(), fg: col, bg: col, modifier: modif})
        }
        SYM_VEC.reverse();
        SYM_LEN = SYM_VEC.len();
    }
}

impl Widget for &RelationDrawer{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x_con = self.x_zm * (1.0 / area.width  as f64);
        let y_con = self.y_zm * (1.0 / area.height as f64);

        for i in 0..area.width{
            for j in 0..area.height{
                let x = (i as f64) * x_con + self.x_off / 2.0;
                let y = (j as f64) * y_con + self.y_off / 2.0;

                let result = (self.function)(&x, &y);
                let d = (255 as f64 * (result * result) / self.dist) as u8;
                let cell = Cell {
                    symbol: " ".to_string(),
                    fg: Color::Rgb(d, d,  d),
                    bg: Color::Rgb(d, d, d),
                    modifier: Modifier::BOLD,
                };
                buf.get_mut(i, j).clone_from(&cell);
            }
        }
    }
}
