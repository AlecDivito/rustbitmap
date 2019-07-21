mod bitmap;

fn main() {
    let b1 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_2.bmp");
    println!("{}", b1);
    // let b2 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_gray.bmp");
    // println!("{}", b2);
    // let b3 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_256.bmp");
    // println!("{}", b3);
    // let b4 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home.bmp");
    // println!("{}", b4);
    b1.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp");
    let b2 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp");
    println!("{}", b2);
}
