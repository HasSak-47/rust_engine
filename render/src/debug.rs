use ash;

pub const VALIDATION_LAYERS : &[*const i8] = &[
    "VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8,
];
pub const ENABLE_VALIDATION : bool = true;

fn print_i8arr(ar: &[i8]){
    for a in ar{
        let a = *a as u8 as char;
        if a == '\0' {break}
        print!("{a}");
    }
}

fn print_i8ptr(ar: *const i8){
    let ar = ar as usize;
    let mut i = 0;
    loop {
        let a =  unsafe { *((ar + i) as *const i8) } as u8 as char;
        if a == '\0'{break;}
        print!("{a}");
        i += 1;

    }
}

fn cmp_str(a: *const i8, b: *const i8) -> bool{
    let a = a as usize; let b = b as usize;
    let mut i = 0;
    loop {
        let (da, db) = unsafe{
            (*((a + i) as *const i8) as u8 as char,
             *((b + i) as *const i8) as u8 as char)
        };
        if da != db    {return false;}
        if da == '\0'  {return true;}

        i += 1;
    }

}

pub fn check_validation_layer_support(entry: &ash::Entry) -> bool{
    let layer_properties = entry.enumerate_instance_layer_properties()
        .expect("Failed to enumerate_instance_layer_properties");

    if layer_properties.len() <= 0{
        eprintln!("No available layers!");
        return false;
    }

    println!("found layers:");
    for val_layer in layer_properties.iter(){
        let name = val_layer.layer_name;
        print_i8arr(&name); println!();
    }

    for val_layer in VALIDATION_LAYERS{

        let mut found = false;
        for aval_layer in layer_properties.iter(){
            let aval_name = aval_layer.layer_name.as_ptr();
            if cmp_str(aval_name, *val_layer) {
                found = true;
                break;
            }
        }

        if !found{
            eprint!("could not find: ");
            print_i8ptr(*val_layer); println!();
            return false;
        }
    }


    true
}

pub fn get_validation_layers() -> (*const *const i8, u32){
    if ENABLE_VALIDATION {
        (VALIDATION_LAYERS.as_ptr(), VALIDATION_LAYERS.len() as u32)
    }
    else {
        (std::ptr::null(), 0)
    }
}
