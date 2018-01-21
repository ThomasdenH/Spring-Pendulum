extern crate rand;

use rand::distributions::{IndependentSample, Range};

/// Represents the spring pendulum system
#[derive(Debug, Clone, Copy)]
pub struct System {
    /// The coordinates of the system
    x: f64,
    y: f64,
    px: f64,
    py: f64,
    /// If this system has been evolved, contains the previous x coordinate
    previous_x: Option<f64>,
    /// Determines balance of the force
    c: f64,
    /// Determines the time step
    d: f64,
    /// The total energy of the system
    h: f64,
    /// The time since the first iteration
    t: f64,
}

impl System {

    /// A quick and dirty method to create a random system for a given energy, c, and time step.
    /// If no system can be made this function will take a long time to run.
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

    pub fn get_exponent(&mut self) -> f64 {
        let n: usize = 100_000;
        let n_start: usize = 1000;

        let d_0 = 0.0000001;
        let mut displacement = System {
            x: self.x,
            y: self.y + d_0,
            px: self.px,
            py: self.py,
            t: self.t,
            c: self.c,
            d: self.d,
            previous_x: self.previous_x,
            h: self.h
        };

        let mut sum = 0.0;
        for i in 0..n {
            self.next().unwrap();
            displacement.next().unwrap();
            let dx = displacement.x - self.x;
            let dy = displacement.y - self.y;
            let dpx = displacement.px - self.px;
            let dpy = displacement.py - self.py;
            let d_1 = (dx.powi(2) + dy.powi(2) + dpx.powi(2) + dpy.powi(2)).sqrt();
            displacement.x = self.x + dx / d_1 * d_0;
            displacement.y = self.y + dy / d_1 * d_0;
            displacement.px = self.px + dpx / d_1 * d_0;
            displacement.py = self.py + dpy / d_1 * d_0;
            if i > n_start {
                sum += (d_1 / d_0).ln();
            }
        }
        sum / (self.d * (n - n_start) as f64)
    }

    /// Creates a new system with energy H, position (x, y) and momentum (px, py), where px is
    /// calculated using the energy. If the energy is too low for a system with the given
    /// properties to exist, the function will return None.
    pub fn new( x: f64, y: f64, py: f64, h: f64, c: f64, d: f64) -> Option<System> {
        let px_2 = h - py.powi(2) - y
            - c*((x.powi(2) + y.powi(2)).sqrt() - 1.0).powi(2);

        if px_2 < 0.0 {
            None
        } else {
            let px = px_2.sqrt();
            Some( System{ x, y, px, py, c, d, h, previous_x: None, t: 0.0})
        }
    }

    /// Returns whether the system has crossed a certain value of x.
    pub fn has_crossed(&self, x: f64) -> bool {
        match self.previous_x {
            None => false,
            Some(x0) => x0 < x && self.x > x
        }
    }

    /// Get the y coordinate of the system
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the py coordinate of the system
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
        self.t += self.d;

        // Return the system
        Some(self.clone())
    }
}