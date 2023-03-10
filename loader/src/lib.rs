use obj::{self, ObjData};

#[derive(Debug)]
pub struct Vertex{
    pub pos: [f32; 3],
    pub nor: [f32; 3],
    pub tex: [f32; 2],
}

#[derive(Debug)]
pub struct Model{ 
    pub vertices: Vec<Vertex>,
    pub indices : Vec<usize>,
}


#[deprecated(note="no it's not lmao, just not finished")]
pub fn load_model(path: &str) -> Model{
    let file = std::fs::File::open(path).unwrap();
    let objs = ObjData::load_buf(file).unwrap();

    let mut model = Model{
        vertices: Vec::new(),
        indices : Vec::new(),
    };

    // nightmare 
    for obj in objs.objects{
        for group in obj.groups{
            for poly in group.polys{
                for index in poly.0{
                    model.vertices.push( Vertex {
                        pos: objs.position[index.0],
                        nor: if index.1.is_some() { objs.normal[index.2.unwrap()] } else {[0., 0., 0.]},
                        tex: if index.2.is_some() { objs.texture[index.1.unwrap()] } else {[0., 0.]},
                    });
                }
            }
        }
    }

    model
}

/*
fn cast_vertex(obj: &ObjData, index: usize) -> Vertex {
    Vertex{
        pos: obj.position[index.0],
        nor 
    }
}
*/

pub fn load_vertices(path: &str) -> Vec<Vertex>{
    let file = std::fs::File::open(path).unwrap();
    let objs = ObjData::load_buf(file).unwrap();

    let mut vertices = Vec::new();

    // oh god, why have you forsaken us?
    for obj in objs.objects{
        for group in obj.groups{
            for poly in group.polys{
                for index in poly.0{
                    println!("pushing!");
                    vertices.push( Vertex {
                        pos: objs.position[index.0],
                        nor: if index.1.is_some() { objs.normal[index.2.unwrap()] } else {[0., 0., 0.]},
                        tex: [0., 0.],
                    });
                }
            }
        }
    }

    vertices

}
