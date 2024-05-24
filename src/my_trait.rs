struct Circom;
struct Noir;

trait IVC {
    fn compute(&self);
}

impl IVC for Circom {
    fn compute(&self) {
        println!("compute circom");
    }
}

impl IVC for Noir {
    fn compute(&self) {
        println!("compute noir");
    }
}

trait FoldingCircuit<T: IVC> {
    fn get_ivc(&self) -> &T;
    fn step_native(&self) {
        self.get_ivc().compute();
    }

    fn constraint(&self) {
        self.get_ivc().compute();
    }
}

impl<T: IVC> FoldingCircuit<T> for Circuit<T> {
    fn get_ivc(&self) -> &T {
        &self.ivc
    }
}

struct CustomIVC {}

impl IVC for CustomIVC {
    fn compute(&self) {
        println!("custom IVC");
    }
}

struct Circuit<T: IVC> {
    ivc: T,
}

struct CustomDefaultCircuit<T: IVC> {
    ivc: T,
}

impl<T: IVC> FoldingCircuit<T> for CustomDefaultCircuit<T> {
    fn get_ivc(&self) -> &T {
        &self.ivc        
    }

    fn step_native(&self) {
        println!("This is custom circom circuit");
    }
}

#[test]
fn tests() {
    // Circom and noir is default IVC
    let a = Circuit { ivc: Circom {} };

    let b = Circuit { ivc: Noir {} };
    let c = Circuit { ivc: CustomIVC {} };

    a.step_native();
    b.step_native();
    c.step_native();
    // compute circom
    // compute noir
    // custom IVC

    let d = CustomDefaultCircuit {ivc: Circom {}};
    d.step_native();
    
}
