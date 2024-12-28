data class ColorRGB(
    val r: Float,
    val g: Float,
    val b: Float,
) {
    companion object {
        fun createDefault(): ColorRGB = ColorRGB(
            r = 0f,
            g = 0f,
            b = 0f,
        )

        fun readFromBuffers(reader: Long): ColorRGB {
            val r = Float.readFromBuffers(reader)
            val g = Float.readFromBuffers(reader)
            val b = Float.readFromBuffers(reader)

            return ColorRGB(
                r = r,
                g = g,
                b = b,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        r.writeToBuffers(writer)
        g.writeToBuffers(writer)
        b.writeToBuffers(writer)
    }
}

data class ColorRGBA(
    val r: Float,
    val g: Float,
    val b: Float,
    val a: Float,
) {
    companion object {
        fun createDefault(): ColorRGBA = ColorRGBA(
            r = 0f,
            g = 0f,
            b = 0f,
            a = 0f,
        )

        fun readFromBuffers(reader: Long): ColorRGBA {
            val r = Float.readFromBuffers(reader)
            val g = Float.readFromBuffers(reader)
            val b = Float.readFromBuffers(reader)
            val a = Float.readFromBuffers(reader)

            return ColorRGBA(
                r = r,
                g = g,
                b = b,
                a = a,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        r.writeToBuffers(writer)
        g.writeToBuffers(writer)
        b.writeToBuffers(writer)
        a.writeToBuffers(writer)
    }
}
