/// Image Filters
///
/// Module containing a bunch of non-destructive operations applied to
/// each image for UI display / when copying image data / when saving.
///
/// Example operations include:
/// - Flipping / Rotation (i.e. `OrientationFilter`)
/// - Cropping (i.e. `CroppingFilter`)
/// - Grayscale / Channel Filter

// TODO:
// - Decouple colour-only filters from those that affect the size/orientation of the canvas
//   so that we don't have to implement a whole bunch of boilerplate

use gelatin::image;

//////////////////////////////////////////

/// "ImageFilterOp" Trait
/// Base trait used to define a filtering operation.
pub trait ImageFilterOp
{
	/// Dimensions of image after applying the filter
	fn dimensions(&self, w: u32, h: u32) -> (u32, u32);
	
	/// 4x4 matrix transform matrix to apply to the image
	/// (viewport transform for display)
	fn transform_matrix(&self) -> Matrix4<f32>;
	
	/// Apply filter to pixels
	fn apply_filter(&self, image: image::RgbaImage) -> image::RgbaImage;
}
