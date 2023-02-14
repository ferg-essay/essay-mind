use symphonia::core::audio::{SampleBuffer, AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::AudioBuffer;

pub struct AudioReader {

}

impl AudioReader {
    pub fn read(path: &str) -> AudioBuffer {
        // Get the first command line argument.
        //let args: Vec<String> = std::env::args().collect();
        //let path = args.get(1).expect("file path not provided");

        //let path = "examples/blip.ogg";

        // Open the media source.
        let src = std::fs::File::open(path).expect("failed to open media");

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Create a probe hint using the file's extension. [Optional]
        let mut hint = Hint::new();
        //hint.with_extension("mp3");
        //hint.with_extension("ogg");

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        // Probe the media source.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        // Get the instantiated format reader.
        let mut format = probed.format;

        // Find the first audio track with a known (decodeable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks");

        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        // Store the track identifier, it will be used to filter packets.
        let track_id = track.id;

        let mut out_buffer = AudioBuffer::new(Vec::new());
        
        // The decode loop.
        loop {
            // Get the next packet from the media format.
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and it is not
                    // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                    // for chained OGG physical streams.
                    todo!();
                }
                Err(err) => {
                    // A unrecoverable error occured, halt decoding.
                    //panic!("{}", err);
                    print!("{:?}\n", err);

                    return out_buffer;
                }
            };

            // Consume any new metadata that has been read since the last packet.
            while !format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // Consume the decoded audio samples (see below).
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    
                    assert!(spec.rate == 44100, "need encoding to be 44100");

                    // let mut sample_buf = SampleBuffer::<f32>::new(duration, spec);

                    match audio_buf {
                        AudioBufferRef::F32(buf) => {
                            for src in buf.chan(0) {
                                out_buffer.push(*src);
                            }
                        },
                        AudioBufferRef::U8(_) => todo!(),
                        AudioBufferRef::U16(_) => todo!(),
                        AudioBufferRef::U24(_) => todo!(),
                        AudioBufferRef::U32(_) => todo!(),
                        AudioBufferRef::S8(_) => todo!(),
                        AudioBufferRef::S16(buf) => {
                            for src in buf.chan(0) {
                                out_buffer.push(*src as f32 / 0x8000 as f32);
                            }
                        },
                        AudioBufferRef::S24(_) => todo!(),
                        AudioBufferRef::S32(_) => todo!(),
                        AudioBufferRef::F64(_) => todo!(),
                    }

                    //sample_buf.copy_interleaved_ref(audio_buf);
                    // buffer.append(sample_buf.samples());
                    //let samples= sample_buf.samples();
                    //print!("buf {:?} {:?}", spec, sample_buf.len());
                }
                Err(Error::IoError(_)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    continue;
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    continue;
                }
                Err(err) => {
                    // An unrecoverable error occured, halt decoding.
                    panic!("{}", err);
                }
            }
        }

        assert!(out_buffer.len() > 0, "no data read");

        out_buffer
    }
}
