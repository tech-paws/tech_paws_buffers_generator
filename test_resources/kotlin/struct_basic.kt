data class ViewData(
    val deltaTime: Float,
    val viewWidth: Float,
    val viewHeight: Float,
    val touchStartX: Float,
    val touchStartY: Float,
    val lastTouchX: Float,
    val lastTouchY: Float,
    val touchX: Float,
    val touchY: Float,
) {
    companion object {
        fun createDefault(): ViewData = ViewData(
            deltaTime = 0f,
            viewWidth = 0f,
            viewHeight = 0f,
            touchStartX = 0f,
            touchStartY = 0f,
            lastTouchX = 0f,
            lastTouchY = 0f,
            touchX = 0f,
            touchY = 0f,
        )

        fun readFromBuffers(reader: Long): ViewData {
            val deltaTime = Float.readFromBuffers(reader)
            val viewWidth = Float.readFromBuffers(reader)
            val viewHeight = Float.readFromBuffers(reader)
            val touchStartX = Float.readFromBuffers(reader)
            val touchStartY = Float.readFromBuffers(reader)
            val lastTouchX = Float.readFromBuffers(reader)
            val lastTouchY = Float.readFromBuffers(reader)
            val touchX = Float.readFromBuffers(reader)
            val touchY = Float.readFromBuffers(reader)

            return ViewData(
                deltaTime = deltaTime,
                viewWidth = viewWidth,
                viewHeight = viewHeight,
                touchStartX = touchStartX,
                touchStartY = touchStartY,
                lastTouchX = lastTouchX,
                lastTouchY = lastTouchY,
                touchX = touchX,
                touchY = touchY,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
                Float.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        deltaTime.writeToBuffers(writer)
        viewWidth.writeToBuffers(writer)
        viewHeight.writeToBuffers(writer)
        touchStartX.writeToBuffers(writer)
        touchStartY.writeToBuffers(writer)
        lastTouchX.writeToBuffers(writer)
        lastTouchY.writeToBuffers(writer)
        touchX.writeToBuffers(writer)
        touchY.writeToBuffers(writer)
    }
}
