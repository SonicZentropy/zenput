mod get_window_title;

use env_logger::Target;
use ds_transcriber::model::DeepSpeechModel;
use inputbot::{handle_input_events, KeybdKey::*};
use winapi::{
	um::{
		winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible, IsWindowEnabled},
		winnt::LPWSTR
	},
	shared::{minwindef::{BOOL, LPARAM}, windef::HWND},
};
use winapi::um::winuser::GetForegroundWindow;
use log::{info, warn, trace };

unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
	if IsWindowVisible(window) == 0 { return true.into() }

	//Unfocused window
	if window != GetForegroundWindow() {
		return true.into();
	}
	trace!("FOUND THE FOREGROUND WINDOW TITLE");

	let state = state as *mut Vec<String>;
	let mut length = GetWindowTextLengthW(window);
	if length == 0 { return true.into() }
	length = length + 1;
	let mut title: Vec<u16> = vec![0; length as usize];
	let textw = GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length);
	if textw != 0 {
		if let Ok(title) = String::from_utf16(title[0..(textw as usize)].as_ref()) {
			(*state).push(title);
		} else {
			warn!("Couldn't get title");
		}
	}
	true.into()
}

pub fn window_titles() -> Vec<String> {
	let state: Box<Vec<String>> = Box::new(Vec::new());
	let ptr = Box::into_raw(state);
	let state;
	unsafe {
		EnumWindows(Some(enumerate_windows), ptr as LPARAM);
		state = Box::from_raw(ptr);
	}
	info!("Found window - {}", (*state).join(""));
	*state

}

fn main() {
	let mut builder = env_logger::Builder::from_default_env();
	builder.target(env_logger::Target::Stderr);
	builder.init();


	window_titles();
	return;
	// Autorun for videogames.
	//NumLockKey.bind(|| {
	//    while NumLockKey.is_toggled() {
	//        LShiftKey.press();
	//        WKey.press();
	//        sleep(Duration::from_millis(50));
	//        WKey.release();
	//        LShiftKey.release();
	//    }
	//});
	//
	//// Rapidfire for videogames.
	//RightButton.bind(|| {
	//    while RightButton.is_pressed() {
	//        LeftButton.press();
	//        sleep(Duration::from_millis(50));
	//        LeftButton.release();
	//    }
	//});

	// Send a key sequence.ap
	//RKey.bind(|| KeySequence("Sample text").send());

	// Move mouse.
	//QKey.bind(|| MouseCursor::move_rel(10, 10));

	// Create a handler to trigger on any and all keyboard events.
	//inputbot::KeybdKey::bind_all(|evnt| {
	//    println!("{:?}", evnt);
	//});

	let model_dir_str = "X:/Dev/deepspeech/models".to_owned();
	let mut ds_model = DeepSpeechModel::instantiate_from(model_dir_str);
	let model = ds_model.model();

	let mut config = ds_transcriber::transcriber::StreamSettings {
		//value used for pause detection, a pause is detected when the amplitude is less than this
		silence_level: 200,
		// takes a reference of the model we instantiated earlier
		model,
		// show the amplitude values on stdout (helps you to find your silence level)
		show_amplitudes: true,
		// seconds of silence indicating end of speech (begin transcription when pause_length is greater than....)
		pause_length: 1.0,
	};

	println!("Before transcribe");

	let i_said = ds_transcriber::transcriber::transcribe(&mut config).unwrap();
	println!("After transcribe");
	println!("I said: {}", i_said);

	AKey.bind(true, || {
		AKey.forward_to(PKey);
	});

	SKey.bind(false, || {
		if SKey.is_pressed() {
			ZKey.press();
		} else {
			ZKey.release();
		}
	});


	//DKey.bind(|| {
	//	while DKey.is_pressed() {
	//		QKey.press();
	//		sleep(Duration::from_millis(50));
	//		QKey.release();
	//	}
	//});

	//AKey.bind(|| {
	//    while AKey.is_pressed() {
	//        PKey.press(false);
	//        sleep(Duration::from_millis(50));
	//        PKey.release();
	//    }
	//});
	//
	//
	//
	//DKey.bind(|| {
	//    while DKey.is_pressed() {
	//        QKey.press(true);
	//        sleep(Duration::from_millis(50));
	//        QKey.release();
	//    }
	//});

	//SKey.blockable_bind(|| -> BlockInput {
	//    while SKey.is_pressed() {
	//        PKey.press();
	//        sleep(Duration::from_millis(50));
	//        PKey.release();
	//
	//    }
	//    return BlockInput::DontBlock;
	//});

	//NKey.bind(false, || {
	//    while NKey.is_pressed() {
	//        LControlKey.press();
	//        sleep(Duration::from_millis(50));
	//        LControlKey.release();
	//    }
	//}, );

	// Call this to start listening for bound inputs.
	//handle_input_events();
}
