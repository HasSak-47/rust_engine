use tui::{
    widgets::{Widget, Block, Borders,
        canvas::{
            MapResolution,
            Rectangle,
            Canvas,
            Line,
            Map,
        }
    },
    style::Color,
    layout::Rect,
    buffer::{Buffer, Cell}, symbols,
}; 

use std::vec;

pub struct FunctionDrawer{
    pub beg_x: f32,
    pub end_x: f32,
    pub beg_y: f32,
    pub end_y: f32,

    pub function: fn(&f32) -> f32, 
}

impl Widget for FunctionDrawer{
    fn render(self, area: Rect, buf: &mut Buffer){
        let mut lines = Vec::<Line>::new();
        let x_con = (self.end_x - self.beg_x) / (area.width as f32);
        for i in 0..area.width{
            let xs = [
                self.beg_x + (i as f32 * x_con),
                self.beg_x + ((i + 1) as f32 * x_con),
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
    /*
    fn render(self, area: Rect, buf: &mut Buffer){
        buf.clone_from(&Buffer::empty(area));
        let x_step = ((self.end_x - self.beg_x) / (area.width  as f32)).abs();
        for i in 0..area.width{
            let x = ((i + 1) as f32 * x_step) + self.beg_x;
            let y = (self.function)(&x);
        }
    }
    */
}

