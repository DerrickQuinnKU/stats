use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();


    let numbers = args[2..]
        .iter()
        .map(|arg| arg.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>();

    let numbers = match numbers {
        Ok(v) => v,
        Err(_) => return Err("Error when parsing arguments (specify operation FIRST)".to_string()),
    };

    if args.contains(&"--average".to_string()) {
        let average = calculate_average(&numbers)?;
        println!("{average}");
        Ok(())
    } else if args.contains(&"--median".to_string()) {
        let median = calculate_median(&numbers)?;
        println!("{median}");
        Ok(())
    } else if args.contains(&"--variance".to_string()) {
        let variance = calculate_variance(&numbers)?;
        println!("{variance}");
        Ok(())
    } else if args.contains(&"--stdev".to_string()) {
        let stdev = calculate_stdev(&numbers)?;
        println!("{stdev}");
        Ok(())
    } else if args.contains(&"--error95".to_string()) {
        let error95 = calculate_error95(&numbers)?;
        println!("{error95}");
        Ok(())
    } else if args.contains(&"--error90".to_string()) {
        let error90 = calculate_error90(&numbers)?;
        println!("{error90}");
        Ok(())
    } else {
        Err(String::from("Please specify one of the following: --average --median --variance --stdev --error90 --error95"))
    }
}

fn calculate_average(numbers: &[f64]) -> Result<f64, String> {
    if numbers.is_empty() {
        return Err(String::from(
            "Please provide at least 1 number for average",
        ));
    }
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    Ok(sum / count)
}

fn calculate_median(numbers: &[f64]) -> Result<f64, String> {
    if numbers.is_empty() {
        return Err(String::from(
            "Please provide at least 1 number for median",
        ));
    }
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}

fn calculate_variance(numbers: &[f64]) -> Result<f64, String> {
    if numbers.len() < 2 {
        return Err(String::from(
            "Please provide at least 2 numbers for variance",
        ));
    }
    let average = calculate_average(numbers)?;
    let squared_diff: f64 = numbers.iter().map(|&num| (num - average).powi(2)).sum();
    Ok(squared_diff / ((numbers.len() - 1) as f64))
}

fn calculate_stdev(numbers: &[f64]) -> Result<f64, String> {
    if numbers.len() < 2 {
        return Err(String::from(
            "Please provide at least 2 numbers for stdev",
        ));
    }
    Ok(calculate_variance(numbers)?.sqrt())
}

fn calculate_error95(numbers: &[f64]) -> Result<f64, String> {
    if numbers.len() < 2 {
        return Err(String::from(
            "Please provide at least 2 numbers for error95",
        ));
    }
    let sample_stdev = calculate_stdev(numbers)?;
    let mean_stdev = sample_stdev / (numbers.len() as f64).sqrt(); //Central limit theorem
    Ok(mean_stdev * 1.95996398454) //z-score of 1.96 for MoE for 95% CI
}

fn calculate_error90(numbers: &[f64]) -> Result<f64, String> {
    if numbers.len() < 2 {
        return Err(String::from(
            "Please provide at least 2 numbers for error90",
        ));
    }
    let sample_stdev = calculate_stdev(numbers)?;
    let mean_stdev = sample_stdev / (numbers.len() as f64).sqrt(); //Central limit theorem
    Ok(mean_stdev * 1.64485362695) //z-score of 1.64 for MoE for 90% CI
}
