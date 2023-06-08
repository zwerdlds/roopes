use crate::{
    vehicle::Vehicle,
    vehicle_visitor::VehicleVisitor,
};
use ropes_lib::prelude::*;

pub(crate) struct Truck {}

impl visitor::Element<VehicleVisitor, Box<dyn Vehicle>> for Truck
{
    fn accept(
        &self,
        visitor: VehicleVisitor,
    )
    {
        todo!()
    }
}
