struct PointLight {
    x : f32,
    y : f32,
}

struct LightSystem {
    lights : Vec<PointLight>
}

struct RenderSystem {
    entities : Vec<i32>  
}

trait Update {
    fn update(&self) ;
}

impl Update for RenderSystem {
    fn update(&self) {
        for e in &self.entities {
            println!("Entity {}", e);
        }
    }
}

impl Update for LightSystem {
    fn update(&self) {
        for l in &self.lights {
            println!("Light x: {} y: {}", l.x, l.y);
        }
    }
}

impl LightSystem {
    fn new() -> LightSystem {
        LightSystem {
            lights : Vec::new(),
        }
    }

    fn add(&mut self, _light : PointLight) {
        &self.lights.push(_light);
    }
}

impl PointLight { 
    fn new( xcoord : f32 , ycoord : f32) -> PointLight {
        PointLight {
            x : xcoord,
            y : ycoord,
        }
    }
}

struct Texture {
    file : String,
    // tex : ID3D11Texture,

}


enum ShaderType {
    EVertex, 
    EFragment,
    EGeometry,
    EHull,
    EDomain,
    ECompute,
}

struct Shader {
    m_Type : ShaderType,
    data : *const i8,
}

#[derive(Default)]
struct Effect {
    shaders : Vec<Shader>,
}

#[derive(Default)]
struct Model {
    vertices : Vec<f32>,
    indices : Vec<u32>,
    effect : Effect,
}

trait Render {
    fn render(&self);
}

impl Render for Model {
    fn render(&self) {
        println!("rendering this");
    }
}

impl Shader {
    fn new(shader_type : ShaderType, shader_data : *const i8) -> Shader {
        Shader {
            m_Type : shader_type,
            data : shader_data,
        }
    }
}

impl Effect {
    fn new() -> Effect {
        Effect {
            shaders : Vec::new(),
        }
    }
}

trait AddEffect {
    fn add_effect(&mut self, effect : Effect);
}

trait AddShader {
    fn add_shader(&mut self, shader : Shader);
}

impl AddShader for Effect {
    fn add_shader (&mut self, shader : Shader ) {
        self.shaders.push(shader);
    }
}

impl AddEffect for Model {
    fn add_effect(&mut self, effect : Effect) {
        self.effect = effect;
    }     
}

impl Model {
    fn new() -> Model {
        Model {
            vertices : Vec::new(),
            indices : Vec::new(),
            effect : Effect::new(),
        }
    }
}

include!("vector.rs");

fn main() {
    println!("Hello, world!");

    let mut ls = LightSystem::new();
    ls.add(PointLight::new(0.0, 10.0));

    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);

    let mut model = Model::new();
    model.add_effect(Effect::new());
    
    let data : i8 = 10;
    let pData : *const i8 = &data;
    model.effect.add_shader(Shader::new(ShaderType::EVertex, pData));
    

    let _model2 : Model = Default::default();

    let apa2 = 123;
    let mut apa = "hello World";
    let s : String = apa2.to_string();

    

    // println!("{}", apa);
    // let apa = &apa2.to_string();
    // println!("{}", apa);

    // println!("{}", apa);
    // apa = std::String("hello world");
    // println!("{}", apa);


    model.render();


    // let _vec20 = Vector2::new( 1.0, 2.0 );
    // let _vec21 = Vector2::new( 3.0, 9.0 );

    let _vec20 : Vector2<f32>;
    let _vec21 : Vector2<f32>;

    let _vec22 = _vec20 + _vec21;

    println!("{}, {}", _vec22.x, _vec22.y);

    for n in vec {
        println!("{}", n);
    }

    ls.update();
}