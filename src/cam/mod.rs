//! Offers the ability to receive camera input.

use anyhow::bail;
use opencv::{
    core::{Mat, MatTraitConst, MatTraitConstManual, Size_},
    videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY},
};

//Reinport important functions
pub use opencv::imgproc::{cvt_color_def, COLOR_BGR2RGB};

/// Webcam struct definition.
/// The struct wraps the ```VideoCapture``` type, and has custom functions for it.
/// You can create a new instance with the ```new``` functions.
#[derive(Debug)]
pub struct Webcam(VideoCapture);

impl Webcam {
    ///
    /// Create new ```Webcam``` instance with api preference and camera index.
    /// 
    /// **If you want to use the default api_preference you should use ```new_def(i32)``` instead**
    /// 
    /// # Behavior
    /// Creates a new webcam instance with a set index and an [api_preference](https://docs.rs/opencv/0.93.4/opencv/videoio/enum.VideoCaptureAPIs.html).
    /// 
    /// # Error
    /// Returns an error if the inpt device could not be found based on the camera_idx, or if the api preference was invalid.
    /// 
    /// # Information
    /// API preference consts are available at the [opencv documentation](https://docs.rs/opencv/latest/opencv/index.html). Some exmaples for this const are: ```CAP_MSMF```, ```CAP_V4L```.
    /// 
    pub fn new(camera_idx: i32, api_preference: i32) -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new(camera_idx, api_preference)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }

        Ok(Self(video_capture_handle))
    }

    /// 
    /// Create new ```Webcam``` instance with automatic camera detection.
    /// 
    /// **If you have more than one camera you should use the [`Self::new_def`] function to define which camera you are wanting to use.**
    /// 
    /// # Behavior
    /// Creates a new [`Webcam`] instance while automaticly detecting the camera input.
    ///
    /// # Error
    /// This returns an error if it could not find the input device.
    /// 
    pub fn new_def_auto_detect() -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new_def(CAP_ANY)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }

        Ok(Self(video_capture_handle))
    }

    ///
    /// Create new ```Webcam``` instance with api preference and camera index.
    /// 
    /// # Behavior
    /// Creates a new webcam instance with a set index and the default ```api_preference``` ([`CAP_ANY`]).
    /// 
    /// # Error
    /// Returns an error if the inpt device could not be found based on the camera_idx.
    pub fn new_def(camera_idx: i32) -> anyhow::Result<Self> {
        let video_capture_handle = VideoCapture::new_def(camera_idx)?;

        if !video_capture_handle.is_opened()? {
            bail!("Failed to open capture device.")
        }

        Ok(Self(video_capture_handle))
    }

    /// 
    /// Requests a frame from the [`Webcam`] instance. 
    /// 
    /// # Behavior
    /// Reads an image out of the ```VideoCapture``` buffer, this removes the bytes of the image from the buffer.
    /// Returns a tuple of the raw image bytes and the size of the image.
    /// 
    /// # Information
    /// Please note the image's bytes returned by this function are automaticly converted from [BRG8](https://learn.microsoft.com/en-us/windows/win32/wic/-wic-codec-native-pixel-formats#rgbbgr-color-model) (Which is returned by opencv by default) to RGB8
    ///
    /// # Error
    /// Returns an error if it:
    ///     * failed to read from the webcam.
    ///     * the color format conversion failed.
    ///     * there was some kind of error when getting the image bytes / the size of the image from the [`Mat`].
    ///     * the Webcam instance was invalid. (It got released before requesting this frame)
    /// 
    pub fn get_frame(&mut self) -> anyhow::Result<(Vec<u8>, Size_<i32>)> {
        //Create frame which will be overwritten
        let mut frame = Mat::default();

        //Read frame
        self.0.read(&mut frame)?;

        //Create corrected_frame
        let mut corrected_frame = Mat::default();

        //Color correction
        cvt_color_def(&frame, &mut corrected_frame, COLOR_BGR2RGB)?;

        //Return captured frame
        Ok((
            corrected_frame.data_bytes()?.to_vec(),
            corrected_frame.size()?,
        ))
    }

    /// 
    /// Get the backend api's name.
    ///
    /// # Behavior
    /// Gets the backend api's name.
    /// 
    /// # Error
    /// Returns an error if it failed to get the backend api's name.
    /// 
    pub fn get_backend_name(&self) -> anyhow::Result<String> {
        Ok(self.0.get_backend_name()?)
    }

    ///
    /// This function drops the inner ```VideoCapture``` instance.
    ///
    /// # Behavior
    /// This function releases the [`Webcam`]'s underlying [`VideoCapture`] instance.
    /// 
    /// # Information
    /// The underlying [`VideoCapture`] instance is invalidated, thus requesting frames on this [`Webcam`] instance will be unsuccessful.
    ///
    /// # Error
    /// Returns an error if it could not invalidate the instance.
    /// 
    pub fn release(&mut self) -> anyhow::Result<()> {
        Ok(self.0.release()?)
    }
}