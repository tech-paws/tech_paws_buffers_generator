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
    }
}
