use super::Observation;

pub fn calculate_score(observations: Vec<Observation>) -> Option<[f64;2]> {
    let [div, mean] = std_deviation_and_mean(&observations)?;
    
    Some([mean + div, mean - div])
}
fn mean(data: &[Observation]) -> Option<f64> {
    let sum = data.iter().sum::<Observation>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation_and_mean(data: &[Observation]) -> Option<[f64; 2]> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 2 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some([variance.sqrt(), data_mean])
        },
        _ => None
    }
}