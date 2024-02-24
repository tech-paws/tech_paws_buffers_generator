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
    }
}
