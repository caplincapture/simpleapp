use std::{time::Duration, thread::sleep};

fn main() {
    let mut client = MyClient { ticks_left: 4 };

    let mut app = App {
        title: "Jack in the box".to_string(),
        running: true,
        client: &mut client,
    };

    app.run();
}

struct App<'a> {
    title: String,
    running: bool,
    client: &'a mut dyn Client,
}

impl App<'_> {
    fn run(&mut self) {
        println!("=== You are now playing {} ===", self.title);

        loop {
            let app = self as *mut _;
            self.client.update(app);
            self.client.render();

            if !self.running {
                break;
            }
            sleep(Duration::from_secs(1));
        }
    }
}

trait Client {
    fn update(&mut self, app: *mut App);
    fn render(&self);
}

struct MyClient {
    ticks_left: usize,
}

impl Client for MyClient {
    fn update(&mut self, app: *mut App) {
        self.ticks_left -= 1;
        if self.ticks_left == 0 {
            unsafe {
                (*app).running = false;
            }
        }
    }

    fn render(&self) {
        if self.ticks_left > 0 {
            println!("You turn the crank...");
        } else {
            println!("Jack POPS OUT OF THE BOX");
        }
    }
}