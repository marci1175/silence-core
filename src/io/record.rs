//! Offers audio recording capabilities via being a middleware on [`cpal`].

use std::{collections::VecDeque, sync::Arc, thread::{sleep, JoinHandle}, time::Duration};

use cpal::{
    traits::{DeviceTrait, StreamTrait},
    StreamConfig, StreamError,
};
use parking_lot::Mutex;

use super::InputDevice;

///
/// Records audio until the user interrupts it.
///
/// # Behavior
/// The recording thread is automaticly started at the creation of the (Input)[`cpal::Stream`].
/// Records audio until (Pushes the samples into the [`Arc<Mutex<Vec<f32>>>`]) the [`tokio::sync::oneshot::Receiver`] receives a message.
/// The [`Sync`] buffer is returned and can be accessed.
///
/// # Error
/// The `err_callback` callback is called if an error occurs in the [`DeviceTrait::build_input_stream`] whilst recording.
/// The function returns an error if an error occured outside of the [`DeviceTrait::build_input_stream`] function.
pub fn record_audio_with_interrupt<E>(
    input_device: InputDevice,
    interrupt: tokio::sync::oneshot::Receiver<()>,
    err_callback: E,
    config: StreamConfig,
) -> anyhow::Result<Arc<Mutex<VecDeque<f32>>>>
where
    E: FnMut(StreamError) + Send + 'static,
{
    let buffer_handle: Arc<parking_lot::lock_api::Mutex<parking_lot::RawMutex, VecDeque<f32>>> =
        Arc::new(parking_lot::Mutex::new(VecDeque::new()));
    let buffer_handle_clone = buffer_handle.clone();

    let _: JoinHandle<anyhow::Result<()>> = std::thread::spawn(move || {
        let stream: cpal::Stream = input_device.build_input_stream(
            &config,
            move |data: &[f32], _info: &cpal::InputCallbackInfo| {
                let mut buffer_handle = buffer_handle_clone.lock();
                for sample in data.iter() {
                    buffer_handle.push_back(*sample);
                }
            },
            err_callback,
            None,
        )?;

        //Start stream
        stream.play()?;

        //Wait for interrupt
        interrupt.blocking_recv()?;

        //Return from thread
        Ok(())
    });

    Ok(buffer_handle)
}

///
/// Records audio until the user interrupts it.
///
/// # Behavior
/// The recording thread is automaticly started at the creation of the (Input)[`cpal::Stream`].
/// Records audio until (Pushes the samples into the [`Arc<Mutex<Vec<f32>>>`]) the [`tokio::sync::oneshot::Receiver`] receives a message.
/// The [`Sync`] buffer is returned and can be accessed.
///
/// # Error
/// The `err_callback` callback is called if an error occurs in the [`DeviceTrait::build_input_stream`] whilst recording.
/// The function returns an error if an error occured outside of the [`DeviceTrait::build_input_stream`] function.
pub fn record_audio_with_duration<E>(
    input_device: InputDevice,
    duration: Duration,
    err_callback: E,
    config: StreamConfig,
) -> anyhow::Result<Arc<Mutex<Vec<f32>>>>
where
    E: FnMut(StreamError) + Send + 'static,
{
    let buffer_handle: Arc<parking_lot::lock_api::Mutex<parking_lot::RawMutex, Vec<f32>>> =
        Arc::new(parking_lot::Mutex::new(Vec::new()));
    let buffer_handle_clone = buffer_handle.clone();

    let _: JoinHandle<anyhow::Result<()>> = std::thread::spawn(move || {
        let stream: cpal::Stream = input_device.build_input_stream(
            &config,
            move |data: &[f32], _info: &cpal::InputCallbackInfo| {
                let mut buffer_handle = buffer_handle_clone.lock();
                for sample in data.iter() {
                    buffer_handle.push(*sample);
                }
            },
            err_callback,
            None,
        )?;

        //Start stream
        stream.play()?;

        //Sleep the thread
        sleep(duration);

        //Return from thread
        Ok(())
    });

    Ok(buffer_handle)
}
