extern crate rand;
use rand::distributions::{ IndependentSample, Range };
use rand::{ Rand };

fn main() {
    let range = Range::new(1u32, 21u32);
    let series = random_series(|| range.ind_sample(&mut rand::thread_rng()))
        .take(10)
        .filter(|n| n & 1 == 0);

    for n in series {
        println!("{}", n);
    }
}

fn random_series<'f, T: Rand, F: FnMut() -> T + 'f>(mut f: F) -> Box<Iterator<Item=T> + 'f> {
    Box::new((0..).map(move |_| f()))
}
