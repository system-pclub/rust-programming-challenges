use std::iter::Cycle;

type IntegerCycle = Cycle<std::vec::IntoIter<i32>>;

fn generate_cycles() -> [IntegerCycle; 2] {
  let cycles = [
    [1, 2],
    [2, 4],
  ];
  cycles
    .map(|points| {
      points.map(|point| point*2)
    })
    .map(|points| {
      points.to_vec().into_iter().cycle()
    })
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cycles() {
        let mut cycles = generate_cycles();
        let mut results = vec![];
        for cycle in cycles.iter_mut() {
            let mut cycle_results = vec![];
            for _ in 0..10 {
                cycle_results.push(cycle.next().unwrap());
            }
            results.push(cycle_results);
        }
        assert_eq!(results, vec![
            vec![2, 4, 2, 4, 2, 4, 2, 4, 2, 4],
            vec![4, 8, 4, 8, 4, 8, 4, 8, 4, 8],
        ]);
    }
}