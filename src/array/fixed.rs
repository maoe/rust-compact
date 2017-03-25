use std::mem;
use std::marker::PhantomData;

use num::*;
use typenum;

type Word = u32;

#[derive(Debug)]
pub struct Array<T, L> {
    storage: Vec<Word>,
    elem_type: PhantomData<T>,
    elem_size: PhantomData<L>,
}

impl<T, L> Array<T, L>
    where T: Unsigned,
          L: typenum::Unsigned
{
    pub fn get(&self, i: usize) -> T
        where T: From<Word>
    {
        let l = L::to_usize();
        let j = i * l;
        let k = (i + 1) * l - 1;
        let w = mem::size_of::<Word>() * 8; // FIXME

        if j / w == k / w {
            let word: Word = (self.storage[k / w] >> (j % w)) & ((1 << (k - j + 1)) - 1);
            word.into()
        } else {
            let word: Word = (self.storage[j / w] >> (j % w)) |
                             (self.storage[k / w] & ((1 << ((k + 1) % w)) - 1)) << (w - (j % w));
            word.into()
        }
    }
    pub fn set(&mut self, i: usize, x: T)
        where T: Into<Word>
    {
        set::<T, L>(&mut self.storage, i, x);
    }
}

impl<T, L> From<Vec<T>> for Array<T, L>
    where T: Into<Word>,
          L: typenum::Unsigned
{
    fn from(vec: Vec<T>) -> Array<T, L> {
        let len = vec.len();
        let l = L::to_usize(); // element size in bits
        let w = mem::size_of::<Word>() * 8; // word size in bits
        let capacity = div_round_up(l * len, w); // capacity of internal storage
        let mut storage: Vec<Word> = vec![0; capacity];

        for (i, x) in vec.into_iter().enumerate() {
            set::<T, L>(&mut storage, i, x);
        }

        Array {
            storage: storage,
            elem_size: PhantomData,
            elem_type: PhantomData,
        }
    }
}

fn set<T, L>(storage: &mut Vec<Word>, i: usize, x: T)
    where T: Into<Word>,
          L: typenum::Unsigned
{
    let l = L::to_usize();
    let j = i * l;
    let k = (i + 1) * l - 1;
    let w = mem::size_of::<Word>() * 8; // FIXME
    let tee = x.into();
    if j / w == k / w {
        let word: Word = !(((1 << (k - j + 1)) - 1) << (j % w));
        storage[j / w] &= word.into();
        storage[j / w] |= tee << (j % w);
    } else {
        storage[j / w] = (storage[j / w] & ((1 << (j % w)) - 1)) | (tee << (j % w));
        storage[k / w] = (storage[k / w] & !((1 << ((k + 1) % w)) - 1)) | (tee >> (w - (j % w)));
    }
}

fn div_round_up<T: Unsigned + PrimInt>(x: T, y: T) -> T {
    if x > zero() {
        one::<T>() + (x - one()) / y
    } else {
        zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", div_round_up(1u32, 0));
    }
}
