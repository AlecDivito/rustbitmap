///
/// Rgba represents the colors red, green, blue, alpha. Alpha represents the
/// transparency of the image while red, green and blue represent the intensity
/// of the colors.
///
/// Alpha is managed between 0 - 100
/// Red Green and Blue is managed between 0 - 255
///
#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Rgba {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Rgba {
    ///
    /// Create the color white
    ///
    pub fn white() -> Rgba {
        Rgba {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 100,
        }
    }

    /// Check to see if a Rgba structure is black
    pub fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0 && self.alpha == 100
    }

    /// Check to see if a Rgba structure is white
    pub fn is_white(&self) -> bool {
        self.red == 255 && self.green == 255 && self.blue == 255 && self.alpha == 100
    }

    ///
    /// Create the color black
    ///
    pub fn black() -> Rgba {
        Rgba {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 100,
        }
    }

    ///
    /// Create a color by specifying red, green and blue
    ///
    pub fn rgb(red: u8, green: u8, blue: u8) -> Rgba {
        Rgba {
            red,
            green,
            blue,
            alpha: 100,
        }
    }

    ///
    /// Create a color by specifying blue, green and red
    ///
    pub fn bgr(blue: u8, green: u8, red: u8) -> Rgba {
        Rgba {
            red,
            green,
            blue,
            alpha: 100,
        }
    }

    ///
    /// Create a color by specifying blue, green, red and alpha
    ///
    pub fn bgra(blue: u8, green: u8, red: u8, alpha: u8) -> Rgba {
        Rgba {
            red,
            green,
            blue,
            alpha: std::cmp::min(alpha, 100),
        }
    }

    ///
    /// Create a color by specifying red, green, blue and alpha
    ///
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba {
        Rgba {
            red,
            green,
            blue,
            alpha: std::cmp::min(alpha, 100),
        }
    }

    ///
    /// Change the current color to the other color
    ///
    pub fn recolor_to(&mut self, other: &Self) {
        self.red = other.red;
        self.green = other.green;
        self.blue = other.blue;
        self.alpha = other.alpha;
    }

    ///
    /// Check if the alpha value is less then 100 (Visible)
    ///
    pub fn is_transparent(&self) -> bool {
        self.alpha < 100
    }

    ///
    /// get the amount of red in the pixel
    ///
    pub fn get_red(&self) -> u8 {
        self.red
    }

    ///
    /// get the amount of green in the pixel
    ///
    pub fn get_green(&self) -> u8 {
        self.green
    }

    ///
    /// get the amount of blue in the pixel
    ///
    pub fn get_blue(&self) -> u8 {
        self.blue
    }

    ///
    /// get the amount of alpha in the pixel
    ///
    pub fn get_alpha(&self) -> u8 {
        std::cmp::min(self.alpha, 100)
    }
}

///
/// Methods used to manipulate the rgba color in a certain way
///
impl Rgba {
    ///
    /// Convert pixel color to gray scale
    ///
    pub fn color_to_gray(&mut self) {
        let red_gray = self.red as f32 * 0.2126;
        let green_gray = self.green as f32 * 0.7152;
        let blue_gray = self.blue as f32 * 0.0722;
        let pixel_gray = (red_gray + green_gray + blue_gray).round() as u8;
        self.set_gray_scale_pixel(pixel_gray);
    }

    ///
    /// set all the colors to the same color
    ///
    fn set_gray_scale_pixel(&mut self, gray: u8) {
        self.red = gray;
        self.green = gray;
        self.blue = gray;
    }

    ///
    /// Blur 2 color's together
    ///
    /// By passing in different factors, you can effect how much one color will
    /// effect the other.
    ///
    /// @param {&Rgba} first color to linear_interpolation
    /// @param {f32} factor how much the first color will effect the outcome
    /// @param {&Rgba} second color to linear_interpolation
    /// @param {f32} factor how much the second color will effect the outcome
    /// @return {Rgba} new color of the 2 colors blurred together
    ///
    pub fn linear_interpolation(
        lhs: &Rgba,
        lhs_factor: f32,
        rhs: &Rgba,
        rhs_factor: f32,
    ) -> Result<Rgba, &'static str> {
        if lhs_factor + rhs_factor > 1.0 || lhs_factor + rhs_factor < 0.0 {
            return Err("Error blurring colors, factors should be able to add up to 1.");
        }
        // create the new colors for the left hand side
        let lhs_red = lhs.red as f32 * lhs_factor;
        let lhs_green = lhs.green as f32 * lhs_factor;
        let lhs_blue = lhs.blue as f32 * lhs_factor;
        let lhs_alpha = lhs.alpha as f32 * lhs_factor;
        // create the new colors for the right hand side
        let rhs_red = rhs.red as f32 * rhs_factor;
        let rhs_green = rhs.green as f32 * rhs_factor;
        let rhs_blue = rhs.blue as f32 * rhs_factor;
        let rhs_alpha = rhs.alpha as f32 * rhs_factor;
        // merge the 2 colors together
        Ok(Rgba {
            red: (lhs_red + rhs_red).round() as u8,
            green: (lhs_green + rhs_green).round() as u8,
            blue: (lhs_blue + rhs_blue).round() as u8,
            alpha: (lhs_alpha + rhs_alpha).round() as u8,
        })
    }

    ///
    /// Use cubic interpolation on 4 colors between range [0, 1] then find x as
    /// a factor that is between [0, 1] (ex. 0.5)
    ///
    /// Reference:
    /// https://www.paulinternet.nl/?page=bicubic
    ///
    pub fn cubic_interpolate(p0: &Rgba, p1: &Rgba, p2: &Rgba, p3: &Rgba, factor: f32) -> Rgba {
        if factor == 0.0 {
            return p1.clone();
        }
        let a_red =
            -0.5 * p0.red as f32 + 1.5 * p1.red as f32 - 1.5 * p2.red as f32 + 0.5 * p3.red as f32;
        let a_green = -0.5 * p0.green as f32 + 1.5 * p1.green as f32 - 1.5 * p2.green as f32
            + 0.5 * p3.green as f32;
        let a_blue = -0.5 * p0.blue as f32 + 1.5 * p1.blue as f32 - 1.5 * p2.blue as f32
            + 0.5 * p3.blue as f32;
        let a_alpha = -0.5 * p0.alpha as f32 + 1.5 * p1.alpha as f32 - 1.5 * p2.alpha as f32
            + 0.5 * p3.alpha as f32;

        let b_red = p0.red as f32 - 2.5 * p1.red as f32 + 2.0 * p2.red as f32 - 0.5 * p3.red as f32;
        let b_green =
            p0.green as f32 - 2.5 * p1.green as f32 + 2.0 * p2.green as f32 - 0.5 * p3.green as f32;
        let b_blue =
            p0.blue as f32 - 2.5 * p1.blue as f32 + 2.0 * p2.blue as f32 - 0.5 * p3.blue as f32;
        let b_alpha =
            p0.alpha as f32 - 2.5 * p1.alpha as f32 + 2.0 * p2.alpha as f32 - 0.5 * p3.alpha as f32;

        let c_red = -0.5 * p0.red as f32 + 0.5 * p2.red as f32;
        let c_green = -0.5 * p0.green as f32 + 0.5 * p2.green as f32;
        let c_blue = -0.5 * p0.blue as f32 + 0.5 * p2.blue as f32;
        let c_alpha = -0.5 * p0.alpha as f32 + 0.5 * p2.alpha as f32;

        let d_red = p1.red as f32;
        let d_green = p1.green as f32;
        let d_blue = p1.blue as f32;
        let d_alpha = p1.alpha as f32;

        let red =
            a_red * (factor * factor * factor) + b_red * (factor * factor) + c_red * factor + d_red;
        let green = a_green * (factor * factor * factor)
            + b_green * (factor * factor)
            + c_green * factor
            + d_green;
        let blue = a_blue * (factor * factor * factor)
            + b_blue * (factor * factor)
            + c_blue * factor
            + d_blue;
        let alpha = a_alpha * (factor * factor * factor)
            + b_alpha * (factor * factor)
            + c_alpha * factor
            + d_alpha;

        // clamp values
        let red = if red > 255.0 || red < 0.0 {
            if red > 255.0 {
                255
            } else {
                0
            }
        } else {
            red.round() as u8
        };
        let green = if green > 255.0 || green < 0.0 {
            if green > 255.0 {
                255
            } else {
                0
            }
        } else {
            green.round() as u8
        };
        let blue = if blue > 255.0 || blue < 0.0 {
            if blue > 255.0 {
                255
            } else {
                0
            }
        } else {
            blue.round() as u8
        };
        let alpha = if alpha > 100.0 || alpha < 0.0 {
            if alpha > 100.0 {
                100
            } else {
                0
            }
        } else {
            alpha.round() as u8
        };

        Rgba {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl PartialEq for Rgba {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Red: {}, Green: {}, Blue: {}, Alpha: {}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[cfg(test)]
mod test {
    use super::Rgba;

    #[test]
    fn is_pixel_transparent() {
        let pixel = Rgba::rgba(0, 0, 0, 0);
        assert_eq!(true, pixel.is_transparent());
        let pixel1 = Rgba::rgba(0, 0, 0, 99);
        assert_eq!(true, pixel1.is_transparent());
        let pixel2 = Rgba::rgba(0, 0, 0, 100);
        assert_eq!(false, pixel2.is_transparent());
        let pixel3 = Rgba::rgba(0, 0, 0, 101);
        assert_eq!(false, pixel3.is_transparent());
    }

    #[test]
    fn test_blur_sent_bad_factors() {
        let white = Rgba::white();
        let black = Rgba::black();
        let linear_interpolation = Rgba::linear_interpolation(&white, 1.0, &black, 1.0);
        assert!(linear_interpolation.is_err());
    }

    #[test]
    fn test_blur_two_whites() {
        let white1 = Rgba::white();
        let white2 = Rgba::white();
        let linear_interpolation = Rgba::linear_interpolation(&white1, 0.5, &white2, 0.5);
        assert!(linear_interpolation.is_ok());
        assert!(linear_interpolation.unwrap() == Rgba::white());
    }

    #[test]
    fn test_blur_correct_color() {
        let white = Rgba::white();
        let black = Rgba::black();
        let linear_interpolation = Rgba::linear_interpolation(&white, 0.5, &black, 0.5);
        let gray = Rgba::rgb(128, 128, 128);
        assert!(linear_interpolation.is_ok());
        assert!(linear_interpolation.unwrap() == gray);
    }
}
