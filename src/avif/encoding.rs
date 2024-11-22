//! Provides AV1 encoding for higher data efficiency via [`ravif`].

use image::GenericImageView;
use ravif::{Encoder, Img, EncodedImage};

/// 
/// Encodes a **formatted** image with the AV1 format.
/// 
/// **If you are looking to encode a [`crate::cam::Webcam::get_frame`]'s returned frame [`encode_raw_image`] is the one to use.**
/// 
/// # Behavior
/// The **formatted** image is re-encoded with the settings the user passes in with the [`Encoder`] argument.
/// The encoded image ([`ravif::EncodedImage`]) is returned from the image.
/// 
/// # Error
/// Will return an error if the image format could not be guessed correctly or if the image had incorrect proerties (Eg.: Invalid size).
/// 
pub fn encode_image(encoder: Encoder, image: &[u8]) -> anyhow::Result<EncodedImage> {
    //Parse image from bytes
    let parsed_image = image::load_from_memory(image)?;

    //Iter over the pixels
    let colors = parsed_image.pixels().map(|(_, _, color)| {
        ravif::RGB8::new(color.0[0], color.0[1], color.0[2])
    }).collect::<Vec<ravif::RGB8>>();

    //Encode image
    let encoded_image: EncodedImage = encoder.encode_rgb(Img::new(colors.as_slice(), parsed_image.width() as usize, parsed_image.height() as usize))?;
    
    Ok(encoded_image)
}

/// 
/// Encodes a raw image with the AV1 format
/// 
/// # Behavior
/// The image is encoded with the settings the user passes in with the [`Encoder`] argument.
/// The encoded image ([`EncodedImage`]) is returned from the raw image.
/// 
/// # Error
/// If the image had incorrect proerties (Eg.: Invalid size).
/// 
pub fn encode_raw_image(encoder: Encoder, image: &[u8], width: usize, height: usize) -> anyhow::Result<EncodedImage> {
    //Iter over the raw pixels
    let colors = image.chunks_exact(3).map(|chunk| {
        ravif::RGB8::new(chunk[0], chunk[1], chunk[2])
    }).collect::<Vec<ravif::RGB8>>();

    //Encode image
    let encoded_image: EncodedImage = encoder.encode_rgb(Img::new(colors.as_slice(), width, height))?;
    
    Ok(encoded_image)
}
