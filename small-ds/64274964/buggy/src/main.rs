type Color = [f64; 3];

pub struct RawImage
{
    data: Vec<Color>,
    width: u32,
    height: u32,
}

impl RawImage
{
    pub fn new(width: u32, height: u32) -> Self
    {
        Self {
            data: vec![[0.0, 0.0, 0.0]; (width * height) as usize],
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
    img: &'a mut RawImage,
    offset_x: u32,
    offset_y: u32,
    width: u32,
    height: u32,
}

impl<'a> RawImageView<'a>
{
    pub fn new(img: &'a mut RawImage, x0: u32, y0: u32, width: u32, height: u32) -> Self
    {
        Self{ img: img,
              offset_x: x0, offset_y: y0,
              width: width, height: height, }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color)
    {
        let index = self.img.xy2index(x + self.offset_x, y + self.offset_y);
        self.img.data[index] = color;
    }
}

fn modify(img: &mut RawImageView)
{
    // Do some heavy calculation and write to the image.
    img.set_pixel(0, 0, [0.1, 0.2, 0.3]);
}

fn main()
{
    let mut img = RawImage::new(20, 10);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    pool.scope(|s| {
        let mut v1 = RawImageView::new(&mut img, 0, 0, 10, 10);
        let mut v2 = RawImageView::new(&mut img, 10, 0, 10, 10);
        s.spawn(|_| {
            modify(&mut v1);
        });
        s.spawn(|_| {
            modify(&mut v2);
        });
    });
}