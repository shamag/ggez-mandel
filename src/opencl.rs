extern crate ocl;
use ocl::ProQue;
use ocl::{SpatialDims, Device, Context, Platform, Program};

pub fn generate(width: usize, height: usize, limit: usize) -> ocl::Result<Vec<u64>> {
    let src = r#"
       // Used to index into the 1D array, so that we can use
// it effectively as a 2D array
int index(int x, int y, int width) {
  return width*y + x;
}

// Turn the raw x coordinates [0, 1] into a scaled x coordinate
// [0, 1] -> [-2, 1.25]
float mapX(float x) {
  return x*3.25 - 2;
}

// Same purpose as mapX
// [0, 1] -> [-1.25, 1.25]
float mapY(float y) {
  return y*2.5 - 1.25;
}

__kernel void render(__global size_t *out, int limit) {
  int x_dim = get_global_id(0);
  int y_dim = get_global_id(1);
  size_t width = get_global_size(0);
  size_t height = get_global_size(1);
  int idx = index(x_dim, y_dim, width);

  float x_origin = mapX((float) x_dim / width);
  float y_origin = mapY((float) y_dim / height);

  // The Escape time algorithm, it follows the pseduocode from Wikipedia
  // _very_ closely
  float x = 0.0;
  float y = 0.0;

  int iteration = 0;

  // This can be changed, to be more or less precise
  int max_iteration = limit;
  while(x*x + y*y <= 4 && iteration < max_iteration) {
    float xtemp = x*x - y*y + x_origin;
    y = 2*x*y + y_origin;
    x = xtemp;
    iteration++;
  }

  if(iteration == max_iteration) {
    // This coordinate did not escape, so it is in the Mandelbrot set
    out[idx] = 255;
  } else {
    // This coordinate did escape, so color based on quickly it escaped
    out[idx] = iteration;

  }

}
    "#;
    let platform = Platform::default();

//    println!("Choosing device...");
//    println!("{}",platform.name().unwrap());

    let device = Device::first(platform)?;
//    println!("{}",device.name().unwrap());
//
//    println!("Creating context...");



    let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build().unwrap();

    let program = Program::builder()
        .devices(device)
        .src(src)
        .build(&context).unwrap();

    let pro_que = ProQue::builder()
        .device(device)
        .src(src)
        .dims(width*height)
        .build()?;

    let buffer = pro_que.create_buffer::<u64>()?;

    let mut kernel = pro_que.kernel_builder("render")
        .arg(&buffer)
        .arg(limit as i32)
        .build()?;

    kernel.set_default_global_work_size(SpatialDims::Two(width,height));

    unsafe { kernel.enq()?; }

    let mut vec = vec![0u64; buffer.len()];
    buffer.read(&mut vec).enq()?;

    let max = vec.iter().max();
    Ok(vec)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(generate(300, 300, 100).expect("error"), vec![1,1]);
    }
}