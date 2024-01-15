use std::{thread, sync::{Arc, Mutex}, path::PathBuf};
use iced_futures::{Subscription, subscription};
use iced_futures::channel::mpsc;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    Start(PathBuf),
    Cancelled,
    Completed,
}

pub fn on_scan_start(file_path: std::path::PathBuf) -> Subscription<ScanEvent> {
	struct Scanner;
	let (sender, receiver) = mpsc::channel::<String>();
	let receiver: Arc<Mutex<mpsc::Receiver<String>>> = Arc::new(Mutex::new(receiver));
	let thread_receiver = Arc::clone(&receiver);

	// let (_, sender) = x::new(move |dir| {
	// 		stream.emit(ConfigMsg::GotResults(dir));
	// });
	// let (send, recv) = channel();
	// self.cancel_sender = Some(send);

	thread::spawn(move || {
			let dir = "foo"; //dir_walker::read_dir(&file_path, &recv);
			sender.send(dir.to_string()).expect("Couldn't send message");
	});
	// todo: fix didn't exit correctly
	println!("Scanning result: {}", result);
	subscription::channel(std::any::TypeId::of::<Scanner>(), 100, |mut output| async move {

	loop {
				match &mut state {
						State::Scanning => {
							let result = thread_receiver.await;
							// .lock().unwrap().recv().unwrap();



						}
						State::Finished(websocket, input) => {

						}
				}
		}
	})
}
// fn on_scan_complete(&mut self, dir: Arc<Mutex<dir_walker::Directory>>) {
// 	self.cancel_sender = None;
// 	let dir_clone = dir.clone();
// 	let error = dir.lock().unwrap().get_error().clone();
// 	match error {
// 			None => {
// 					self.window.hide();
// 					let analyzer_win = init::<analyzer::AnalyzerWindow>(dir_clone).expect("Couldn't init");
// 					analyzer_win.widget().show_all();
// 					self.analyzer_win = Some(analyzer_win);
// 			},
// 			Some(e) => match e {
// 					dir_walker::ReadError::IOError(_) => {
// 							let msg = "Could not read directory contents";
// 							let message_box = gtk::MessageDialog::new(Some(&self.window), gtk::DialogFlags::MODAL, gtk::MessageType::Error,
// 																												gtk::ButtonsType::Ok, &msg);
// 							message_box.run();
// 							message_box.hide();
// 							self.reset_ui();
// 					},
// 					dir_walker::ReadError::OperationCancelled => self.reset_ui()
// 			}
// 	}
// }
// fn on_scan_cancel(&self) {
// 	self.cancel_button.set_sensitive(false);
// 	if let Some(tracker) = &self.cancel_sender {
// 			tracker.send(()).unwrap();
// 	}
// }
