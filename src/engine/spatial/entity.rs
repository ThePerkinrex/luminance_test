use luminance::context::GraphicsContext;
use luminance::pipeline::{Pipeline, TessGate};
use luminance::pixel::NormRGBA8UI;
use luminance::shader::program::ProgramInterface;
use luminance::tess::{Mode as TessMode, Tess, TessBuilder, TessSliceIndex as _};
use luminance::texture::{Dim2, Texture};

// TODO Add entity implementation