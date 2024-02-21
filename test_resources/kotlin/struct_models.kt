class Empty() {
    companion object {
        fun createDefault(): Empty = Empty()
    }
}

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

data class Test(
    val touchX: Float,
    val touchY: Float,
    val touchStatus: TouchStatus,
) {
    companion object {
        fun createDefault(): Test = Test(
            touchX = 0f,
            touchY = 0f,
            touchStatus = TouchStatus.createDefault(),
        )
    }
}

data class GenericType(
    val items: List<Test>,
    val table: LinearTable<Float, Test>,
) {
    companion object {
        fun createDefault(): GenericType = GenericType(
            items = listOf(),
            table = LinearTable.createDefault<Float, Test>(),
        )
    }
}
