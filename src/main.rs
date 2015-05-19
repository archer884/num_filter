extern crate rand;
use rand::distributions::{ IndependentSample, Range };
use rand::{ Rng };

fn main() {
    let series = emit_series(rand::weak_rng(), Range::new(1, 21))
        .take(10)
        .filter(|n| n & 1 == 0);

    for n in series {
        println!("{}", n);
    }
}

fn emit_series<'a, R: Rng + 'a>(mut rng: R, range: Range<u32>) -> Box<Iterator<Item=u32> + 'a> {
    Box::new((0..).map(move |_| range.ind_sample(&mut rng)))
}
