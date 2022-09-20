use tui::{
    widgets::{
        Widget,
        canvas::{
            Canvas,
            Line,
        }
    },
    style::Color,
    layout::Rect,
    buffer::Buffer,
    symbols,
}; 

use std::vec::Vec;

pub struct FunctionDrawer{
    pub beg_x: f64,
    pub end_x: f64,
    pub beg_y: f64,
    pub end_y: f64,

    pub function: fn(&f64) -> f64, 
}

impl Widget for FunctionDrawer{
    fn render(self, area: Rect, buf: &mut Buffer){
        let mut lines = Vec::<Line>::new();
        let x_con = (self.end_x - self.beg_x) / (area.width as f64);
        for i in 0..area.width{
            let xs = [
                self.beg_x + (i as f64 * x_con),
                self.beg_x + ((i + 1) as f64 * x_con),
            ];
            let ys = [
                (self.function)(&xs[0]),
                (self.function)(&xs[1]),
            ];

            (&mut lines).push(Line {
                x1: xs[0] as f64,
                x2: xs[1] as f64,
                y1: ys[0] as f64,
                y2: ys[1] as f64,

                color: Color::White,
            })

        }
        let c = Canvas::default()
            .x_bounds([self.beg_x as f64, self.end_x as f64])
            .y_bounds([self.beg_y as f64, self.end_y as f64])
            .marker(symbols::Marker::Dot)
            .paint(|ctx| {
                for line in &lines{
                    ctx.draw(line);
                }
            })
        ;
        c.render(area, buf);
    }
}
