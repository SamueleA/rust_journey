use std::io;

fn main() {
    println!("What do you want to convert ([C]elcius or [F]ahrenheit?");

    let mut source_unit;

    loop {
        source_unit = String::new();
        io::stdin()
            .read_line(&mut source_unit)
            .expect("Failed to read input");

        source_unit = source_unit.trim().to_lowercase();

        if source_unit == 'c'.to_string() {
            let source_value = get_source_value();
            let result = (source_value * 9.0 / 5.0) + 32.0;
            println!("{source_value}C converts to {result}F");
            break;
        } else if source_unit == 'f'.to_string() {
            let source_value = get_source_value();
            let result = (source_value - 32.0) / 9.0 * 5.0;
            println!("{source_value}F converts to {result}C");
            break;
        }

        println!("Unrecognized option. Enter [C]elsius or [F]ahrenheit");
        continue;
    }
}

fn get_source_value() -> f64 {
    println!("Enter the value to convert");
    loop {
        let mut source_value = String::new();

        io::stdin()
            .read_line(&mut source_value)
            .expect("Failed to read input");

        match source_value.trim().parse::<f64>() {
            Ok(n) => break n,
            Err(_) => {
                println!("Invalid value. Enter a number to convert.");
                continue;
            }
        }
    }
}
