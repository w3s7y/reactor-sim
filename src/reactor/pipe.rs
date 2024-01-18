use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Pipe {
    // All dimensions are in mm
    inner_diameter: u16,
    outer_diameter: u16,
    length: u16,
}

impl Pipe {
    pub fn new(inner_diameter: u16, outer_diameter: u16, length: u16) -> Pipe {
        Pipe {
            inner_diameter,
            outer_diameter,
            length,
        }
    }

    // getters for state in the struct
    pub fn get_length(self) -> u16 {
        return self.length
    }

    pub fn get_id(self) -> u16 {
        return self.inner_diameter
    }

    pub fn get_od(self) -> u16 {
        return self.outer_diameter
    }

    fn get_cross_sectional_area(self) -> f64 {
        // Private function
        // in mm2
        let calc = self.inner_diameter as f64 / 2 as f64;
        // Pass a ref
        return PI * (&calc * &calc);
    }

    pub fn get_volume(self) -> f64 {
        // in mL
        return self.get_cross_sectional_area() * self.length as f64/1000 as f64;
    }

}

#[cfg(test)]
mod tests {
    // Why do I need to use create:: ?!?!
    // find out
    use crate::reactor::pipe::Pipe;
    #[test]
    fn test_pipe_volume() {
        let pipe = Pipe::new(33, 36, 500);
        assert_eq!(pipe.get_volume(), 427.6492999699106);
    }
}
