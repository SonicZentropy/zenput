use inputbot::{handle_input_events, KeybdKey::*};

fn main() {
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
	handle_input_events();
}
