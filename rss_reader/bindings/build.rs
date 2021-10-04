fn main() {
    windows::build!(
        Windows::Foundation::Collections::IVector,
        Windows::Foundation::{IAsyncOperationWithProgress, Uri},

        Windows::Web::Syndication::{
            ISyndicationText, RetrievalProgress, SyndicationClient, SyndicationFeed, SyndicationItem,
        },

        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
    );
}