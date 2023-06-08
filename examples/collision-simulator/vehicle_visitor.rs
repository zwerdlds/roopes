use crate::vehicle::Vehicle;
use ropes_lib::prelude::Visitor;

pub(crate) struct VehicleVisitor {}

impl Visitor<Box<dyn Vehicle>, Box<dyn Vehicle>> for VehicleVisitor {}
