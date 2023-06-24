use roopes::prelude::*;

#[cfg(test)]
mod test;

#[derive(Clone, Copy)]
struct Boxes(usize);

#[derive(Visitor, Clone, Copy)]
enum Vehicle
{
    Truck
    {
        boxes: Boxes,
    },
    Car,
    Bicycle,
}

struct CollideElementCollider
{
    subject: Vehicle,
}

impl VehicleVisitor for CollideElementCollider
{
    fn visit_truck(
        &self,
        contents: &Boxes,
    )
    {
        match self.subject {
            | Truck { boxes } => {
                println!(
                    "A truck collides sending {} boxes flying.",
                    contents.0 + boxes.0
                )
            }
            | Car => {
                println!(
                    "A truck rams a car, sending {} boxes flying",
                    contents.0
                )
            }
            | Bicycle => {
                println!("A truck rams a bicycle, injuring the person.")
            }
        }
    }

    fn visit_car(&self)
    {
        match self.subject {
            | Truck { boxes } => {
                println!("A truck rams a car, sending {} boxes flying", boxes.0)
            }
            | Car => {
                println!("Two cars damage each other's bumpers",)
            }
            | Bicycle => {
                println!("A car rams a bicycle, injuring the person.")
            }
        }
    }

    fn visit_bicycle(&self)
    {
        match self.subject {
            | Truck { .. } => {
                println!("A truck rams a bicycle, injuring the person.")
            }
            | Car => {
                println!("A car rams a bicycle, injuring the person.")
            }
            | Bicycle => {
                println!(
                    "Two bicycles hit each other, resulting in some bruising."
                )
            }
        }
    }
}

fn main()
{
    println!("Collision Sim");

    let truck = Vehicle::Truck { boxes: Boxes(2) };
    let bicycle = Vehicle::Bicycle;
    let car = Vehicle::Car;

    let mut collider =
        VehicleAcceptor::new(CollideElementCollider { subject: truck });

    collider.accept(&bicycle);
    collider.accept(&truck);
    collider.accept(&car);

    collider.delegate.subject = bicycle;

    collider.accept(&bicycle);
    collider.accept(&truck);
    collider.accept(&car);

    collider.delegate.subject = car;

    collider.accept(&bicycle);
    collider.accept(&truck);
    collider.accept(&car);
}
