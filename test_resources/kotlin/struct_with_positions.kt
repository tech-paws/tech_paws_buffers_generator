data class Test(
    val touchY: Float,
    val touchX: Float,
    val touchStatus: TouchStatus,
) {
    companion object {
        fun createDefault(): Test = Test(
            touchY = 0f,
            touchX = 0f,
            touchStatus = TouchStatus.createDefault(),
        )

        fun readFromBuffers(reader: Long): Test {
            val touchY = Float.readFromBuffers(reader)
            val touchX = Float.readFromBuffers(reader)
            val touchStatus = TouchStatus.readFromBuffers(reader)

            return Test(
                touchY = touchY,
                touchX = touchX,
                touchStatus = touchStatus,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                TouchStatus.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        touchY.writeToBuffers(writer)
        touchX.writeToBuffers(writer)
        touchStatus.writeToBuffers(writer)
    }
}
