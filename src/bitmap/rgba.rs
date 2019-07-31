
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
    pub fn rgb(red: u8, green: u8, blue: u8) -> Rgba
    {
        Rgba {
            red,
            green,
            blue,
            alpha: 100
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