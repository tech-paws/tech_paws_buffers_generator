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
