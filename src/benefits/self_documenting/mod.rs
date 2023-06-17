mod old {
    struct Message {}

    // what does &str represent, is it the phone number ? email ?
    async fn send_message(message: &Message, recipient: &str) {}
}

mod new {
    struct Message {}

    struct WhatsAppNumber(String);

    async fn send_message(message: &Message, recipient: WhatsAppNumber) {}
}
