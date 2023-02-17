// use math::
// use obj::{Obj, load_obj};
// 
// use std::io::BufReader;
// 
// pub fn load(_path: &str) -> (Vec<LocalVertex>, Vec<u16>) {
//     let input = BufReader::new(std::fs::File::open("/home/lilith/Swiedowski-Engine/test_assets/untitled.obj").unwrap());
//     let obj: Obj = match load_obj(input){
//         Result::Ok(o) => o,
//         Result::Err(e) => panic!("{}", e),
//     };
// 
//     let mut vrtc = Vec::<LocalVertex>::new();
//     for vertex in obj.vertices{
//         vrtc.push(LocalVertex::from(vertex));
//     }
// 
//     (vrtc, obj.indices)
// }
// 
