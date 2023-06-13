#[cfg(test)]
mod test {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total = v1_iter.sum();
    }
}
