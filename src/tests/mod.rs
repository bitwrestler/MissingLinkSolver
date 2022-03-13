#[cfg(test)]
mod tests {
    use crate::missing_link_solver::*;

    #[test]
    pub fn initbrd_expect_default()
    {
        let sample = initbrd();
        assert_eq!(3,sample.blank_x);
        assert_eq!(0,sample.blank_y);
    }
}