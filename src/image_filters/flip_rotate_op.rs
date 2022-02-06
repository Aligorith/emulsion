/// Flip/Rotate operator
use crate::image_cache::image_loader::Orientation;

// Data Structure for operator - defines the operations stack
struct ImageFlipRotateOp {
	// Stack of operations to apply. Latest operation is at the end of the vec.
	// NOTE: The first orientation in the stack comes from the EXIF data, and cannot be edited
	// NOTE: We combine adjacent operations together 
	op_stack : Vec<Orientation>
};


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


// Internal API for Orientations stack
impl ImageFlipRotateOp {
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
		let last_is_flip = orientation_is_flip_op(self.op_stack.last());
		let op_is_flip = orientation_is_flip_op(op);
		
		return last_is_flip == op_is_flip;
	}
}


// Externally Usable API for this operation
impl ImageFlipRotateOp {
	// Constructor
	pub fn new(exif_orientation: Orientation) -> Self
	{
		Self {
			// Init stack with the orientation from EXIF
			vec!([ exif_orientation ])
		}
	}
	
	// Add a "rotate-left" operation to the stack
	pub fn rotate_left(&mut self)
	{
		
	}
	
	// Add a "rotate-right" operation to the stack
	pub fn rotate_right(&mut self)
	{
		
	}
	
	// Add a "flip-horizontal" operation to the stack
	pub fn flip_horizontal(&mut self)
	{
		
	}
	
	// Add a "flip-vertical" operation to the stack
	pub fn flip_vertical(&mut self)
	{
		
	}
}


// Implement standard ImageFilterOp behaviours for this filter
// These are used to actually apply the filter's effects
impl ImageFilterOp for ImageFlipRotateOp {
	/// Dimensions of image after applying the filter
	fn dimensions(&self) -> (u32, u32)
	{
		
	}
	
	/// 4x4 matrix transform matrix to apply to the image
	/// (viewport transform for display)
	fn transform_matrix(&self) -> Matrix4<f32>
	{
		
	}
	
	/// Apply filter to pixels
	fn apply_filter(&self, image::RgbaImage) -> image::RgbaImage
	{
		
	}
}
