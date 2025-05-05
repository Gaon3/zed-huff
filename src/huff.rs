use zed_extension_api as zed;

struct HuffExtension;

impl zed::Extension for HuffExtension {
    fn new() -> Self {
        HuffExtension
    }
}

zed::register_extension!(HuffExtension);
