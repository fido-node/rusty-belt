pub fn vec_to_chart(vec: &Vec<u64>) -> String {
    let max = vec.iter().max().unwrap_or(&1);
    let percents = vec
        .iter()
        .map(|v| ((*v as f64) / (*max as f64)) * 100.0)
        .collect();

    percent_vec_to_chart(&percents)
}

pub fn percent_vec_to_chart(vec: &Vec<f64>) -> String {
    let chars = vec![' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let chars_len = chars.len();
    let step = 100.0 / (chars_len as f64);
    vec.iter()
        .map(|v| {
            let mut batch = ((v / step).ceil() as i64) - 1;
            if batch == -1 {
                batch += 1;
            }
            chars.get(batch as usize).map(|c| *c).unwrap_or(chars[0])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_to_chart_empty() {
        let result = vec_to_chart(&vec![]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_vec_to_chart_single_value() {
        let result = vec_to_chart(&vec![42]);
        assert_eq!(result, "█");
    }

    #[test]
    fn test_vec_to_chart_all_zeros() {
        let result = vec_to_chart(&vec![0, 0, 0, 0]);
        assert_eq!(result, "    ");
    }

    #[test]
    fn test_vec_to_chart_normal_case() {
        let result = vec_to_chart(&vec![10, 20, 30, 40, 50]);
        assert_eq!(result, "▁▃▅▇█");
    }

    #[test]
    fn test_percent_vec_to_chart_empty() {
        let result = percent_vec_to_chart(&vec![]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_percent_vec_to_chart_single_value() {
        let result = percent_vec_to_chart(&vec![42.0]);
        assert_eq!(result, "▃");
    }

    #[test]
    fn test_percent_vec_to_chart_all_zeros() {
        let result = percent_vec_to_chart(&vec![0.0, 0.0, 0.0, 0.0]);
        assert_eq!(result, "    ");
    }

    #[test]
    fn test_percent_vec_to_chart_normal_case() {
        let result = percent_vec_to_chart(&vec![10.0, 20.0, 30.0, 40.0, 50.0]);
        assert_eq!(result, " ▁▂▃▄");
    }
}
