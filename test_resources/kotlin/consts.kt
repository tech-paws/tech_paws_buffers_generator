object Commands {
    const val DRAW_LINES: ULong = 131073UL
    const val DRAW_PATH: ULong = 131074UL
    const val DRAW_QUADS: ULong = 131075UL
    const val DRAW_CENTERED_QUADS: ULong = 131076UL
    const val DRAW_TEXTS: ULong = 131077UL
    const val SET_COLOR_PIPELINE: ULong = 131078UL
    const val SET_TEXTURE_PIPELINE: ULong = 131079UL
    const val DRAW_CIRCLES: ULong = 131080UL
    const val DRAW_HOLLOW_CIRCLES: ULong = 131081UL
}

object Addr {
    const val SOME_VALUE: String = "Hello World!"

    object Groups {
        const val MAIN: ULong = 0UL
        const val MAIN_RENDER: ULong = 1UL
        const val RPC: ULong = 2UL
        const val RPC_SYNC: ULong = 3UL
        const val RPC_READ: ULong = 4UL
    }

    const val DELTA_TIME: Double = 16.6
    const val FLAG: Boolean = true

    object CommandsBuffers {
        const val WIN1_MAIN_RENDER: ULong = 0UL
    }
}
