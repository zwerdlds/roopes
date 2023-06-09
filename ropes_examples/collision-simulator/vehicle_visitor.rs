use crate::truck::Truck;
use ropes::prelude::*;

pub(crate) struct VehicleVisitor {}

impl Visitor<Truck> for VehicleVisitor
{
    fn visit(element: Truck)
    {
        todo!()
    }
}
