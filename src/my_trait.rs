struct Circom;
struct Noir;

trait IVCFunction {
    fn step_native(&self);
    fn constraint(&self) {
        println!("This is constraint by default");
    }
}

impl IVCFunction for Circom {
    fn step_native(&self) {
        println!("compute circom");
    }
}

impl IVCFunction for Noir {
    fn step_native(&self) {
        println!("compute noir");
    }
}

trait FCircuit<T: IVCFunction> {
    fn functional(&self) -> &T;
    fn step_native(&self) {
        self.functional().step_native();
    }

    fn constraint(&self) {
        self.functional().constraint();
    }
}

impl<T: IVCFunction> FCircuit<T> for Circuit<T> {
    fn functional(&self) -> &T {
        &self.func
    }
}

struct CustomIVC {}

impl IVCFunction for CustomIVC {
    fn step_native(&self) {
        println!("custom IVC");
    }
}

struct Circuit<T: IVCFunction> {
    func: T,
}

struct CustomDefaultCircuit<T: IVCFunction> {
    func: T,
}

impl<T: IVCFunction> FCircuit<T> for CustomDefaultCircuit<T> {
    fn functional(&self) -> &T {
        &self.func        
    }

    fn step_native(&self) {
        println!("This is custom circuit in rust");
    }
}

#[test]
fn tests() {
    // Circom and noir is default IVC
    let a = Circuit { func: Circom {} };

    let b = Circuit { func: Noir {} };
    let c = Circuit { func: CustomIVC {} };

    a.step_native();
    b.step_native();
    c.step_native();
    // compute circom
    // compute noir
    // custom IVC

    let d = CustomDefaultCircuit {func: Circom {}};
    d.step_native(); 
    // this is custom circuit

}
