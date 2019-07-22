mod bitmap;

fn main() {
    // let mut b1 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_2.bmp").unwrap();
    // println!("{}", b1);
    // let b3 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp").unwrap();
    // println!("{}", b3);


    // b1.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp", bitmap::bit_count::BitCount::BW);



    // let b2 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_gray.bmp");
    // println!("{}", b2);
    let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/example.bmp").unwrap();
    // let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/sm_ex.bmp").unwrap();
    // let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home.bmp").unwrap();
    println!("{}", b4);
    b4.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp", bitmap::bit_count::BitCount::BW).unwrap();


    let b2 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp").unwrap();
    println!("{}", b2);

}
