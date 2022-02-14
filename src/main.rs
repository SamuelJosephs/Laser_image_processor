fn main() {
    use image::io::Reader as ImageReader;
    use image::GenericImageView;
    use std::fs::*;
    use std::io::LineWriter;
    use std::io::Write;
    
    let img = ImageReader::open("/home/samuel/Documents/rli/image_processor/Images/test.jpeg").expect("Failed to open file").decode();

    let (width, height) = img.as_ref().unwrap().dimensions();
    println!("width = {}, height = {}",width,height);
    let mut coordinates = Vec::<(u32,u32)>::with_capacity(width as usize * height as usize); 

    'label: for x in (0..width) {
        for y in (0..height) {
            
            let pix = img.as_ref().unwrap().get_pixel(x,y);
            let channels = pix.0;

            if channels[0] > 160 {
                let coord = (x,height - y);
                coordinates.push(coord);
                continue 'label;    
            }
             

        }

    }

 

    let file = File::create("Results.csv").expect("Failed to open file");
    let mut file_write = LineWriter::new(file);
    for coord in coordinates {
       let x = coord.0;
       let y = coord.1;

       let x_str = x.to_string();
       let y_str = y.to_string();

       let mut output_string = String::with_capacity(width as usize * height as usize);
       output_string.push_str(&x_str);
       output_string.push_str(" ");
       output_string.push_str(&y_str);
       output_string.push_str("\n");
       file_write.write_all(output_string.as_bytes()).expect("failed to write to file");


            
     
    }

}

