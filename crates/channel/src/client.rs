/// Client channel callback, low level network channel event trigger on when client created.
pub trait ClientStub {
    fn client_on_channel_open();

    fn client_on_channel_open_failed();

    fn client_on_channel_closed();

    fn client_on_channel_msg_received();
}
