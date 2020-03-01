extern crate ocl;
use ocl::ProQue;
use ocl::{SpatialDims, Device, Context, Platform, Program, Buffer};
use crate::renderer::{MandelbrotRenderer};
use std::error::Error;

pub struct OCLMandelbrot{
    queue: ProQue,
    buffer: Buffer<u64>,
    dims: (usize, usize)
}
impl MandelbrotRenderer for OCLMandelbrot {
    fn new(dims: (usize, usize)) -> OCLMandelbrot {
        let platform = Platform::default();
        let device = Device::first(platform).unwrap();
        let src = r#"
         #pragma OPENCL EXTENSION cl_khr_fp64 : enable
        int index(int x, int y, int width) {
          return width*y + x;
        }
        #pragma OPENCL EXTENSION cl_khr_fp64 : enable
        __kernel void render(__global size_t *out, float x_s, float x_e, float y_s, float y_e,  int limit) {
          int x_dim = get_global_id(0);
          int y_dim = get_global_id(1);
          size_t width = get_global_size(0);
          size_t height = get_global_size(1);
          int idx = index(x_dim, y_dim, width);

          float dx = (x_e - x_s) / width;
          float dy = (y_e - y_s) / height;

          float x_origin = x_s + dx * x_dim;
          float y_origin = y_s + dy * y_dim;

          float x = 0.0;
          float y = 0.0;

          int iteration = 0;

          int max_iteration = limit;
          while(x*x + y*y <= 4 && iteration < max_iteration) {
            float xtemp = x*x - y*y + x_origin;
            y = 2*x*y + y_origin;
            x = xtemp;
            iteration++;
          }

          if(iteration == max_iteration) {
            out[idx] = max_iteration;
          } else {
            out[idx] = iteration;
          }
        }
    "#;


        let context = Context::builder()
            .platform(platform)
            .devices(device.clone())
            .build().unwrap();

        let program = Program::builder()
            .devices(device)
            .src(src)
            .build(&context).unwrap();

        let pro_que = ProQue::builder()
            .platform(platform)
            .device(device)
            .src(src)
            .dims(dims.0*dims.1)
            .build().unwrap();
        dbg!(pro_que.device());
        let buffer = pro_que.create_buffer::<u64>().unwrap();
        OCLMandelbrot{
            queue: pro_que,
            dims,
            buffer
        }
    }
    fn render(&self, xr: std::ops::Range<f64>, yr: std::ops::Range<f64>, limit: usize) ->Result<Vec<u64>, Box<Error>> {
        //println!("xr=({},{}), yr=({},{}), limit={}", xr.start, xr.end, yr.start, yr.end, limit);
        let mut kernel = self.queue.kernel_builder("render")
            .arg(&self.buffer)
            .arg(xr.start as f32)
            .arg(xr.end as f32)
            .arg(yr.start as f32)
            .arg(yr.end as f32)
            .arg(limit as i32)
            .build().expect("cant render");

        kernel.set_default_global_work_size(SpatialDims::Two(self.dims.0,self.dims.1));

        unsafe { kernel.enq().expect("cant render"); }

        let mut vec = vec![0u64; self.buffer.len()];
        self.buffer.read(&mut vec).enq().expect("cant render");

        Ok(vec)
    }
}


//
//impl OCLMandelbrot{
//
//
//}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        let renderer= OCLMandelbrot::new((300, 300));
        let dims = (2000, 2000);
        let xr = std::ops::Range{start: -1., end: -2.};
        let yr = std::ops::Range{start: -1., end: -2.};
        assert_eq!(renderer.generate(dims, xr, yr, 100).expect("error"), vec![1,1]);
    }
}