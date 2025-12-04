// Playing with trais

struct Car {
    name: String,
    color: String,
    model: String,
}

impl Car 
{
    fn new() -> Self 
    {
        Self {
            name: "None".into(),
            color: "None".into(),
            model: "None".into(),
        }
    }

    fn from(name: &str, color: &str, model: &str) -> Self 
    {
        Self {
            name: name.into(),
            color: color.into(),
            model: model.into(),
        }
    }

    fn set_name(&mut self, name: &str) -> &mut Self 
    {
        self.name = name.into();
        self
    }

    fn set_color(&mut self, color: &str) -> &mut Self 
    {
        self.color = color.into();
        self
    }

    fn set_model(&mut self, model: &str) -> &mut Self 
    {
        self.model = model.into();
        self
    }
}

// Trait only for behaviors
trait Printable 
{
    fn print(&self);
}

impl Printable for Car 
{
    fn print(&self) 
    {
        println!("Car {}\nModel: {}\nColor {}\n",
            self.name, self.model, self.color
        );
    }
}

fn main() 
{
    let mut corolla = Car::new();
    let mut bmw = Car::from("BMW", "Black", "");
    
    corolla.set_name("Corolla").set_model("Altis").set_color("White");
    bmw.set_model("320i");

    bmw.print();
    corolla.print();
}
