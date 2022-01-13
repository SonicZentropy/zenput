use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};

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
        println!("In AKey.Bind()");
        if AKey.is_pressed() {
            println!("Pressing PKey because A was pressedqq");
            PKey.press(true);
        } else {
            //dbg!("Releasing PKey because A was released");
            PKey.release(true);
        }
    });

    SKey.bind(false,  || {
        println!("In SKey.Bind()");
        if SKey.is_pressed() {
            println!("NOT Pressing PKey because S was pressedqq");
            PKey.press(false);
        } else {
            //dbg!("Releasing PKey because A was released");
            PKey.release(false);
        }
    });



    DKey.bind(false, || {
        while DKey.is_pressed() {
            QKey.press(false);
            sleep(Duration::from_millis(50));
            QKey.release(false);
        }
    });

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
