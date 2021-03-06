use num_traits::{Float, One, NumAssignOps};

// Lets fake a 2D array (square shape matrix) with a 1D vector:
struct Matrix<T> {
    values: Vec<T>,
    dim: usize,
}

// ... and implement getter and setter methods using 'unsafe' functions :
impl<T: PartialEq + Clone> Matrix<T> {
    pub fn new(init_value: T, dim: usize) -> Matrix<T> {
        Matrix {
            values: vec![init_value; dim * dim],
            dim: dim,
        }
    }

    #[inline(always)]
    pub fn get(&self, ix: (usize, usize)) -> &T {
        unsafe { self.values.get_unchecked(ix.0 * self.dim + ix.1) }
    }

    #[inline(always)]
    pub fn set(&mut self, ix: (usize, usize), value: T) {
        let v = unsafe { self.values.get_unchecked_mut(ix.0 * self.dim + ix.1) };
        *v = value;
    }
}

/// Compute the "Natural Breaks" on a list of sorted values, based on Jenks optimization.
pub fn get_jenks_breaks<T>(sorted_values: &[T], nb_class: u32) -> Vec<T>
    where T: Float + NumAssignOps
{
    let k: usize = nb_class as usize;
    let nb_elem: usize = sorted_values.len();
    let mut v1 = Matrix::new(1, nb_elem);
    let mut v2 = Matrix::new(Float::max_value(), nb_elem);

    let (mut v, mut val, mut s1, mut s2, mut w, mut i3, mut i4): (T, T, T, T, T, usize, usize);

    for l in 2..(nb_elem + 1) {
        s1 = T::zero();
        s2 = T::zero();
        w = T::zero();
        for m in 1..(l + 1) {
            i3 = l - m + 1;
            val = unsafe { *sorted_values.get_unchecked(i3 - 1) };
            s2 += val * val;
            s1 += val;
            w += One::one();
            v = s2 - (s1 * s1) / w;
            i4 = i3 - 1;
            if i4 != 0 {
                for j in 2..k + 1 {
                    let _v = v + *v2.get((i4 - 1, j - 2));
                    if *v2.get((l - 1, j - 1)) >= _v {
                        v2.set((l - 1, j - 1), _v);
                        v1.set((l - 1, j - 1), i3);
                    }
                }
            }
            v1.set((l - 1, 0), 1);
            v2.set((l - 1, 0), v);
        }
    }
    let mut kclass = vec![0usize; k];
    let mut n = nb_elem;
    let mut j = k;
    while j > 1 {
        n = *v1.get(((n - 1), (j - 1))) - 1usize;
        kclass[(j - 2)] = n;
        j -= 1;
    }
    let mut breaks = Vec::with_capacity(k);
    breaks.push(sorted_values[0]);
    for i in 1..k {
        breaks.push(sorted_values[(kclass[i - 1usize] - 1)]);
    }
    breaks.push(sorted_values[(nb_elem - 1)]);
    breaks
}
