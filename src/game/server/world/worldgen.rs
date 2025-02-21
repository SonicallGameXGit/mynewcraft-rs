use std::{f64, num::Wrapping};
use noise::{NoiseFn, Perlin, Seedable};

use crate::game::common::coords::BlockAxis;

pub struct WorldGen {
    perlin: Perlin,
}

impl WorldGen {
    pub fn random_seed() -> u32 {
        rand::random::<u32>()
    }
    pub fn create(seed: u32) -> Self {
        Self { perlin: Perlin::new(seed) }
    }

    fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    pub fn get_plains_height(&self, x: BlockAxis, z: BlockAxis) -> f64 {
        (self.perlin.get([x as f64 * 0.01, z as f64 * 0.01]) * 0.5 + 0.5) * 24.0 + 32.0
    }
    pub fn get_mountains_height(&self, x: BlockAxis, z: BlockAxis) -> f64 {
        let base_height = (self.perlin.get([x as f64 * 0.015, z as f64 * 0.015]) * 0.5 + 0.5) * 128.0;
        let peaks_height = (self.perlin.get([x as f64 * 0.07, z as f64 * 0.07]) * 0.5 + 0.5) * 24.0;

        base_height + peaks_height + 32.0
    }

    pub fn get_height(&self, x: BlockAxis, z: BlockAxis) -> f64 {
        let mixer = Self::smoothstep(
            -0.5, 0.8,
            self.perlin.get([x as f64 * 0.007,z as f64 * 0.007])
        );

        let plains_height = self.get_plains_height(x, z);
        let mountains_height = self.get_mountains_height(x, z);

        plains_height + (mountains_height - plains_height) * mixer
    }

    pub fn get_random(&self, x: BlockAxis, z: BlockAxis) -> f64 {
        self.hash_coords(x, z) as f64 / u64::MAX as f64
    }

    fn hash_coords(&self, x: BlockAxis, z: BlockAxis) -> u64 {
        let mut hash = Wrapping(self.perlin.seed() as u64);
    
        hash ^= Wrapping(x as u64).0.wrapping_mul(0x517cc1b727220a95);
        hash ^= Wrapping(z as u64).0.wrapping_mul(0x6d8a9b5e8377e61d);
        hash ^= hash >> 33;
        hash *= Wrapping(0xff51afd7ed558ccd);
        hash ^= hash >> 33;
        hash *= Wrapping(0xc4ceb9fe1a85ec53);
        hash ^= hash >> 33;
    
        hash.0
    }    
}

// pub struct Biome {
// 	axes: [[Option<Rc<Biome>>; 2]; 2],
// }

// impl Biome {
// 	pub fn default() -> Self {
// 		Self {
// 			axes: [[None, None], [None, None]]
// 		}
// 	}

// 	pub fn set_cold_dry(mut self, biome: Self) -> Self {
// 		self.axes[0][0] = Some(Rc::new(biome));
// 		self
// 	}
// 	pub fn set_hot_dry(mut self, biome: Self) -> Self {
// 		self.axes[1][0] = Some(Rc::new(biome));
// 		self
// 	}
// 	pub fn set_cold_wet(mut self, biome: Self) -> Self {
// 		self.axes[0][1] = Some(Rc::new(biome));
// 		self
// 	}
// 	pub fn set_hot_wet(mut self, biome: Self) -> Self {
// 		self.axes[1][1] = Some(Rc::new(biome));
// 		self
// 	}

// 	pub fn get(&self, x: usize, y: usize) -> &Option<Rc<Self>> {
// 		if x > 1 || y > 1 {
// 			return &None
// 		}

// 		&self.axes[x][y]
// 	}
// }

// pub fn test() {
// 	let desert = Biome::default();
// 	let ice_peaks = Biome::default();

// 	let taiga = Biome::default()
// 		.set_cold_wet(ice_peaks);
// 	let plains = Biome::default()
// 		.set_hot_dry(desert)
// 		.set_cold_wet(taiga);


// }

// 3, 9

// # Temperature
// Plains(0)
// 	if <:
// 		Desert(6)
// 	else:
// 		Taiga(-3)

// Plains(0, 0)
// 	if x < V:
// 		if y < V:
// 			Desert(6, -5)
// 		else:
// 			Savanna(4, 12)
// 	else:
// 		if y < V:
// 			Taiga(-3, 12)
// 		else:
// 			IcePeaks(-4, -9)