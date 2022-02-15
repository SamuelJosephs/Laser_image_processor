# Laser_image_processor
A program to process an image and output coordinates of it's path

This is written for a university project.
This program takes an input image of a red laser (It will not work if it is not red) and outputs the coordinates of the pixels of the lasers path.

Input image:
![test](Images/test.jpeg)

Output image:
![test_output](Images/graph)

The results are written to Results.csv.
The graph was plotted with gnuplot.


## Install
# Windows
Download the windows binary, [image_processor.exe](/image_processor.exe) from the repository and add to path using: https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/

# Linux
Install the rust toolchain at https://www.rust-lang.org/tools/install
Clone the directory using git then run ` cargo build --release `
Then add /target/release to path in your ~/.profile 


## How to use

run the following command in a terminal: `image_processor --image-file-name <IMAGE_FILE_NAME> --r-value <R_VALUE> --output-file-name <OUTPUT_FILE_NAME>`

Where R_VALUE is an integer between 0 and 255, the higher the numer the more pixels will be filtered out.
The results will be written to OUTPUT_FILE_NAME where the first column is the x corrdinates and the y column lists the y coordinates of the pixels.

For best results you want the image to be mostly dark with a very bright path of the red laser. Ideally you would crop the image such that the glass tank was the only thing that is visible, this will avoid noise and allow pixel coordinates to be converted into real distances, e.g for a tank that is 10 meters long, with 2000 pixels along the x axis, an x coordinate of 100 corrisponds to a physical distance of 100/200 * 10 meters if the image is cropped appropriatly and parralax is minnimal.
