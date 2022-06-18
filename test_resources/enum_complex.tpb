enum ComplexEnum {
    #[1] Idle = 1,
    #[2] Move {
        #[1] x: f64,
        #[2] y: f64,
    } = 2,
    #[3] Update(#[1] f64, #[2] f64, #[4] String) = 3,
}
