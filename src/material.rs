use std::fmt::Display;

type pounds_per_cubic_inch = f32;
type pounds_per_square_inch = f32;
type kips_per_square_inch = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub name: &'static str,
    pub color: [f32; 3],
    pub density: pounds_per_cubic_inch,
    pub yield_strength: pounds_per_square_inch,
    pub ultimate_strength: pounds_per_square_inch,
    pub modulus_of_elasticity: kips_per_square_inch,
    pub poissons_ratio: f32
}

impl Material {
    pub const ALUMINUM_6061: Material = Material {
        name: "6061 T6 Aluminum",
        color: [0.8, 0.8, 0.9],
        density: 0.0975,
        yield_strength: 40000.0,
        ultimate_strength: 45000.0,
        modulus_of_elasticity: 10000.0,
        poissons_ratio: 0.33
    };

    pub const ALUMINUM_7075: Material = Material {
        name: "7075 T6 Aluminum",
        color: [0.8, 0.8, 0.9],
        density: 0.102,
        yield_strength: 73000.0,
        ultimate_strength: 83000.0,
        modulus_of_elasticity: 10400.0,
        poissons_ratio: 0.33
    };
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Material {{ name: {}, density: {}, yield_strength: {}, ultimate_strength: {}, modulus_of_elasticity: {}, poissons_ratio: {} }}",
            self.name,
            self.density,
            self.yield_strength,
            self.ultimate_strength,
            self.modulus_of_elasticity,
            self.poissons_ratio
        )
    }
}
