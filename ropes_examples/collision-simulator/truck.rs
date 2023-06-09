use crate::{
    vehicle::Vehicle,
    vehicle_visitor::VehicleVisitor,
};
use ropes::prelude::*;

pub(crate) struct Truck {}

impl visitor::Element<VehicleVisitor, Vehicle> for Truck
{
    fn accept(
        &self,
        visitor: VehicleVisitor,
    )
    {
        todo!()
    }
}
