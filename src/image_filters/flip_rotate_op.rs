/// Flip/Rotate operator
use crate::image_cache::image_loader::Orientation;

use gelatin::cgmath::Matrix4;
use gelatin::image::imageops::{
	flip_horizontal_in_place, flip_vertical_in_place, rotate180_in_place, rotate270, rotate90,
};


// ============================================================

// Data Structure for operator - defines the operations stack
struct ImageFlipRotateOp {
	// Stack of operations to apply. Latest operation is at the end of the vec.
	// NOTE: The first orientation in the stack comes from the EXIF data, and cannot be edited
	// NOTE: We combine adjacent operations together 
	op_stack : Vec<Orientation>
	
	// Allow combining operations
	// TODO: Delete this variable and have it always on
	combine_ops: bool
};


// ============================================================

// Returns if a given Orientation is a "flip" operation
// We need to know this, as flip operations are not compatible with rotate operations
fn orientation_is_flip_op(orientation: Orientation) -> bool
{
	use Orientation::*;
	match self.orientation {
		Deg0HorFlip | Deg180HorFlip | Deg90VerFlip | Deg270VerFlip => true
		_ => false
	}
}

// ============================================================

// Internal API for Orientations stack
impl ImageFlipRotateOp {
	// Get "last_op" - i.e. the previous orientation-change operation on the stack
	pub fn last_op(&self) -> Orientation
	{
		assert!(self.op_stack.len() >= 1);
		self.op_stack.last();
	}
	
	// Check if the incoming orientation can be combined
	// with the latest one on the stack of operations.
	pub fn can_combine_with_last_op(&self, op: Orientation) -> bool
	{
		// First op comes from EXIF data, so cannot be combined with anything
		if (self.op_stack.len() == 1) {
			return false;
		}
		
		// If the incoming and latest op on the stack are different types, they can't be combined
		// * flip / flip = true
		// * flip / rot  = false
		// * rot  / flip = false
		// * rot  / rot  = true
		let last_is_flip = orientation_is_flip_op(self.last_op());
		let op_is_flip = orientation_is_flip_op(op);
		
		return last_is_flip == op_is_flip;
	}
	
	// -----------------------------------------
	
	// Add operation to stack
	// Note: `Orientation` type describes counter-clockwise rotations
	pub fn add_operation(&mut self, op: Orientation)
	{
		if self.combine_ops && self.can_combine_with_last_op(op) {
			// Combine with "last_op"
			let combined_op = op; // FIXME: get combined op, and use that here...
			
			assert!(self.op_stack.len() >= 1);
			
			self.op_stack.pop().unwrap();
			self.op_stack.push(combined_op);
		}
		else {
			// Just add to the stack...
			self.op_stack.push(op);
		}
	}
}

// ============================================================

// Externally Usable API for this operation
// Note: `Orientation` type describes counter-clockwise rotations
impl ImageFlipRotateOp {
	// Constructor
	pub fn new(exif_orientation: Orientation) -> Self
	{
		Self {
			// Init stack with the orientation from EXIF
			vec!([ exif_orientation ]),
			
			// combine_ops
			false
		}
	}
	
	// Add a "rotate-left" operation to the stack
	pub fn rotate_left(&mut self)
	{
		self.add_operation(Orientation::Deg90);
	}
	
	// Add a "rotate-right" operation to the stack
	pub fn rotate_right(&mut self)
	{
		self.add_operation(Orientation::Deg270);
	}
	
	// Add a "flip-horizontal" operation to the stack
	pub fn flip_horizontal(&mut self)
	{
		self.add_operation(Orientation::Deg0HorFlip);
	}
	
	// Add a "flip-vertical" operation to the stack
	pub fn flip_vertical(&mut self)
	{
		eprintln!("flip_vertical() not implemented! Deg0VerFlip missing!")
		//self.add_operation(Orientation::Deg0VerFlip);
		self.add_operation(Orientation::Deg180); // XXX?
	}
}

// ============================================================

// Implement standard ImageFilterOp behaviours for this filter
// These are used to actually apply the filter's effects
impl ImageFilterOp for ImageFlipRotateOp {
	/// Dimensions of image after applying the filter
	pub fn dimensions(&self, w: u32, h: u32) -> (u32, u32)
	{
		use Orientation::*;
		
		// Apply each operation in the stack to the image
		let mut result: (u32, u32) = (w, h);
		
		for op in self.op_stack {
			result = match self.orientation {
				Deg0 | Deg0HorFlip | Deg180 | Deg180HorFlip   => (result.0, result.1),
				Deg90 | Deg90VerFlip | Deg270 | Deg270VerFlip => (result.1, result.0),
			};
		}
		
		result
	}
	
	/// 4x4 matrix transform matrix to apply to the image
	/// (viewport transform for display)
	pub fn transform_matrix(&self) -> Matrix4<f32>
	{
		#[rustfmt::skip]
		let mut result = Matrix4::from_scale(1.0);
		
		for op in self.op_stack {
			let matrix_for_op = match op {
				Orientation::Deg0 => Matrix4::from_scale(1.0),
				Orientation::Deg0HorFlip => Matrix4::new(
					-1.0, 0.0, 0.0, 0.0,
					0.0, 1.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg180 => Matrix4::new(
					-1.0, 0.0, 0.0, 0.0,
					0.0, -1.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg180HorFlip => Matrix4::new(
					1.0, 0.0, 0.0, 0.0,
					0.0, -1.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg90 => Matrix4::new(
					0.0, -1.0, 0.0, 0.0,
					1.0, 0.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg90VerFlip => Matrix4::new(
					0.0, -1.0, 0.0, 0.0,
					-1.0, 0.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg270 => Matrix4::new(
					0.0, 1.0, 0.0, 0.0,
					-1.0, 0.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
				Orientation::Deg270VerFlip => Matrix4::new(
					0.0, 1.0, 0.0, 0.0,
					1.0, 0.0, 0.0, 0.0,
					0.0, 0.0, 1.0, 0.0,
					0.0, 0.0, 0.0, 1.0
				),
			};
			
			// Accumulate / combine transforms described by the matrices
			result = result * matrix_for_op;
		}
		result
	}
	
	/// Apply filter to pixels
	pub fn apply_filter(&self, image: image::RgbaImage) -> image::RgbaImage
	{
		let mut result: image::RgbaImage = image;
		
		// Apply each operation in the stack to the image...
		for op in self.op_stack {				
			result = match orientation {
				Orientation::Deg0 => result,
				Orientation::Deg0HorFlip => {
					flip_horizontal_in_place(&mut result);
					result
				}
				Orientation::Deg90 => rotate270(&result),
				Orientation::Deg90VerFlip => {
					let mut rotated_img = rotate270(&result);
					flip_vertical_in_place(&mut rotated_img);
					rotated_img
				}
				Orientation::Deg180 => {
					rotate180_in_place(&mut result);
					result
				}
				Orientation::Deg180HorFlip => {
					// This is identical to just a vertical flip with no rotation.
					flip_vertical_in_place(&mut result);
					result
				}
				Orientation::Deg270 => rotate90(&result),
				Orientation::Deg270VerFlip => {
					let mut rotated_img = rotate90(&result);
					flip_vertical_in_place(&mut rotated_img);
					rotated_img
				}
			};
		}
		
		// Return the resulting image
		return result;
	}
}
