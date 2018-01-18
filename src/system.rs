extern crate rand;

use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Clone, Copy)]
pub struct System {
    previous_x: Option<f64>,
    x: f64,
    y: f64,
    px: f64,
    py: f64,
    c: f64,
    d: f64,
    h: f64
}

impl System {

    pub fn random(h: f64, c: f64, d: f64) -> System {
        let mut rng = rand::thread_rng();
        loop {
            let y_range = Range::new(-10.0, 10.0);
            let y = y_range.ind_sample(&mut rng);

            let py_range = Range::new(-10.0, 10.0);
            let py = py_range.ind_sample(&mut rng);

            let system_option = System::new(0.0, y, py, h, c, d);

            if let Some(system) = system_option {
                return system;
            }
        }
    }

    fn difference(system_1: System, system_2: System) -> f64 {
        ((system_1.x - system_2.x).powi(2)
            + (system_1.y - system_2.y).powi(2)
            + (system_1.px - system_2.px).powi(2)
            + (system_1.py - system_2.py).powi(2)
        ).sqrt()
    }

    pub fn get_exponent(&mut self) -> f64 {
        let time = 1000;
        let base = self;
        let perturbation = System::new(self.x, self.y * 0.99, self.py * 0.99, h, c, d)
            .expect("Small perturbation outside energy range");
        let start_difference = System::difference(self, perturbation);
        let base_later = base.skip(time).next().expect("No system");
        let perturbation_later = perturbation.skip(time).next().expect("No system");
        let end_difference = System::difference(base_later, perturbation_later);

        (end_difference / start_difference).ln() / time as f64
    }

    /// Creates a new system with energy H, position (x, y) and momentum
    /// (px, py), where px is calculated using the energy.
    pub fn new( x: f64, y: f64, py: f64, h: f64, c: f64, d: f64) -> Option<System> {
        let px_2 = h - py.powi(2) - y
            - c*((x.powi(2) + y.powi(2)).sqrt() - 1.0).powi(2);

        if px_2 < 0.0 {
            None
        } else {
            let px = px_2.sqrt();
            Some( System{ x, y, px, py, c, d, h, previous_x: None })
        }
    }

    /// Returns whether the system has crossed a certain value of x.
    pub fn has_crossed(&self, x: f64) -> bool {
        match self.previous_x {
            None => false,
            Some(x0) => x0 < x && self.x > x
        }
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn py(&self) -> f64 {
        self.py
    }
}

impl Iterator for System {
    type Item = System;

    fn next(&mut self) -> Option<System> {
        let length = (self.x.powi(2) + self.y.powi(2)).sqrt();
        self.previous_x = Some(self.x);
        self.px += -self.c * self.d * self.x * (1.0 - 1.0 / length);
        self.py += - 0.5 * self.d - self.c * self.d * self.y * (1.0 - 1.0 / length);
        self.x += self.d * self.px;
        self.y += self.d * self.py;

        // Return the system
        Some(self.clone())
    }
}