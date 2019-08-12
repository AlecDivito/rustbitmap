
#[derive(Clone, Copy)]
pub struct Rgba
{
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Rgba
{
    pub fn black() -> Rgba
    {
        Rgba {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 100
        }
    }

    pub fn rgb(red: u8, green: u8, blue: u8) -> Rgba
    {
        Rgba {
            red,
            green,
            blue,
            alpha: 100
        }
    }

    pub fn bgr(blue: u8, green: u8, red: u8) -> Rgba
    {
        Rgba {
            red,
            green,
            blue,
            alpha: 100
        }
    }

    pub fn bgra(blue: u8, green: u8, red: u8, alpha: u8) -> Rgba
    {
        Rgba {
            red,
            green,
            blue,
            alpha: std::cmp::min(alpha, 100)
        }
    }

    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba
    {
        Rgba {
            red,
            green,
            blue,
            alpha: std::cmp::min(alpha, 100)
        }
    }

    pub fn recolor_to(&mut self, other: &Self)
    {
        self.red = other.red;
        self.green = other.green;
        self.blue = other.blue;
        self.alpha = other.alpha;
    }

    pub fn get_red(&self) -> u8
    {
        self.red
    }

    pub fn get_green(&self) -> u8
    {
        self.green
    }

    pub fn get_blue(&self) -> u8
    {
        self.blue
    }

    pub fn get_alpha(&self) -> u8
    {
        self.alpha
    }

    /**
     * Blur 2 color's together
     * 
     * By passing in different factors, you can effect how much one color will
     * effect the other.
     * 
     * TODO: Add ways for this method to fail
     * ex.
     *      lhs + rhs should = 1.0 or 1
     * 
     * @param {&Rgba} first color to blur
     * @param {f32} factor how much the first color will effect the outcome
     * @param {&Rgba} second color to blur
     * @param {f32} factor how much the second color will effect the outcome
     * @return {Rgba} new color of the 2 colors blurred together
     */
    pub fn blur(lhs: &Rgba, lhs_factor: f32, rhs: &Rgba, rhs_factor: f32) -> Rgba
    {
        // create the new colors for the left hand side
        let lhs_red   = lhs.red as f32 * lhs_factor;
        let lhs_green = lhs.green as f32 * lhs_factor;
        let lhs_blue  = lhs.blue as f32 * lhs_factor;
        let lhs_alpha = lhs.alpha as f32 * lhs_factor;
        // create the new colors for the right hand side
        let rhs_red   = rhs.red as f32 * rhs_factor;
        let rhs_green = rhs.green as f32 * rhs_factor;
        let rhs_blue  = rhs.blue as f32 * rhs_factor;
        let rhs_alpha = rhs.alpha as f32 * rhs_factor;
        // merge the 2 colors together
        Rgba {
            red:   (lhs_red + rhs_red).round() as u8,
            green: (lhs_green + rhs_green).round() as u8,
            blue:  (lhs_blue + rhs_blue).round() as u8,
            alpha: (lhs_alpha + rhs_alpha).round() as u8,
        }
    }

}

impl std::ops::Mul<f32> for Rgba
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red:   ((self.red as f32) * rhs).floor() as u8,
            green: ((self.green as f32) * rhs).floor() as u8,
            blue:  ((self.blue as f32) * rhs).floor() as u8,
            alpha: ((self.alpha as f32) * rhs).floor() as u8,
        }
    }
}

impl std::ops::Add for Rgba
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha
        }
    }
}

impl std::cmp::PartialEq for Rgba
{
    fn eq(&self, other: &Self) -> bool
    {
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue &&
        self.alpha == other.alpha
    }
}

impl std::fmt::Display for Rgba
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}, Alpha: {}",
            self.red,
            self.green,
            self.blue,
            self.alpha)
    }
}