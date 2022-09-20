use core::primitive::str;
use tui::{
    widgets::Widget,
    buffer::Buffer,
    layout::Rect,
};

static SYMBOLS: &str = "#0Oo°. ";
//static SYMBOLS: &str = "ÆÑÊŒØMÉËÈÃÂWQBÅæ#NÁþEÄÀHKRŽœXgÐêqÛŠÕÔA€ßpmãâG¶øðé8ÚÜ$ëdÙýèÓÞÖåÿÒb¥FDñáZPäšÇàhû§ÝkŸ®S9žUTe6µOyxÎ¾f4õ5ôú&aü™2ùçw©Y£0VÍL±3ÏÌóC@nöòs¢u‰½¼‡zJƒ%¤Itocîrjv1lí=ïì<>i7†[¿?×}*{+()\\/»«•¬|!¡÷¦¯—^ª„”“~³º²–°­¹‹›;:’‘‚’˜ˆ¸…·¨´` ";
static mut SYM_VEC: Vec<String> = Vec::new();
static mut SYM_LEN: usize = SYMBOLS.len(); 

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
        SYM_LEN = SYMBOLS.chars().count();
        for i in 0..SYM_LEN{
            let push_str: String;
            match SYMBOLS.chars().nth(i){
                None => push_str = "X".to_string(),
                Some(s) => push_str = s.to_string(),
            }
            for j in 0..i{
                SYM_VEC.push(push_str.to_string());
            }
        }
        SYM_LEN = SYM_VEC.len();
    }
}

impl Widget for &RelationDrawer{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x_con = self.x_zm * (1.0 / area.width  as f64);
        let y_con = self.y_zm * (1.0 / area.height as f64);

        for i in 0..area.width{
            for j in 0..area.height{
                let x = (i as f64) * x_con + self.x_off;
                let y = (j as f64) * y_con + self.y_off;

                let mut d: usize;
                unsafe {
                    d = (SYM_LEN as f64 * (self.function)(&x, &y) / self.dist) as usize;
                    if d >= SYM_LEN{
                        d = SYM_LEN - 1;
                    }
                    buf.get_mut(i, j).symbol = SYM_VEC.get(d).unwrap().clone();
                }
            }
        }
    }
}
