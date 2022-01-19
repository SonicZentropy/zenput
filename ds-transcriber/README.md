# DS-TRANSCRIBER

## _Need an Offline Speech To Text converter?_

Records your mic, and returns a `String` containing what was said.

## Features

- Begins transcription after a long enough pause has been detected
- Change the length of the pause required to begin transcription
- Shows you the audio levels of what is being recorded so that you can...
- Change the audio level of what you deem as silence in your environment

Ds-Transcriber is backed by the awesome [cpal](https://github.com/RustAudio/cpal) for streaming and [nnnoiseless](https://github.com/jneem/nnnoiseless) for audio cleanup.

## Setting Up

You need to obtain the `deepspeech-model` (tested with `0.9.x`) and the `native-client` for your system and add that folder to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables. See the [quick start](https://github.com/RustAudio/deepspeech-rs#quickstart) guide over at [deepspeech-rs](https://github.com/RustAudio/deepspeech-rs#quickstart).

## Usage

Add the crate to your `Cargo.toml`

```toml
[dependencies]
ds-transcriber = "0.1.3"
```

Create a configuration wherever you want to use it

```rust
    // the path where your model and native-client lie
    let model_dir_str = args().nth(1).expect("Please specify model dir");
    let mut ds_model = DeepSpeechModel::instantiate_from(model_dir_str);
    let model = ds_model.model();
    let mut config = ds_transcriber::transcriber::StreamSettings {
        //value used for pause detection, a pause is detected when the amplitude is less than this
        silence_level: 200,
        // takes a reference of the model we instantiated earlier
        model,
        // show the amplitude values on stdout (helps you to find your silence level)
        show_amplitudes: true,
        // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
        pause_length: 2.0,
    };
```

After getting config ready, all you need to do is pass it to the function:

```rust
    let i_said = ds_transcriber::transcriber::transcribe(&mut config).unwrap();
    println!("I said: {}", i_said);

    // Reuse the same configuration for another transcription
    let i_said = ds_transcriber::transcriber::transcribe(&mut config).unwrap();
    println!("I also said: {}", i_said);
```

## Contributions

Heck yeah! Pull requests are the greatest thing since sliced bread.

## License

MIT

**Free Software, Gotta love it!**
