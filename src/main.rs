extern crate rand;
use rand::distributions::{ IndependentSample, Range };
use rand::{ Rand, Rng };

fn main() {
    let mut rng = rand::weak_rng();
    let range = Range::new(1u32, 21u32);
    let series = random_series(|| { range.ind_sample(&mut rng) })
        .take(10)
        .filter(|n| n & 1 == 0);

    for n in series {
        println!("{}", n);
    }
}

fn random_series<'f, T: Rand, F: Fn() -> T + 'f>(f: F) -> Box<Iterator<Item=T> + 'f> {
    Box::new((0..).map(move |_| f()))
}
