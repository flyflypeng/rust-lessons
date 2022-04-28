#[derive(Debug)]
enum Genders {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Genders,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("message {:?}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("get message: {:?}", msg);
    } else {
        println!("get message nothing");
    }
}

fn main() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Genders::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Genders::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    process_event(&event1);
    process_event(&event2);
    process_event(&event3);

    process_message(&event1);
    process_message(&event3);
}
