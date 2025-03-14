pub struct PerlinNoise {
    permutation: [usize; 512],
}

impl PerlinNoise {
    pub fn new(seed: usize) -> Self {
        let mut perm = [0usize; 512];
        let mut p = [0usize; 256];

        for i in 0..256 {
            p[i] = i;
        }

        // Shuffle permutation table
        for i in (1..256).rev() {
            let j = seed % (i + 1); // Random index
            p.swap(i, j);
        }

        // Duplicate the permutation array
        for i in 0..256 {
            perm[i] = p[i];
            perm[i + 256] = p[i];
        }

        Self { permutation: perm }
    }

    pub fn noise(&self, x: f64, y: f64) -> f64 {
        let xi = x.floor() as isize & 255;
        let yi = y.floor() as isize & 255;
        let xf = x - x.floor();
        let yf = y - y.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);

        let aa = self.permutation[(self.permutation[xi as usize] + yi as usize) % 256];
        let ab = self.permutation[(self.permutation[xi as usize] + yi as usize + 1) % 256];
        let ba = self.permutation[(self.permutation[xi as usize + 1] + yi as usize) % 256];
        let bb = self.permutation[(self.permutation[xi as usize + 1] + yi as usize + 1) % 256];

        let grad_aa = Self::grad(aa, xf, yf);
        let grad_ba = Self::grad(ba, xf - 1.0, yf);
        let grad_ab = Self::grad(ab, xf, yf - 1.0);
        let grad_bb = Self::grad(bb, xf - 1.0, yf - 1.0);

        let lerp_x1 = Self::lerp(grad_aa, grad_ba, u);
        let lerp_x2 = Self::lerp(grad_ab, grad_bb, u);

        Self::lerp(lerp_x1, lerp_x2, v)
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + t * (b - a)
    }

    fn grad(hash: usize, x: f64, y: f64) -> f64 {
        let h = hash & 3;
        match h {
            0 => x + y,
            1 => -x + y,
            2 => x - y,
            _ => -x - y,
        }
    }
}
