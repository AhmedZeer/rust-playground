use crossbeam::scope;
use image::ExtendedColorType;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use num::Complex;
use std::env;
use std::fs::File;
use std::str::FromStr;

fn main() {
    println!("Welcome to the Mandelbrot Set Plotter.");
    // .args() returns an iterator that can be
    // *collected* and returned as a whole collectin,
    // in this case a string vector.
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage : {} FILENAME.png IMG_SIZE UPPERLEFT UPPERRIGHT",
            args[0]
        );
        eprintln!(
            "Example : {} mandle.png 1920x1080 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let (width, height) = parse_pair::<usize>(&args[2], 'x').expect("Cant parse image size.");
    let upperleft = parse_complex(&args[3]).expect("Cant parse upperleft.");
    let lowerright = parse_complex(&args[4]).expect("Cant parse upperright.");
    let mut buf = vec![0; width * height];

    let threads = 8;
    let rows_per_band = height / threads + 1; // Columns processed by each thread.

    // To borrow from the buffer for writing and later
    // for reading we need to define a scope for threads
    // to operate on. Once this scope ends, the bands
    // are automatically released.
    {
        // Borrow mutable instances of `buf` to write
        // results. It will be freed after the scope.
        let bands: Vec<&mut [u8]> = buf.chunks_mut(rows_per_band * width).collect();

        // We initialize each thread inside this scope.
        // It ensures joining all the threads. If any
        // thread fails, the whole scope return an error.
        scope(|spawner| {
            // No threads until now.
            for (i, band) in bands.into_iter().enumerate() {
                // Current tile's starting index.
                let start = i * rows_per_band;
                // Each tile's height.
                let band_height = band.len() / width;
                let band_upperleft =
                    pixel_to_point((width, height), (upperleft, lowerright), (0, start));
                let band_lowerright = pixel_to_point(
                    (width, height),
                    (upperleft, lowerright),
                    (width, band_height + start),
                );
                // Spawn the thread with `move` keyword to
                // basically move the ownership of each object
                // inside the closure to the closure itself, i.e band.
                spawner.spawn(move |_| {
                    render(
                        band,
                        (width, band_height),
                        (band_upperleft, band_lowerright),
                    );
                });
            }
        })
        .unwrap()
    }

    // Single Threaded Approach.
    // render(&mut buf, (width, height), (upperleft, upperright));
    write_image(&mut buf, &args[1], (width, height)).expect("Couldnt write the image.");
}

fn write_image(
    pixels: &mut [u8],
    filename: &str,
    img_size: (usize, usize),
) -> Result<(), std::io::Error> {
    // Since creating a file can fail for
    // whatever reason the method itself
    // returns a `Result`. Which may be
    // Ok() or Err(). The `?` handles the
    // logic. Still,`?` syntactic sugar cant
    // be used in the `main` function because
    // it does not return anything. In this case,
    // handling the error must be done manually
    // using `match` statement.
    let output = File::create(filename)?;

    let encoder = PngEncoder::new(output);
    encoder
        .write_image(
            pixels,
            img_size.0 as u32,
            img_size.1 as u32,
            ExtendedColorType::L8,
        )
        .expect("Couldnt write image.");

    Ok(())
}

// Complex Number -> (real, imaginary)
// @param img_size: (x, y) -> (row, col)
// @param complex_size: (upper_left, lower_right)
fn pixel_to_point(
    img_size: (usize, usize),
    complex_size: (Complex<f64>, Complex<f64>),
    pixel: (usize, usize),
) -> Complex<f64> {
    let resolution = (
        // Right side's real number is bigger
        // than that of Left.
        complex_size.1.re - complex_size.0.re,
        // Upper side's imaginary number is
        // bigger than that of Lowre's.
        complex_size.0.im - complex_size.1.im,
    );

    Complex {
        re: complex_size.0.re + (pixel.0 as f64 * resolution.0 / img_size.0 as f64),
        im: complex_size.0.im - (pixel.1 as f64 * resolution.1 / img_size.1 as f64),
    }
}

fn render(pixels: &mut [u8], img_size: (usize, usize), complex_size: (Complex<f64>, Complex<f64>)) {
    assert!(img_size.0 * img_size.1 == pixels.len());

    for row in 0..img_size.1 {
        for col in 0..img_size.0 {
            let idx = row * img_size.0 + col;
            let c = pixel_to_point(img_size, complex_size, (col, row));
            pixels[idx] = match escape_time(c, 255) {
                None => 0,
                Some(val) => 255 - val as u8,
            };
        }
    }
}

// Here we exhibit the power of the `match`
// statement in Rust : **Tuple comparison**
// Unlike C switch case, where we are only able
// to compare a single value, in Rust we are able
// to compare tuples and derive a logic from there.

// If you wonder where `index` is defined, it is
// automatically bound from the previous statement.
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
        None => None,
    }
}

// Here we can notice that the constructor of
// Complex number allowed us to initialize the
// object with out sepcifying attribute names.
// The reason is basic : Rust automatically binds
// attributes with the **same** name.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

// Ideally, to calculate whether a number
// is in the mandelbrot set we iterate
// infinitely. There is a specific statement
// which allow us to loop for infinity : `loop`
// However, we instead obtain an approximation.

// We return an `Option<dtype>` which indicates
// that we either return a value in the expected
// dtype, Some(i) or None.
fn escape_time(c: Complex<f64>, limit: u64) -> Option<u64> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    // let i: u64 = 0;

    // while i < limit && z.norm_sqr() < 4.0 {
    //     z = z * z + c;
    // }
    //
    // if (i < limit) {
    //     return Some(i);
    // }

    for i in 0..limit {
        // Normally we would take the square_root
        // and find out the result. However, the
        // square operation is much more faster.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

#[test]
fn test_pair() {
    assert_eq!(parse_pair::<i32>("51x21", 'x'), Some((51, 21)));
    assert_eq!(parse_pair::<i32>("x21", 'x'), None);
    assert_eq!(parse_pair::<i32>("x", 'x'), None);

    assert_eq!(
        parse_pair::<f32>("21.534,432.2", ','),
        Some((21.534, 432.2))
    );
    assert_eq!(parse_pair::<f32>("21.534x432.2", ','), None);
}

#[test]
fn test_complex_pair() {
    assert_eq!(
        parse_complex("21.0,10.1"),
        Some(Complex { re: 21.0, im: 10.1 })
    );

    assert_eq!(parse_complex(",10.1"), None);
}
