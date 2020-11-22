#include "media-rust-pipe/mediapipe-bind.h"

namespace mediapipe {

int width(const ImageFrame &image) {

    return image.Width();
}

}
