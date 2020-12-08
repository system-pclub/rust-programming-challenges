// To make the code compilable while keeping the functionality of two threads
// modify the image in parallel, raw pointer operation is needed, besides moving
// ImageView out of the closure.

use rayon;
use std::marker::PhantomData;

type Color = [f64; 3];

pub struct RawImage<'a>
{
    _pd: PhantomData<&'a mut Color>,
    data: *mut Color,
    width: u32,
    height: u32,
}

impl<'a> RawImage<'a>
{
    pub fn new(data: &'a mut [Color], width: u32, height: u32) -> Self
    {
        Self {
            _pd: PhantomData,
            data: data.as_mut_ptr(),
            width: width,
            height: height
        }
    }

    fn xy2index(&self, x: u32, y: u32) -> usize
    {
        (y * self.width + x) as usize
    }
}

pub struct RawImageView<'a>
{
    img: &'a RawImage<'a>,
    offset_x: u32,
    offset_y: u32,
    width: u32,
    height: u32,
}

unsafe impl Send for RawImageView<'_> {}

impl<'a> RawImageView<'a>
{
    pub fn new(img: &'a RawImage<'a>, x0: u32, y0: u32, width: u32, height: u32) -> Self
    {
        Self {
            img,
            offset_x: x0, offset_y: y0,
            width: width, height: height,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color)
    {
        let index = self.img.xy2index(x + self.offset_x, y + self.offset_y);
        //TODO! check bounds
        unsafe { self.img.data.add(index).write(color) };
    }
}

fn modify(img: &mut RawImageView)
{
    img.set_pixel(0, 0, [0.1, 0.2, 0.3]);
}

fn main()
{
    let mut pixels = vec![[0.0, 0.0, 0.0]; (20 * 10) as usize];
    let mut img = RawImage::new(&mut pixels, 20, 10);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    let mut v1 = RawImageView::new(&img, 0, 0, 10, 10);
    let mut v2 = RawImageView::new(&img, 10, 0, 10, 10);
    pool.scope(|s| {
        s.spawn(|_| {
            modify(&mut v1);
        });
        s.spawn(|_| {
            modify(&mut v2);
        });
    });
}
