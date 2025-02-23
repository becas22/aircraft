use crate::simulation::{
    InitContext, Read, SimulationElement, SimulationElementVisitor, SimulatorReader,
    VariableIdentifier,
};
use nalgebra::Vector3;
use num_traits::Zero;
use uom::si::{f64::Mass, mass::kilogram};

pub const FUEL_GALLONS_TO_KG: f64 = 3.039075693483925;

pub trait FuelPayload {
    fn total_load(&self) -> Mass;
    fn fore_aft_center_of_gravity(&self) -> f64;
}

pub trait FuelCG {
    fn center_of_gravity(&self) -> Vector3<f64>;
}
#[derive(Debug)]
pub struct FuelInfo<'a> {
    pub fuel_tank_id: &'a str,
    pub position: (f64, f64, f64),
}

#[derive(Debug)]
pub struct FuelTank {
    fuel_id: VariableIdentifier,
    location: Vector3<f64>,
    quantity: Mass,
}
impl FuelTank {
    pub fn new(context: &mut InitContext, id: &str, location: Vector3<f64>) -> Self {
        FuelTank {
            fuel_id: context.get_identifier(id.to_owned()),
            location,
            quantity: Mass::default(),
        }
    }

    pub fn location(&self) -> Vector3<f64> {
        self.location
    }

    pub fn quantity(&self) -> Mass {
        self.quantity
    }
}
impl SimulationElement for FuelTank {
    fn read(&mut self, reader: &mut SimulatorReader) {
        let volume: f64 = reader.read(&self.fuel_id);
        self.quantity = Mass::new::<kilogram>(volume * FUEL_GALLONS_TO_KG);
    }
}

pub struct FuelSystem<const N: usize> {
    unlimited_fuel_id: VariableIdentifier,
    unlimited_fuel: bool,

    fuel_total_weight_id: VariableIdentifier,
    fuel_total_weight: Mass,

    fuel_tanks: [FuelTank; N],
}
impl<const N: usize> FuelSystem<N> {
    pub fn new(context: &mut InitContext, fuel_tanks: [FuelTank; N]) -> Self {
        FuelSystem {
            unlimited_fuel_id: context.get_identifier("UNLIMITED FUEL".to_owned()),
            unlimited_fuel: false,
            fuel_total_weight_id: context.get_identifier("FUEL TOTAL QUANTITY WEIGHT".to_owned()),
            fuel_total_weight: Mass::default(),
            fuel_tanks,
        }
    }

    pub fn total_load(&self) -> Mass {
        self.fuel_total_weight
    }

    pub fn tank_has_fuel(&self, t: usize) -> bool {
        self.unlimited_fuel || !self.fuel_tanks[t].quantity().is_zero()
    }

    pub fn center_of_gravity(&self) -> Vector3<f64> {
        let positions = self.fuel_tanks.iter().map(|t| t.location());
        let masses = self.fuel_tanks.iter().map(|t| t.quantity());

        // This section of code calculates the center of gravity (assume center of gravity/center of mass is near identical)
        let total_mass_kg = self.fuel_total_weight.get::<kilogram>();
        if total_mass_kg > 0. {
            positions
                .zip(masses)
                .map(|(pos, m)| pos * m.get::<kilogram>())
                .fold(Vector3::zeros(), |acc, x| acc + x)
                / total_mass_kg
        } else {
            Vector3::zeros()
        }
    }
}
impl<const N: usize> SimulationElement for FuelSystem<N> {
    fn accept<T: SimulationElementVisitor>(&mut self, visitor: &mut T) {
        accept_iterable!(self.fuel_tanks, visitor);
        visitor.visit(self);
    }

    fn read(&mut self, reader: &mut SimulatorReader) {
        self.unlimited_fuel = reader.read(&self.unlimited_fuel_id);
        self.fuel_total_weight = reader.read(&self.fuel_total_weight_id);
    }
}
