use linear_algebra_42::Matrix;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;

struct ProjectionArgs {
    fov_degrees: f32,
    ratio: f32,
    near: f32,
    far: f32,
    output_file: String,
}

impl Default for ProjectionArgs {
    fn default() -> Self {
        Self {
            fov_degrees: 45.,
            ratio: 16. / 9.,
            near: 0.1,
            far: 100.0,
            output_file: "proj".to_string(),
        }
    }
}

fn print_help() {
    println!("Projection Matrix Generator\n");
    println!("USAGE:");
    println!("    projection [OPTIONS]\n");
    println!("OPTIONS:");
    println!("    -h, --help              Show this help message");
    println!("    --fov <DEGREES>         Vertical field of view in degrees (default: 45.0)");
    println!("    --ratio <RATIO>         Aspect ratio width/height (default: 16:9)");
    println!("    --near <DISTANCE>       Near plane distance (default: 0.1)");
    println!("    --far <DISTANCE>        Far plane distance (default: 100.0)");
    println!("    --output <FILE>         Output file name (default: proj)\n");
    println!("EXAMPLES:");
    println!("    projection");
    println!("    projection --fov 45 --ratio 1.7778 --near 0.1 --far 1000");
    println!("    projection --fov 60 --ratio 1.333 --output matrix.txt\n");
    println!("OUTPUT:");
    println!("    The program generates a file containing the projection matrix");
    println!("    in column-major format, compatible with OpenGL.");
}

fn parse_args() -> ProjectionArgs {
    let args: Vec<String> = env::args().collect();
    let mut result = ProjectionArgs::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "--fov" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --fov requires a value");
                    process::exit(1);
                }
                match args[i + 1].parse::<f32>() {
                    Ok(value) => {
                        if value <= 0.0 || value >= 180.0 {
                            eprintln!("Error: FOV must be between 0 and 180 degrees");
                            process::exit(1);
                        }
                        result.fov_degrees = value;
                    }
                    Err(_) => {
                        eprintln!("Error: invalid value for --fov: {}", args[i + 1]);
                        process::exit(1);
                    }
                }
                i += 1;
            }
            "--ratio" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --ratio requires a value");
                    process::exit(1);
                }
                match args[i + 1].parse::<f32>() {
                    Ok(value) => {
                        if value <= 0.0 {
                            eprintln!("Error: aspect ratio must be positive");
                            process::exit(1);
                        }
                        result.ratio = value;
                    }
                    Err(_) => {
                        eprintln!("Error: invalid value for --ratio: {}", args[i + 1]);
                        process::exit(1);
                    }
                }
                i += 1;
            }
            "--near" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --near requires a value");
                    process::exit(1);
                }
                match args[i + 1].parse::<f32>() {
                    Ok(value) => {
                        if value <= 0.0 {
                            eprintln!("Error: near plane distance must be positive");
                            process::exit(1);
                        }
                        result.near = value;
                    }
                    Err(_) => {
                        eprintln!("Error: invalid value for --near: {}", args[i + 1]);
                        process::exit(1);
                    }
                }
                i += 1;
            }
            "--far" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --far requires a value");
                    process::exit(1);
                }
                match args[i + 1].parse::<f32>() {
                    Ok(value) => {
                        if value <= 0.0 {
                            eprintln!("Error: far plane distance must be positive");
                            process::exit(1);
                        }
                        result.far = value;
                    }
                    Err(_) => {
                        eprintln!("Error: invalid value for --far: {}", args[i + 1]);
                        process::exit(1);
                    }
                }
                i += 1;
            }
            "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --output requires a value");
                    process::exit(1);
                }
                result.output_file = args[i + 1].clone();
                i += 1;
            }
            arg => {
                eprintln!("Error: unknown argument: {arg}");
                eprintln!("Use --help to see available options");
                process::exit(1);
            }
        }
        i += 1;
    }

    if result.near >= result.far {
        eprintln!(
            "Error: near plane ({}) must be less than far plane ({})",
            result.near, result.far
        );
        process::exit(1);
    }

    result
}

fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let tan_half_fov = (fov / 2.0).tan();

    let x_scale = 1.0 / (ratio * tan_half_fov);
    let y_scale = 1.0 / tan_half_fov;
    let z_scale = far / (near - far);
    let z_translation = (near * far) / (near - far);

    Matrix::from([
        [x_scale, 0., 0., 0.],
        [0., y_scale, 0., 0.],
        [0., 0., z_scale, -1.],
        [0., 0., z_translation, 0.],
    ])
}

fn write_matrix_to_file(matrix: &Matrix<f32>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    for row in matrix.iter() {
        writeln!(
            file,
            "{}",
            row.iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
    }

    Ok(())
}
fn main() {
    let args = parse_args();

    let fov = args.fov_degrees.to_radians();
    let ratio = args.ratio;
    let near = args.near;
    let far = args.far;

    let proj_matrix = projection(fov, ratio, near, far);

    println!("Calculated projection matrix:\n{proj_matrix}");

    match write_matrix_to_file(&proj_matrix, &args.output_file) {
        Ok(()) => println!(
            "\nProjection matrix saved to file '{}' successfully!",
            args.output_file
        ),
        Err(e) => eprintln!("Error saving file: {e}"),
    }

    println!("\nParameters used:");
    println!("FOV: {fov:.4} radians ({:.2} degrees)", args.fov_degrees);
    println!("Aspect ratio: {ratio:.4}");
    println!("Near plane: {near:.4}");
    println!("Far plane: {far:.4}");
    println!("Output file: {}", args.output_file);
}
